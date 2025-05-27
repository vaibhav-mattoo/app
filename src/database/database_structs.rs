use std::collections:: {BTreeSet, HashMap};
use std::cmp::Reverse;

#[derive(Debug)]
pub struct Database {
    command_list: BTreeSet<Reverse<Command>>,
    reverse_command_map: HashMap<String, Command>,
    total_num_commands: i32,
    total_score: i64,
}

#[derive(Debug)]
pub struct Deleted_Commands {
    deleted_commands: BTreeSet<String>,
}

#[derive(Debug)]
pub struct Suggestions {
    alias_suggestion: Vec<String>,
}

#[derive(Debug)]
pub struct Command {
    score: i32,
    last_access_time: i64,
    frequency: i32,
    length: i16,
    command_text: String,
    number_of_words: i8,
}
