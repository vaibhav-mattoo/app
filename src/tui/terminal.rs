use crate::cli::cli_data::Operation;
use crate::database::database_structs::{Database, Deleted_Commands};
use crate::ops::{alias_ops, delete_suggestion, get_suggestions};
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
use std::collections::{BTreeSet, HashMap};
use std::io;
use std::path::PathBuf;

pub fn run_tui(alias_file_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Initialize database and deleted commands
    let mut database = Database {
        command_list: BTreeSet::new(),
        reverse_command_map: HashMap::new(),
        total_num_commands: 0,
        total_score: 0,
    };
    let mut deleted_commands = Deleted_Commands {
        deleted_commands: BTreeSet::new(),
    };

    // Create app and load initial data
    let mut app = App::new(alias_file_path.clone());
    app.load_commands(&database);

    // Run the app
    let res = run_app(
        &mut terminal,
        &mut app,
        &mut database,
        &mut deleted_commands,
    );

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    database: &mut Database,
    deleted_commands: &mut Deleted_Commands,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| render_ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                if let Some(operation) = app.handle_key_event(key.code) {
                    handle_operation(operation, app, database, deleted_commands);
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
    deleted_commands: &mut Deleted_Commands,
) {
    let alias_file_path = app.alias_file_path.to_string_lossy().to_string();

    match operation {
        Operation::Add { alias } => {
            if let Some(eq_pos) = alias.find('=') {
                let alias_name = alias[..eq_pos].trim();
                let command = alias[eq_pos + 1..].trim();
                alias_ops::add_alias(&alias_file_path, alias_name, command);
                app.status_message = format!("Added alias: {} = {}", alias_name, command);
            } else {
                app.status_message = "Invalid alias format".to_string();
            }
        }
        Operation::Remove { alias } => {
            alias_ops::remove_alias(&alias_file_path, &alias);
            app.status_message = format!("Removed alias: {}", alias);
        }
        Operation::Change { alias } => {
            if let Some(eq_pos) = alias.find('=') {
                let alias_name = alias[..eq_pos].trim();
                let command = alias[eq_pos + 1..].trim();
                alias_ops::remove_alias(&alias_file_path, alias_name);
                alias_ops::add_alias(&alias_file_path, alias_name, command);
                app.status_message = format!("Changed alias: {} = {}", alias_name, command);
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
        }
        Operation::Tui => {
            // Already in TUI mode, do nothing
        }
    }
}
