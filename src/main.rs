use self::args::{Cli, Commands};
use self::commands::init_command;
use clap::Parser;

mod args;
mod commands;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { todo } => {
            println!("the todo was: {}", todo)
        }
        Commands::Init => {
            init_command().ok();
        }
    }
}
