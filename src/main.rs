use clap::Parser;
use std::process::{self, Command};

mod parser;
mod processor;
mod compiler;

use processor::FileProcessor;

#[derive(Parser)]
#[command(name = "braceless")]
#[command(about = "A preprocessor for braceless C/C++ syntax")]
struct Cli {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    if cli.args.is_empty() {
        eprintln!("Error: No arguments provided");
        process::exit(1);
    }

    let mut processor = FileProcessor::new();

    match processor.process_args(cli.args) {
        Ok(modified_args) => {
            let mut cmd = Command::new(&modified_args[0]);
            cmd.args(&modified_args[1..]);

            let result = cmd.status();

            processor.cleanup();

            match result {
                Ok(status) => {
                    if let Some(code) = status.code() {
                        process::exit(code);
                    } else {
                        process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("Error executing compiler: {}", e);
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Error processing files: {}", e);
            process::exit(1);
        }
    }
}