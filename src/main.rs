mod cli;
mod ops;
mod database;

use cli::arg_handler::parse_args;
use cli::cli_data::Operation;
use ops::get_suggestions::get_suggestions;
use database::database_structs::{Database, Deleted_Commands};

fn main() {
    let cli = parse_args();
    let mut db: Database = Database{
        command_list: std::collections::BTreeSet::new(),
        reverse_command_map: std::collections::HashMap::new(),
        total_num_commands: 0,
        total_score: 0,
    };
    let deleted_commands: Deleted_Commands = Deleted_Commands {
        deleted_commands: std::collections::BTreeSet::new(),
    };

    db.add_command("example command".to_string(), &deleted_commands);

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
        Operation::GetSuggestions { } => {
            let list = get_suggestions(Some(5), &db);
            println!("{}", list.iter().map(|cmd| cmd.command_text.clone()).collect::<Vec<String>>().join(", "));
        }
        Operation::DeleteSuggestion { alias } => {
            println!("Delete suggestion called: {}", alias);
        }
    }
}
