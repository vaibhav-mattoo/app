extern crate clap;
mod cli;
mod database;
mod ops;
mod tui;

use cli::arg_handler::parse_args;
use cli::cli_data::Operation;
use database::database_structs::{Database, Deleted_Commands};
use ops::delete_suggestion::delete_suggestion;
use ops::get_suggestions::get_suggestions;
use ops::insert_command::insert_command;
use tui::run_tui;

fn main() {
    let cli = parse_args();
    let mut db: Database = Database {
        command_list: std::collections::BTreeSet::new(),
        reverse_command_map: std::collections::HashMap::new(),
        total_num_commands: 0,
        total_score: 0,
    };
    let db_ref: &mut Database = &mut db;
    let mut deleted_commands: Deleted_Commands = Deleted_Commands {
        deleted_commands: std::collections::BTreeSet::new(),
    };
    let dc_ref: &mut Deleted_Commands = &mut deleted_commands;

    insert_command("example command".to_string(), db_ref, dc_ref);
    insert_command("example command lil big".to_string(), db_ref, dc_ref);
    insert_command("example command".to_string(), db_ref, dc_ref);
    // insert_command("HI".to_string(), db_ref, dc_ref);
    // insert_command("BI".to_string(), db_ref, dc_ref);
    // insert_command("BI".to_string(), db_ref, dc_ref);
    // insert_command("HI".to_string(), db_ref, dc_ref);

    match &cli.operation {
        Operation::Add { alias } => {
            println!("alias added: {}", alias);
            println!("alias file: {:?}", cli.alias_file_path);
            println!("command list number: {}", cli.command_list_number);
        }
        Operation::Remove { alias } => {
            println!("remove alias: {}", alias);
        }
        Operation::List => {
            println!("list is called");
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
        }
        Operation::Tui => {
            if let Err(e) = run_tui(cli.alias_file_path) {
                eprintln!("TUI error: {}", e);
            }
        }
    }
}
