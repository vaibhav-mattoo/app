use std::collections::BTreeSet;

use super::database_structs::{Command, Database, Deleted_Commands};

fn get_score(command: &Command) -> i32 {
    // Make a simple scoring func for now, just return the length
    command.length as i32 + command.frequency + command.number_of_words as i32
}

impl Database {
    pub fn add_command(&mut self, command_str: String, deleted_commands: &Deleted_Commands) {
        if !deleted_commands.deleted_commands.contains(&command_str) {
            // check if command already exists
            if let Some(existing_command) = self.reverse_command_map.get(&command_str) {
                let mut updated_command = existing_command.clone();
                updated_command.update();

                self.command_list.remove(existing_command);
                self.reverse_command_map.remove(&command_str);

                self.command_list.insert(updated_command.clone());
                self.reverse_command_map.insert(command_str, updated_command);
            } else {
                let new_command = Command::new(command_str.clone());
                self.command_list.insert(new_command.clone());
                self.reverse_command_map.insert(command_str, new_command);
            }
            self.total_num_commands += 1;
            self.total_score += self.reverse_command_map.get(&command_str).unwrap().score as i64;
        }
    }

    pub fn remove_command(&mut self, command_str: &String, deleted_commands: &mut Deleted_Commands) {
        // first check if already exists in Deleted_Commands
        if !deleted_commands.deleted_commands.contains(&command_str) {
            deleted_commands
                .deleted_commands
                .insert(command_str.clone());
            // then remove from command list, first get command from reverse map
            let command_to_remove = self.reverse_command_map.get(&command_str);
            // if not null, remove it fromm both
            if let Some(cmd) = command_to_remove {
                self.command_list.remove(cmd);
                self.total_num_commands -= 1;
                self.total_score -= cmd.score as i64;
                self.reverse_command_map.remove(&command_str);
            }
        }
    }

    pub fn get_top_commands(&self, n: usize) -> Vec<&Command> {
        // will be sorted by score, get top n
        self.command_list.iter().take(n).collect()
    }

    pub fn get_total_score(&self) -> i64 {
        self.total_score
    }
}

impl Command {
    pub fn new(command_text: String) -> Self {
        let length = command_text.len() as i16;
        let number_of_words = command_text.split_whitespace().count() as i8;
        let frequency = 1;
        let last_access_time = 0; // or set as needed

        // Create a temporary Command to calculate the score
        let temp_command = Command {
            last_access_time,
            frequency,
            length,
            score: 0, // placeholder
            command_text: command_text.clone(),
            number_of_words,
        };

        let score = get_score(&temp_command);

        Command {
            last_access_time,
            frequency,
            length,
            score,
            command_text,
            number_of_words,
        }
    }
    pub fn update(&mut self) {
        // here we update the last_access time, frequency, and score
        self.last_access_time = 0; // placeholder
        self.frequency += 1;
        self.score = get_score(self);
    }
}
