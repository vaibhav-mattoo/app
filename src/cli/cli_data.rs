use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, disable_help_subcommand = true)]
pub struct Cli {
    #[command(subcommand)]
    pub operation: Option<Operation>,

    #[arg(short, long, value_name = "ALIAS_FILE_PATH")]
    pub alias_file_path: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
pub enum Operation {
    Add {
        alias: String,
        #[arg(short = 'c', long)]
        command: String,
    },
    Remove {
        alias: String,
    },
    List,
    Change {
        old_alias: String,
        new_alias: String,
    },
    GetSuggestions {
        #[arg(short = 't', long)]
        num: Option<usize>,
    },
    DeleteSuggestion {
        alias: String,
    },
    Tui,
    Init {
        #[arg(value_enum)]
        shell: InitShell,
    },
}

#[derive(ValueEnum, Clone, Debug)]
pub enum InitShell {
    Bash,
    Zsh,
    Fish,
    #[clap(alias = "ksh")]
    Posix,
}
