use self::args::{Cli, Commands};
use self::commands::{add_command, init_command};
use clap::Parser;

mod args;
mod commands;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add => {
            add_command().ok();
        }
        Commands::Init => {
            init_command().ok();
        }
    }
}
