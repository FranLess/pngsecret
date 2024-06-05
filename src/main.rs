mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
use args::{Cli, Commands};
use clap::{Parser, ValueEnum};
use commands::{decode, encode, print, remove};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    let cli = Cli::parse();
    match &cli.commands {
        Commands::Encode(args) => encode(args),
        Commands::Decode(args) => decode(args),
        Commands::Remove(args) => remove(args),
        Commands::Print(args) => print(args),
    }
}
