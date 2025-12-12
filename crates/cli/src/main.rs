use clap::{Parser, Subcommand};
use rustx_core::{Interpreter, Lexer, Parser as RxParser};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "rustx")]
#[command(about = "RustX Language Interpreter", long_about = None)]
#[command(version = "0.2.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Script file to execute
    file: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start an interactive REPL
    Repl,
    /// Run a script file
    Run { file: PathBuf },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Repl) => run_repl(),
        Some(Commands::Run { file }) => run_file(&file),
        None => {
            if let Some(file) = cli.file {
                run_file(&file);
            } else {
                run_repl();
            }
        }
    }
}

/// Runs the REPL (Read-Eval-Print Loop)
fn run_repl() {
    println!("RustX Language REPL v0.2.0");
    println!("Type 'exit' or press Ctrl+C to quit\n");

    let mut interpreter = Interpreter::new();
    let mut buffer = String::new();

    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() {
            break;
        }

        let line = line.trim();
        if line == "exit" || line == "quit" {
            break;
        }

        if line.is_empty() {
            continue;
        }

        buffer.push_str(line);
        buffer.push('\n');

        // Try to execute the buffer
        match execute(&buffer, &mut interpreter) {
            Ok(value) => {
                println!("{}", value);
                buffer.clear();
            }
            Err(e) => {
                // If it's a parsing error, might be incomplete input
                if e.contains("Unexpected token: Eof") {
                    print!("... ");
                    io::stdout().flush().unwrap();
                    continue;
                } else {
                    eprintln!("Error: {}", e);
                    buffer.clear();
                }
            }
        }
    }

    println!("\nGoodbye!");
}

/// Runs a script file
fn run_file(path: &PathBuf) {
    let source = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };

    let mut interpreter = Interpreter::new();
    match execute(&source, &mut interpreter) {
        Ok(value) => {
            if value != rustx_core::Value::Null {
                println!("{}", value);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

/// Executes source code
fn execute(source: &str, interpreter: &mut Interpreter) -> Result<rustx_core::Value, String> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;
    let mut parser = RxParser::new(tokens);
    let ast = parser.parse()?;
    interpreter.eval_program(ast)
}
