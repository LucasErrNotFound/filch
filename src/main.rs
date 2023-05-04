#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
mod listen_clipboard_data;
mod delete_clipboard_data;
use clap::{Parser, Subcommand};

/// Filch, a sneaky clipboard listener (not official, I guess)
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]

struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {

    /// Initiate filch 
    start {
        /// Listen to Clipboard's Data 
        #[arg(long)]
        listen: bool,

        /// Initiate file store 
        #[arg(long, requires("listen"))]
        filestore: bool,

        /// file path or file name
        #[arg(long, requires("filestore"))]
        filepath: String,

        /// Delete Clipboard's Data 
        #[arg(long)]
        delete: bool,
    }
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::start{listen, delete, filestore, filepath} => {
            if listen && filestore {
                listen_clipboard_data::listen_data(filepath);
            }
            else if delete {
                delete_clipboard_data::delete_data();
            }
            else {
                println!("No clipboard options specified");
            }
        }
    }
}
