extern crate core;

use std::fs;
use crate::args::{Cli, Commands};
use clap::{Parser};

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args: Cli = Cli::parse();

    match args.command {
        Commands::Encode { file, chunk_type, message } => {
            match file {
                Some(oss) => {
                    let new_png = Commands::encode(&oss, chunk_type, message)?;
                    fs::write(&oss, new_png.as_bytes().as_slice())?;
                }
                None => panic!("no png file"),
            }
        }
        Commands::Decode { file, chunk_type } => {
            match file {
                Some(oss) => {
                    match Commands::decode(&oss, chunk_type) {
                        Some(msg) => println!("{}", msg),
                        None => println!("No message for type"),
                    }
                }
                None => panic!("no png file"),
            }
        }
        Commands::Remove { file, chunk_type } => {
            match file {
                Some(oss) => {
                    let new_png = Commands::remove(&oss, chunk_type)?;
                    fs::write(&oss, new_png.as_bytes().as_slice())?;
                }
                None => panic!("no png file"),
            }
        }
        Commands::Print { file } => {
            match file {
                Some(oss) => {
                    match Commands::print(&oss) {
                        Ok(types) => {
                            println!("{}", types.join("\r\n"));
                        }
                        Err(_) => {
                            panic!("empty file")
                        }
                    }
                }
                None => panic!("no png file"),
            }
        }
    }

    Ok(())
}
