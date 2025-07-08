use clap::{Parser, Subcommand};
use rslox::Interpreter;
use rslox::ParseResult;
use std::{path::PathBuf, process::ExitCode};
#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Tokenize { filename: PathBuf },
    Parse { filename: PathBuf },
    Evaluate { filename: PathBuf },
}

fn main() -> ExitCode {
    let args = Args::parse();
    let mut exit_code = ExitCode::from(0);
    match args.command {
        Command::Tokenize { filename } => {
            let content = std::fs::read_to_string(&filename).expect("Failed to read the file");
            let mut lexer = rslox::Lexer::new(content.as_str());
            while let Some(token) = lexer.next() {
                match token {
                    Ok(t) => println!("{}", t),
                    Err(e) => {
                        exit_code = ExitCode::from(65);
                        eprintln!("{:?}", miette::Report::new(e))
                    }
                }
            }
            println!("EOF  null");
        }
        Command::Parse { filename } => {
            let content = std::fs::read_to_string(&filename).expect("Failed to read the file");
            let mut parser = rslox::Parser::new(content.as_str());
            let ParseResult { tree, errors } = parser.parse();

            if !errors.is_empty() {
                exit_code = ExitCode::from(65);
                for error in errors {
                    eprintln!("{:?}", miette::Report::new(error));
                }
            }

            if !tree.0.is_empty() {
                println!("{}", tree);
            }
        }
        Command::Evaluate { filename } => {
            let content = std::fs::read_to_string(&filename).expect("Failed to read the file");
            let mut interpreter = Interpreter::new(content.as_str());
            match interpreter.interpret() {
                Ok(_) => {}
                Err(e) => {
                    exit_code = ExitCode::from(70);
                    eprintln!("{:?}", miette::Report::new(e));
                }
            }
        }
    }

    return exit_code;
}
