use std::convert::TryFrom;
use std::str::FromStr;
use std::fs::File;
use std::io::prelude::*;

use crate::{Error, Result};
use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::chunk::Chunk;

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodeArgs) -> Result<()> {
    let mut png = Png::from_file(args.file_path)?;
    let chunk_type = ChunkType::from_str(args.chunk_type.as_str())?;
    let chunk = Chunk::new(chunk_type, args.message.as_bytes().to_vec());
    png.append_chunk(chunk);

    let mut file = File::create(args.output_file.unwrap_or("./output.png".to_string()))?;
    file.write_all(&png.as_bytes())?;

    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> Result<()> {
    let png = Png::from_file(args.file_path)?;
    match png.chunk_by_type(args.chunk_type.as_str()) {
        Some(chunk) => {
            let message = chunk.data_as_string()?;
            println!("{}", message);
        }
        None => {
            println!("No message found");
        }
    }
    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> Result<()> {
    let mut png = Png::from_file(args.file_path)?;

    match png.remove_chunk(args.chunk_type.as_str()) {
        Ok(chunk) => {
            println!("Removed chunk: {:?}", chunk.data_as_string()?);
            let mut file = File::create("./output.png")?;
            file.write_all(&png.as_bytes())?;
        }
        _ => {
            println!("No chunk found");
        }
    }
    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> Result<()> {
    let png = Png::from_file(args.file_path)?;
    for chunk in png.chunks() {
        println!("{:?}", chunk);
    }
    Ok(())
}