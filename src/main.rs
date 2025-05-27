mod cli;
mod ops;
mod database;

use cli::arg_handler::parse_args;
use cli::cli_data::Operation;
use ops::get_suggestions::give_suggestions;
use database::database_structs::{Database};

fn main() {
    let cli = parse_args();
    let db: Database=Database.new();

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
        Operation::GetSuggestions { alias_type } => {
            give_suggestions();
        }
        Operation::DeleteSuggestion { alias } => {
            println!("Delete suggestion called: {}", alias);
        }
    }
}
