use crate::database::database_structs::{Database,Deleted_Commands};

pub fn delete_suggestion(alias: &str, db: &mut Database,deleted_commands: &mut Deleted_Commands) {
    db.remove_command(&alias.to_string(), deleted_commands);
    println!("Suggestion '{}' deleted successfully.", alias);
}
