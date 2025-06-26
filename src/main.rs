mod cli;
mod ops;
mod database;

use std::env;
use cli::arg_handler::parse_args;
use cli::cli_data::Operation;
use ops::get_suggestions::{get_suggestions};
use ops::insert_command::insert_command;
use ops::delete_suggestion::delete_suggestion;
use ops::add_alias::add_alias;
use ops::remove_alias::remove_alias;
use database::database_structs::{Database, DeletedCommands};
use ops::alias_ops::{get_aliases_list};

fn main() {
   
    let args: Vec<String> = env::args().collect();
        if args.len() >=3 {
            if args[0]=="app" && args[1]=="app" && (args[2]!="add" || args[2]!="remove" || args[2]!="list" || args[2]!="change" || args[2]!="get_suggestions" || args[2]!="delete_suggestion") {
                eprintln!("Usage: {} <operation> [options]", args[0]);
                eprintln!("Available operations: add, remove, list, change, get_suggestions, delete_suggestion");
                return;
            }
            if args[0]!="add" || args[0]!="remove" || args[0]!="list" || args[0]!="change" || args[0]!="get_suggestions" || args[0]!="delete_suggestion" {
                eprintln!("Usage: {} <operation> [options]", args[2]);
                eprintln!("Available operations: add, remove, list, change, get_suggestions, delete_suggestion");
                return;
            }
            // eprintln!("Usage: {} <operation> [options]", args[0]);
            // eprintln!("Available operations: add, remove, list, change, get_suggestions, delete_suggestion");
            // return;
    }
    
    let cli = parse_args();
    let mut db: Database = Database{
        command_list: std::collections::BTreeSet::new(),
        reverse_command_map: std::collections::HashMap::new(),
        total_num_commands: 0,
        total_score: 0,
    };
    let db_ref: &mut Database = &mut db;
    let mut deleted_commands: DeletedCommands = DeletedCommands {
        deleted_commands: std::collections::BTreeSet::new(),
    };
    let dc_ref: &mut DeletedCommands = &mut deleted_commands;

    let file_path = "./store.aliases";

    insert_command("example command".to_string(), db_ref, dc_ref);
    // delete_suggestion("example", db_ref, dc_ref);
    insert_command("example command lil big".to_string(), db_ref, dc_ref);
    insert_command("example command".to_string(), db_ref, dc_ref);
    insert_command("HI".to_string(), db_ref, dc_ref);
    insert_command("BI".to_string(), db_ref, dc_ref);
    insert_command("BI".to_string(), db_ref, dc_ref);
    insert_command("HI".to_string(), db_ref, dc_ref);

    println!("{:#?}", db_ref);


    match &cli.operation {
        Operation::Add { alias } => {
            println!("alias added: {}", alias);
            println!("alias file: {:?}", cli.alias_file_path);
            println!("command: {}", cli.command);
            add_alias(db_ref, dc_ref, file_path, alias, cli.command.as_str());
            // println!("{:#?}", dc_ref);
        }
        Operation::Remove { alias } => {
            println!("remove alias: {}", alias);
            remove_alias(dc_ref, file_path, alias);
        }
        Operation::List => {
            // println!("list is called");
            let aliases = get_aliases_list(file_path);
            println!("Aliases: {:?}", aliases);
        }
        Operation::Change { alias } => {
            println!("change alias called: {}", alias);
        }
        Operation::GetSuggestions { num  } => {
            let list = get_suggestions(*num, db_ref);
            println!("{}", list.iter().map(|cmd| cmd.command_text.clone()).collect::<Vec<String>>().join(", "));
        }
        Operation::DeleteSuggestion { alias } => {
           delete_suggestion(alias, db_ref, dc_ref);
        }
    }
}
