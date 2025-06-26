use std::fs::File;
use std::io::{BufRead, Write, BufReader};

// funcs to do:
// func to get vector < pair < command, alias > > from file
// func to add alias to file
// func to remove alias from file
// func to get list of aliases from file

pub fn get_aliases(file_path: &str) -> Vec<(String, String)> {
    let file = File::open(file_path).expect("Could not open alias file");
    // if file not there, create one
    if file.metadata().is_err() {
        File::create(file_path).expect("Could not create alias file");
        return Vec::new(); // Return empty vector if file was just created
    }
    let reader = BufReader::new(file);
    let mut aliases = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Could not read line from alias file");
        let line = line.trim();
        if line.starts_with ("alias ") {
            let line_in = line[6..].trim();
            // now split by =
            if let Some(eq_index) = line_in.find('=') {
                let alias = line_in[..eq_index].trim().to_string();
                let mut command = line_in[(eq_index + 1)..].trim();
                if (command.starts_with('\'') && command.ends_with('\'')) ||
                   (command.starts_with('"') && command.ends_with('"')) {
                    command = &command[1..command.len()-1];
                }
                let command = command.to_string();
                aliases.push((alias, command));
            } else {
                println!("Invalid alias format in line: {}", line);
            }
        }
    }

    aliases
}

pub fn write_aliases(file_path: &str, aliases: Vec<(String, String)>) {
    let mut file = File::create(file_path).expect("Could not create alias file");
    for (alias, command) in aliases {
        let line = format!("alias {}='{}'\n", alias, command);
        file.write_all(line.as_bytes()).expect("Could not write to alias file");
    }
    file.flush().expect("Could not flush alias file");
}

pub fn get_aliases_list (file_path: &str) -> Vec<(String, String)> {
    let aliases = get_aliases(file_path);
    aliases
}

pub fn add_alias_to_file(file_path: &str, alias: &str, command: &str) {
    let mut aliases = get_aliases (file_path);
    // Check if alias already exists, just the alias part
    if aliases.iter().any(|(a, _)| a == alias) {
        println!("Alias '{}' already exists.", alias);
        return;
    }
    // Add new alias
    aliases.push((alias.to_string(), command.to_string()));
    write_aliases(file_path, aliases);
}

pub fn remove_alias_from_file(file_path: &str, alias: &str) {
    let mut aliases = get_aliases(file_path);
    // Remove alias if it exists
    if let Some(pos) = aliases.iter().position(|(a, _)| a == alias) {
        aliases.remove(pos);
        write_aliases(file_path, aliases);
    } else {
        println!("Alias '{}' not found.", alias);
    }
}