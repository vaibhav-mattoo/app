mod cli;
mod database;
mod ops;

use cli::arg_handler::parse_args;
use cli::cli_data::Operation;
use database::database_structs::{Database, DeletedCommands};
use database::persistence::{
    ensure_data_directory, get_database_path, get_deleted_commands_path, load_database,
    load_deleted_commands, save_database, save_deleted_commands,
};
use ops::add_alias::add_alias;
use ops::alias_ops::get_aliases_list;
use ops::delete_suggestion::delete_suggestion;
use ops::get_suggestions::get_suggestions;
use ops::insert_command::insert_command;
use ops::remove_alias::remove_alias;
use std::env;

fn main() {
    // Ensure data directory exists
    if let Err(e) = ensure_data_directory() {
        eprintln!("Failed to create data directory: {}", e);
        return;
    }

    // Load database and deleted commands from persistent storage
    let db_path = get_database_path();
    let deleted_commands_path = get_deleted_commands_path();

    let mut db = match load_database(&db_path) {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Failed to load database: {}", e);
            Database {
                command_list: std::collections::BTreeSet::new(),
                reverse_command_map: std::collections::HashMap::new(),
                total_num_commands: 0,
                total_score: 0,
            }
        }
    };

    let mut deleted_commands = match load_deleted_commands(&deleted_commands_path) {
        Ok(dc) => dc,
        Err(e) => {
            eprintln!("Failed to load deleted commands: {}", e);
            DeletedCommands {
                deleted_commands: std::collections::BTreeSet::new(),
            }
        }
    };

    let db_ref: &mut Database = &mut db;
    let dc_ref: &mut DeletedCommands = &mut deleted_commands;

    let file_path = "./store.aliases";
    let command_strings: Vec<String> = env::args().collect();
    let command = command_strings[1..].join(" ");

    // Check if this is a subcommand (starts with "app") or a direct command
    if command_strings.is_empty() {
        eprintln!("No arguments provided");
        return;
    }

    println!("{}", command_strings[0]);
    if command_strings[0] == "/home/vaibhav/alias_suggestor/app/target/debug/app" {
        println!("Detected the command");
        // This is a subcommand, parse and handle it
        let cli = parse_args();

        match &cli.operation {
            Operation::Add { alias } => {
                println!("alias added: {}", alias);
                println!("alias file: {:?}", cli.alias_file_path);
                println!("command: {}", cli.command);
                add_alias(db_ref, dc_ref, file_path, alias, cli.command.as_str());
                // Save after adding alias
                if let Err(e) = save_database(db_ref, &db_path) {
                    eprintln!("Failed to save database: {}", e);
                }
                if let Err(e) = save_deleted_commands(dc_ref, &deleted_commands_path) {
                    eprintln!("Failed to save deleted commands: {}", e);
                }
            }
            Operation::Remove { alias } => {
                println!("remove alias: {}", alias);
                remove_alias(dc_ref, file_path, alias);
                // Save after removing alias
                if let Err(e) = save_deleted_commands(dc_ref, &deleted_commands_path) {
                    eprintln!("Failed to save deleted commands: {}", e);
                }
            }
            Operation::List => {
                let aliases = get_aliases_list(file_path);
                println!("Aliases: {:?}", aliases);
            }
            Operation::Change { alias } => {
                println!("change alias called: {}", alias);
            }
            Operation::GetSuggestions { num } => {
                let list = get_suggestions(*num, db_ref);
                println!(
                    "{}",
                    list.iter()
                        .map(|cmd| cmd.command_text.clone())
                        .collect::<Vec<String>>()
                        .join(", ")
                );
            }
            Operation::DeleteSuggestion { alias } => {
                delete_suggestion(alias, db_ref, dc_ref);
                // Save after deleting suggestion
                if let Err(e) = save_database(db_ref, &db_path) {
                    eprintln!("Failed to save database: {}", e);
                }
                if let Err(e) = save_deleted_commands(dc_ref, &deleted_commands_path) {
                    eprintln!("Failed to save deleted commands: {}", e);
                }
            }
        }
    } else {
        // This is a direct command to insert
        // let command = command_strings.join(" ");
        insert_command(command.to_string(), db_ref, dc_ref);

        // Save database after inserting command
        if let Err(e) = save_database(db_ref, &db_path) {
            eprintln!("Failed to save database: {}", e);
        }
        if let Err(e) = save_deleted_commands(dc_ref, &deleted_commands_path) {
            eprintln!("Failed to save deleted commands: {}", e);
        }

        // println!("{:#?}", db_ref);
        println!("hi");
    }
}
