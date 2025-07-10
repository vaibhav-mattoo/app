use crate::cli::cli_data::Operation;
use crate::tui::app::{App, AppMode};
use ratatui::crossterm::event::KeyCode;

impl App {
    pub fn handle_key_event(&mut self, key: KeyCode) -> Option<Operation> {
        if self.show_popup {
            match key {
                KeyCode::Esc | KeyCode::Enter | KeyCode::Char(' ') => {
                    self.hide_popup();
                    return None;
                }
                _ => return None,
            }
        }

        match self.mode {
            AppMode::Main => self.handle_main_mode(key),
            AppMode::AddAlias => self.handle_input_mode(key, "add"),
            AppMode::RemoveAlias => self.handle_input_mode(key, "remove"),
            AppMode::ChangeAlias => self.handle_input_mode(key, "change"),
            AppMode::GetSuggestions => self.handle_input_mode(key, "suggestions"),
            AppMode::DeleteSuggestion => self.handle_input_mode(key, "delete"),
        }
    }

    fn handle_main_mode(&mut self, key: KeyCode) -> Option<Operation> {
        match key {
            KeyCode::Char('q') => {
                self.should_quit = true;
                None
            }
            KeyCode::Char('a') => {
                self.set_mode(AppMode::AddAlias);
                self.status_message = "Enter alias to add (format: alias=command):".to_string();
                None
            }
            KeyCode::Char('r') => {
                self.set_mode(AppMode::RemoveAlias);
                self.status_message = "Enter alias to remove:".to_string();
                None
            }
            KeyCode::Char('c') => {
                self.set_mode(AppMode::ChangeAlias);
                self.status_message =
                    "Enter alias to change (format: alias=new_command):".to_string();
                None
            }
            KeyCode::Char('s') => {
                self.set_mode(AppMode::GetSuggestions);
                self.status_message =
                    "Enter number of suggestions (or press Enter for default):".to_string();
                None
            }
            KeyCode::Char('d') => {
                self.set_mode(AppMode::DeleteSuggestion);
                self.status_message = "Enter command text to delete from suggestions:".to_string();
                None
            }
            KeyCode::Char('l') => Some(Operation::List),
            KeyCode::Up => {
                if !self.filtered_commands.is_empty() {
                    let selected = match self.list_state.selected() {
                        Some(i) => {
                            if i > 0 {
                                i - 1
                            } else {
                                i
                            }
                        }
                        None => 0,
                    };
                    self.list_state.select(Some(selected));
                }
                None
            }
            KeyCode::Down => {
                if !self.filtered_commands.is_empty() {
                    let selected = match self.list_state.selected() {
                        Some(i) => {
                            if i < self.filtered_commands.len() - 1 {
                                i + 1
                            } else {
                                i
                            }
                        }
                        None => 0,
                    };
                    self.list_state.select(Some(selected));
                }
                None
            }
            KeyCode::Enter => {
                if let Some(cmd) = self.get_selected_command() {
                    self.show_popup(format!(
                        "Selected: {}\nScore: {}\nFrequency: {}\nLast used: {}",
                        cmd.command_text, cmd.score, cmd.frequency, cmd.last_access_time
                    ));
                }
                None
            }
            KeyCode::Char('/') => {
                self.clear_input();
                self.status_message = "Type to filter commands... (Esc to clear)".to_string();
                None
            }
            KeyCode::Char(c) => {
                self.input.push(c);
                self.cursor_position = self.input.len();
                self.filter_commands();
                self.status_message = format!("Filtering: '{}' (Esc to clear)", self.input);
                None
            }
            KeyCode::Backspace => {
                if !self.input.is_empty() {
                    self.input.pop();
                    self.cursor_position = self.input.len();
                    self.filter_commands();
                    if self.input.is_empty() {
                        self.status_message = "Filter cleared".to_string();
                    } else {
                        self.status_message = format!("Filtering: '{}'", self.input);
                    }
                }
                None
            }
            KeyCode::Esc => {
                if !self.input.is_empty() {
                    self.clear_input();
                    self.filter_commands();
                    self.status_message = "Filter cleared".to_string();
                }
                None
            }
            _ => None,
        }
    }

