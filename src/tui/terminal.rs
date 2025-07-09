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
        Operation::Add { alias } => {
            if let Some(eq_pos) = alias.find('=') {
                let alias_name = alias[..eq_pos].trim();
                let command = alias[eq_pos + 1..].trim();
                add_alias::add_alias(database, deleted_commands, &alias_file_path, alias_name, command);
                app.status_message = format!("Added alias: {} = {}", alias_name, command);
                // Save after adding alias
                if let Err(e) = save_database(database, db_path) {
                    eprintln!("Failed to save database: {}", e);
                }
                if let Err(e) = save_deleted_commands(deleted_commands, deleted_commands_path) {
                    eprintln!("Failed to save deleted commands: {}", e);
                }
            } else {
                app.status_message = "Invalid alias format".to_string();
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
        Operation::Change { alias } => {
            if let Some(eq_pos) = alias.find('=') {
                let alias_name = alias[..eq_pos].trim();
                let command = alias[eq_pos + 1..].trim();
                remove_alias::remove_alias(deleted_commands, &alias_file_path, alias_name);
                add_alias::add_alias(database, deleted_commands, &alias_file_path, alias_name, command);
                app.status_message = format!("Changed alias: {} = {}", alias_name, command);
                // Save after changing alias
                if let Err(e) = save_database(database, db_path) {
                    eprintln!("Failed to save database: {}", e);
                }
                if let Err(e) = save_deleted_commands(deleted_commands, deleted_commands_path) {
                    eprintln!("Failed to save deleted commands: {}", e);
                }
            } else {
                app.status_message = "Invalid alias format".to_string();
            }
        }
        Operation::List => {
            let aliases = alias_ops::get_aliases_list(&alias_file_path);
            let mut message = "Current aliases:\n".to_string();
            for (alias, command) in aliases {
                message.push_str(&format!("{} = {}\n", alias, command));
            }
            app.show_popup(message);
        }
        Operation::GetSuggestions { num } => {
            let suggestions = get_suggestions::get_suggestions(num, database);
            let mut message = "Top suggestions:\n".to_string();
            for (i, cmd) in suggestions.iter().enumerate() {
                message.push_str(&format!(
                    "{}. {} (score: {})\n",
                    i + 1,
                    cmd.command_text,
                    cmd.score
                ));
            }
            app.show_popup(message);
            app.load_commands(database);
        }
        Operation::DeleteSuggestion { alias } => {
            delete_suggestion::delete_suggestion(&alias, database, deleted_commands);
            app.status_message = format!("Deleted suggestion: {}", alias);
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
    }
}
