use clap::{Parser, Subcommand, Args};
use pngme::args::PngMeArgs::{Encode, Decode, Remove, Print};
use pngme::args::{Cli};
use pngme::commands;
use pngme::{Result, Error};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Encode(args) => {
            commands::encode(args)?;
        }
        Decode(args) => {
            commands::decode(args)?;
        }
        Remove(args) => {
            commands::remove(args)?;
        }
        Print(args) => {
            commands::print_chunks(args)?;
        }
    }
    Ok(())
}