    fn handle_input_mode(&mut self, key: KeyCode, operation: &str) -> Option<Operation> {
        match key {
            KeyCode::Enter => {
                let input_value = self.input.trim().to_string();
                let operation_result = self.process_input_operation(operation, input_value);
                self.set_mode(AppMode::Main);
                operation_result
            }
            KeyCode::Char(c) => {
                self.input.insert(self.cursor_position, c);
                self.cursor_position += 1;
                if operation == "delete" {
                    self.filter_commands();
                }
                None
            }
            KeyCode::Backspace => {
                if self.cursor_position > 0 {
                    self.input.remove(self.cursor_position - 1);
                    self.cursor_position -= 1;
                    if operation == "delete" {
                        self.filter_commands();
                    }
                }
                None
            }
            KeyCode::Left => {
                if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                }
                None
            }
            KeyCode::Right => {
                if self.cursor_position < self.input.len() {
                    self.cursor_position += 1;
                }
                None
            }
            KeyCode::Up | KeyCode::Down => {
                if operation == "delete" && !self.filtered_commands.is_empty() {
                    match key {
                        KeyCode::Up => {
                            let selected = match self.list_state.selected() {
                                Some(i) => {
                                    if i > 0 {
                                        i - 1
                                    } else {
                                        i
                                    }
                                }
                                None => 0,
                            };
                            self.list_state.select(Some(selected));
                        }
                        KeyCode::Down => {
                            let selected = match self.list_state.selected() {
                                Some(i) => {
                                    if i < self.filtered_commands.len() - 1 {
                                        i + 1
                                    } else {
                                        i
                                    }
                                }
                                None => 0,
                            };
                            self.list_state.select(Some(selected));
                        }
                        _ => {}
                    }
                }
                None
            }
            KeyCode::Tab => {
                if operation == "delete" {
                    if let Some(cmd) = self.get_selected_command() {
                        self.input = cmd.command_text.clone();
                        self.cursor_position = self.input.len();
                    }
                }
                None
            }
            KeyCode::Esc => {
                self.set_mode(AppMode::Main);
                self.status_message = "Operation cancelled.".to_string();
                None
            }
            _ => None,
        }
    }

    fn process_input_operation(
        &mut self,
        operation: &str,
        input_value: String,
    ) -> Option<Operation> {
        match operation {
            "add" => {
                if input_value.is_empty() {
                    self.status_message = "Alias cannot be empty".to_string();
                    return None;
                }

                if let Some(eq_pos) = input_value.find('=') {
                    let alias = input_value[..eq_pos].trim();
                    let command = input_value[eq_pos + 1..].trim();

                    if alias.is_empty() || command.is_empty() {
                        self.status_message = "Both alias and command must be provided".to_string();
                        return None;
                    }

                    self.status_message = format!("Added alias: {} = {}", alias, command);
                    Some(Operation::Add { alias: alias.to_string(), command: command.to_string() })
                } else {
                    self.status_message = "Invalid format. Use: alias=command".to_string();
                    None
                }
            }
            "remove" => {
                if input_value.is_empty() {
                    self.status_message = "Alias name cannot be empty".to_string();
                    None
                } else {
                    self.status_message = format!("Removed alias: {}", input_value);
                    Some(Operation::Remove { alias: input_value })
                }
            }
            "change" => {
                if input_value.is_empty() {
                    self.status_message = "Alias cannot be empty".to_string();
                    return None;
                }

                if let Some(eq_pos) = input_value.find('=') {
                    let old_alias = input_value[..eq_pos].trim();
                    let new_alias = input_value[eq_pos + 1..].trim();

                    if old_alias.is_empty() || new_alias.is_empty() {
                        self.status_message = "Both old and new alias must be provided".to_string();
                        return None;
                    }

                    self.status_message = format!("Changed alias: {} -> {}", old_alias, new_alias);
                    Some(Operation::Change { old_alias: old_alias.to_string(), new_alias: new_alias.to_string() })
                } else {
                    self.status_message = "Invalid format. Use: old_alias=new_alias".to_string();
                    None
                }
            }
            "suggestions" => {
                let num = if input_value.is_empty() {
                    None
                } else {
                    match input_value.parse::<usize>() {
                        Ok(n) if n > 0 => Some(n),
                        Ok(_) => {
                            self.status_message =
                                "Number must be greater than 0, using default".to_string();
                            None
                        }
                        Err(_) => {
                            self.status_message = "Invalid number, using default".to_string();
                            None
                        }
                    }
                };

                self.status_message = match num {
                    Some(n) => format!("Getting {} suggestions", n),
                    None => "Getting default number of suggestions".to_string(),
                };

                Some(Operation::GetSuggestions { num })
            }
            "delete" => {
                let command_to_delete = if input_value.is_empty() {
                    if let Some(cmd) = self.get_selected_command() {
                        cmd.command_text.clone()
                    } else {
                        self.status_message = "No command selected or entered".to_string();
                        return None;
                    }
                } else {
                    input_value
                };

                self.status_message = format!("Deleted suggestion: {}", command_to_delete);
                Some(Operation::DeleteSuggestion {
                    alias: command_to_delete,
                })
            }
            _ => None,
        }
    }
}
