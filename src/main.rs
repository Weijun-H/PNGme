mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use clap::{Parser, Subcommand, Args};
use crate::args::PngMeArgs::{Encode, Decode, Remove, Print};
use crate::args::Cli;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Encode(args) => {
            print!("Encoding message: ");
        }
        Decode(args) => {
            print!("Decoding message: ");
        }
        Remove(args) => {
            print!("Removing chunk: ");
        }
        Print(args) => {
            print!("Printing chunks: ");
        }
    }
        
    Ok(())
}
