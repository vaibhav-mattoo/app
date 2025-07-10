use crate::cli::cli_data::Operation;
use crate::database::database_structs::{Database, DeletedCommands};
use crate::database::persistence::{
    ensure_data_directory, get_database_path, get_deleted_commands_path, load_database,
    load_deleted_commands, save_database, save_deleted_commands,
};
use crate::ops::{alias_ops, add_alias , remove_alias, delete_suggestion, get_suggestions};
use crate::tui::app::App;
use crate::tui::ui::render_ui;
use ratatui::crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
};
use std::io;
use std::path::PathBuf;


pub fn run_tui(alias_file_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // Ensure data directory exists
    if let Err(e) = ensure_data_directory() {
        eprintln!("Failed to create data directory: {}", e);
        return Err(e);
    }

    // Load database and deleted commands from persistent storage
    let db_path = get_database_path();
    let deleted_commands_path = get_deleted_commands_path();

    let mut database = match load_database(&db_path) {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Failed to load database: {}", e);
            return Err(e);
        }
    };

    let mut deleted_commands = match load_deleted_commands(&deleted_commands_path) {
        Ok(dc) => dc,
        Err(e) => {
            eprintln!("Failed to load deleted commands: {}", e);
            return Err(e);
        }
    };

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    // Use a struct to ensure cleanup happens
    struct TerminalGuard {
        terminal: Terminal<CrosstermBackend<io::Stdout>>,
    }

    impl Drop for TerminalGuard {
        fn drop(&mut self) {
            // Always try to restore terminal state, even if it fails
            let _ = disable_raw_mode();
            let _ = execute!(
                self.terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            );
            let _ = self.terminal.show_cursor();
        }
    }

    let mut terminal_guard = TerminalGuard { terminal };

    // Create app and load initial data
    let mut app = App::new(alias_file_path.clone());
    app.load_commands(&mut database);

    // Run the app
    let res = run_app(
        &mut terminal_guard.terminal,
        &mut app,
        &mut database,
        &mut deleted_commands,
        &db_path,
        &deleted_commands_path,
    );

    if let Err(err) = res {
        eprintln!("TUI error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    database: &mut Database,
    deleted_commands: &mut DeletedCommands,
    db_path: &str,
    deleted_commands_path: &str,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| render_ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                if let Some(operation) = app.handle_key_event(key.code) {
                    handle_operation(operation, app, database, deleted_commands, db_path, deleted_commands_path);
                }
            }
        }

        if app.should_quit {
            break;
        }
    }
    Ok(())
}

fn handle_operation(
    operation: Operation,
    app: &mut App,
    database: &mut Database,
    deleted_commands: &mut DeletedCommands,
    db_path: &str,
    deleted_commands_path: &str,
) {
    let alias_file_path = app.alias_file_path.to_string_lossy().to_string();

    match operation {
        Operation::Add { alias, command } => {
            add_alias::add_alias(database, deleted_commands, &alias_file_path, &alias, &command);
            app.status_message = format!("Added alias: {} = {}", alias, command);
            // Save after adding alias
            if let Err(e) = save_database(database, db_path) {
                eprintln!("Failed to save database: {}", e);
            }
            if let Err(e) = save_deleted_commands(deleted_commands, deleted_commands_path) {
                eprintln!("Failed to save deleted commands: {}", e);
            }
        }
        Operation::Remove { alias } => {
            remove_alias::remove_alias(deleted_commands, &alias_file_path, &alias);
            app.status_message = format!("Removed alias: {}", alias);
            // Save after removing alias
            if let Err(e) = save_deleted_commands(deleted_commands, deleted_commands_path) {
                eprintln!("Failed to save deleted commands: {}", e);
            }
        }
        Operation::Change { old_alias, new_alias } => {
            // First remove the old alias
            remove_alias::remove_alias(deleted_commands, &alias_file_path, &old_alias);
            // Then add the new alias with the same command
            // Note: This assumes we want to keep the same command, just change the alias name
            // If we need to change both alias and command, we'd need additional logic
            let aliases = alias_ops::get_aliases_list(&alias_file_path);
            if let Some((_, command)) = aliases.iter().find(|(alias, _)| alias == &old_alias) {
                add_alias::add_alias(database, deleted_commands, &alias_file_path, &new_alias, command);
                app.status_message = format!("Changed alias: {} -> {}", old_alias, new_alias);
                // Save after changing alias
                if let Err(e) = save_database(database, db_path) {
                    eprintln!("Failed to save database: {}", e);
                }
                if let Err(e) = save_deleted_commands(deleted_commands, deleted_commands_path) {
                    eprintln!("Failed to save deleted commands: {}", e);
                }
            } else {
                app.status_message = format!("Alias '{}' not found", old_alias);
            }
        }
        Operation::List => {
            let aliases = alias_ops::get_aliases_list(&alias_file_path);
            if aliases.is_empty() {
                app.status_message = "No aliases found.".to_string();
                return;
            }
            
            // Find the longest alias for alignment
            let max_alias_length = aliases.iter()
                .map(|(alias, _)| alias.len())
                .max()
                .unwrap_or(0);
            
            let mut message = "┌─ ALIASES ──────────────────────────────────────────────────────────────┐\n".to_string();
            message.push_str(&format!("│ {:<width$} │ COMMAND\n", "ALIAS", width = max_alias_length));
            message.push_str(&format!("├─{}─┼─{}─┤\n", "─".repeat(max_alias_length + 2), "─".repeat(60)));
            
            for (alias, command) in &aliases {
                let formatted_alias = format!("{:<width$}", alias, width = max_alias_length);
                message.push_str(&format!("│ {} │ {}\n", formatted_alias, command));
            }
            
            message.push_str(&format!("└─{}─┴─{}─┘\n", "─".repeat(max_alias_length + 2), "─".repeat(60)));
            message.push_str(&format!("Total: {} alias(es)", aliases.len()));
            
            app.show_popup(message);
        }
        Operation::GetSuggestions { num } => {
            let suggestions = get_suggestions::get_suggestions_with_aliases(num, database, &alias_file_path);
            let mut message = "Top suggestions:\n".to_string();
            for (i, cmd_with_alias) in suggestions.iter().enumerate() {
                message.push_str(&format!(
                    "{}. {} (score: {})",
                    i + 1,
                    cmd_with_alias.command.command_text,
                    cmd_with_alias.command.score
                ));
                
                if !cmd_with_alias.alias_suggestions.is_empty() {
                    message.push_str(" -> aliases: ");
                    let alias_strings: Vec<String> = cmd_with_alias.alias_suggestions.iter()
                        .map(|a| format!("{}={} ({})", a.alias, a.command, a.reason))
                        .collect();
                    message.push_str(&alias_strings.join(", "));
                }
                message.push('\n');
            }
            app.show_popup(message);
            app.load_commands(database);
        }
        Operation::DeleteSuggestion { alias } => {
            delete_suggestion::delete_suggestion(&alias, database, deleted_commands);
            app.status_message = format!("Deleted suggestions for: {}", alias);
            app.load_commands(database);
            // Save after deleting suggestion
            if let Err(e) = save_database(database, db_path) {
                eprintln!("Failed to save database: {}", e);
            }
            if let Err(e) = save_deleted_commands(deleted_commands, deleted_commands_path) {
                eprintln!("Failed to save deleted commands: {}", e);
            }
        }
        Operation::Tui => {
            // Already in TUI mode, do nothing
        }
        Operation::Init { .. } => {
            // Init is not available in TUI mode
            app.status_message = "Init command not available in TUI mode".to_string();
        }
    }
}
