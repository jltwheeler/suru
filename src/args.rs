use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a todo via an interactive prompt
    Add {
        #[clap(value_parser)]
        todo: String,
    },
    /// Initialise the todo sqlite db
    Init,
}
