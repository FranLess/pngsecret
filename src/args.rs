use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// does testing things
    Encode(EncodeArgs),
    Remove(RemoveArgs),
    Decode(DecodeArgs),
    Print(PrintArgs),
}
#[derive(Args)]
pub struct EncodeArgs {
    file_path: PathBuf,
    chunk_type: String,
    message: String,
    output_file: Option<PathBuf>,
}
#[derive(Args)]
pub struct RemoveArgs {
    file_path: PathBuf,
    chunk_type: String,
}

#[derive(Args)]
pub struct DecodeArgs {
    file_path: PathBuf,
    chunk_type: String,
}

#[derive(Args)]
pub struct PrintArgs {
    file_path: PathBuf,
}
