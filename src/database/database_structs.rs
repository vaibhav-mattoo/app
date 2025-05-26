use std::collections::BTreeSet;

#[derive(Debug)]
struct Database {
    command_list: BTreeSet<Command>,
    total_num_commands: i32,
    total_score: i64,
}

#[derive(Debug)]
struct Suggestions {
    alias_suggestion: String,
}

#[derive(Debug)]
struct Command {
    last_access_time: i64,
    frequency: i32,
    length: i16,
    score: i8,
    command_text: &str,
    number_of_words: i8,
}
