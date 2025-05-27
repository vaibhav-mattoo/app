mod database_structs;
mod database_ops;
use crate::database_structs::{Command,Database};
use crate::database_ops::Command::get_top_commands;

fn give_suggestions(num:Option<i8>,db:&Database)->Vec<&Command>{
    Command::get_top_commands(db,num)
}

