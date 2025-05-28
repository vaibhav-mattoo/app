use crate::database::database_structs::{Database, Deleted_Commands};

pub fn insert_command(command_str: String, db: &mut Database, deleted_commands: &Deleted_Commands) {
    db.add_command(command_str, deleted_commands);
    // println!("Command '{}' inserted successfully.", command_str);why cant i just use println! here? some error related to the borrowing of command_str
}