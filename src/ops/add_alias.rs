use crate::database::database_structs::{Database,DeletedCommands};
use crate::ops::alias_ops::{add_alias_to_file};

pub fn add_alias(file_path: &str, alias: &str, command: &str) {
    // first 

    // second add to file.
    add_alias_to_file(file_path, alias, command);
}

