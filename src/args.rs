use std::ffi::OsString;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "pngme")]
#[command(about = "A pngme CLI", long_about = None)]
#[command(author, version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(arg_required_else_help = true)]
    #[command(about = "encode a message with a key into the PNG file")]
    Encode{
        #[arg(value_name = "PNG file")]
        file: Option<OsString>,
        #[arg(value_name = "message key")]
        chunk_type: Option<String>,
        #[arg(value_name = "message content")]
        message: Option<String>,
    },
    #[command(arg_required_else_help = true)]
    #[command(about ="decode the key's message from the PNG file")]
    Decode{
        #[arg(value_name = "PNG file")]
        file: Option<OsString>,
        #[arg(value_name = "message key")]
        chunk_type: Option<String>,
    },
    #[command(arg_required_else_help = true)]
    #[command(about ="remove the key's message from the PNG file")]
    Remove{
        #[arg(value_name = "PNG file")]
        file: Option<OsString>,
        #[arg(value_name = "message key")]
        chunk_type: Option<String>,
    },
    #[command(arg_required_else_help = true)]
    #[command(about ="show all keys in the PNG file")]
    Print {
        #[arg(value_name = "PNG file")]
        file: Option<OsString>
    },
}
