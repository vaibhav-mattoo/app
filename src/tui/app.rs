use crate::database::database_structs::{Command, Database};
use ratatui::widgets::ListState;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum AppMode {
    Main,
    AddAlias,
    RemoveAlias,
    ChangeAlias,
    GetSuggestions,
    DeleteSuggestion,
}

#[derive(Debug)]
pub struct App {
    pub mode: AppMode,
    pub input: String,
    pub cursor_position: usize,
    pub list_state: ListState,
    pub commands: Vec<Command>,
    pub filtered_commands: Vec<Command>,
    pub alias_file_path: PathBuf,
    pub should_quit: bool,
    pub status_message: String,
    pub show_popup: bool,
    pub popup_message: String,
}

impl App {
    pub fn new(alias_file_path: PathBuf) -> Self {
        Self {
            mode: AppMode::Main,
            input: String::new(),
            cursor_position: 0,
            list_state: ListState::default(),
            commands: Vec::new(),
            filtered_commands: Vec::new(),
            alias_file_path,
            should_quit: false,
            status_message: "Welcome to App TUI!".to_string(),
            show_popup: false,
            popup_message: String::new(),
        }
    }

    pub fn load_commands(&mut self, database: &mut Database) {
        self.commands = database
            .get_top_commands(None)
            .iter()
            .map(|cmd| (*cmd).clone())
            .collect();
        self.filtered_commands = self.commands.clone();
        // Reset list selection when commands are reloaded
        self.list_state.select(None);
    }

    pub fn filter_commands(&mut self) {
        if self.input.is_empty() {
            self.filtered_commands = self.commands.clone();
        } else {
            self.filtered_commands = self
                .commands
                .iter()
                .filter(|cmd| {
                    cmd.command_text
                        .to_lowercase()
                        .contains(&self.input.to_lowercase())
                })
                .cloned()
                .collect();
        }
        self.list_state.select(None);
    }

    pub fn show_popup(&mut self, message: String) {
        self.popup_message = message;
        self.show_popup = true;
    }

    pub fn hide_popup(&mut self) {
        self.show_popup = false;
        self.popup_message.clear();
    }

    pub fn get_selected_command(&self) -> Option<&Command> {
        if let Some(selected) = self.list_state.selected() {
            self.filtered_commands.get(selected)
        } else {
            None
        }
    }

    pub fn clear_input(&mut self) {
        self.input.clear();
        self.cursor_position = 0;
    }

    pub fn set_mode(&mut self, mode: AppMode) {
        self.mode = mode;
        self.clear_input();
    }
}
