use std::collections::BTreeSet;

#[derive(Debug)]
pub struct Database {
    command_list: BTreeSet<Command>,
    total_num_commands: i32,
    total_score: i64,
}

#[derive(Debug)]
pub struct deleted_commands {
    deleted_commands: BTreeSet<Command>,
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
    command_text: &str,
    number_of_words: i8,
}
