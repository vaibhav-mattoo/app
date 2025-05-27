use std::collections::BTreeSet;

#[derive(Debug)]
pub struct Database {
    command_list: BTreeSet<Command>,
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
    last_access_time: i64,
    frequency: i32,
    length: i16,
    score: i8,
    command_text: String,
    number_of_words: i8,
}
