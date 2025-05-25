mod error;
mod lexer;
mod token;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Tokenize { filename: PathBuf },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Tokenize { filename } => {
            let content = std::fs::read_to_string(&filename).expect("Failed to read the file");
            let lexer = lexer::Lexer::new(content);
            for token in lexer.consume() {
                println!("{:?}", token);
            }
        }
    }
}
