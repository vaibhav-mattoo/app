mod cli;

use cli::arg_handler::parse_args;
use cli::cli_data::Operation;

fn main() {
    println!("Hello, world!");
    let cli = parse_args();

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
        Operation::GetSuggestions { alias } => {
            println!("get suggestions called: {}", alias);
        }
        Operation::DeleteSuggestion { alias } => {
            println!("Delete suggestion called: {}", alias);
        }
    }
}
