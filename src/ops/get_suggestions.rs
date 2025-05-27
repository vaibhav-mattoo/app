use crate::database::database_structs::{Command, Database};

pub fn get_suggestions(num: Option<usize>, db: &Database) -> Vec<&Command> {
    db.get_top_commands(num)
}

