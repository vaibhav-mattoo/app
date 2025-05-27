use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub operation: Operation,

    #[arg(short, long, default_value = ".bash_aliases")]
    pub alias_file_path: PathBuf,

    #[arg(short = 'n', long, default_value = "0")]
    pub command_list_number: u8,
}

#[derive(Subcommand, Debug)]
pub enum Operation {
    Add { alias: String },
    Remove { alias: String },
    List,
    Change { alias: String },
    GetSuggestions {
        #[arg(short = 't', long)]
        num: Option<usize>,
    },
    DeleteSuggestion { alias: String },
}
