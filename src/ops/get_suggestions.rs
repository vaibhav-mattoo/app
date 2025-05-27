use crate::database::database_structs::{Command, Database,Deleted_Commands};

pub fn get_suggestions(num: Option<usize>, db: &mut Database) -> Vec<&Command> {
    db.get_top_commands(num)
}

