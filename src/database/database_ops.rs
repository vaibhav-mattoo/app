use std::collections::BTreeSet;

use super::database_structs::{Command, Database, Deleted_Commands};

impl Database {

    pub fn add_command(&mut self, command: Command, deleted_commands: &Deleted_Commands) {
        // first check if it is there in deleted_commands
        // then check if already exists in database, if exists update score and insert again
        self.command_list.insert(command);
        self.total_num_commands += 1;
    }

    pub fn remove_command(&mut self, command: &Command, deleted_commands: &mut Deleted_Commands) {
        if self.command_list.remove(command) {
            self.total_num_commands -= 1;
        }
        let command_str: String = command.command_text.to_string();
        deleted_commands.deleted_commands.insert(command_str);
    }

    pub fn get_total_score(&self) -> i64 {
        self.total_score
    }
}