use clap::{Parser, Subcommand};
use colored::Colorize;
use rustx_core::{Interpreter, Lexer, Parser as RxParser, Value};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Parser)]
#[command(name = "rustx")]
#[command(about = "RustX Language Interpreter", long_about = None)]
#[command(version = "0.2.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Script file to execute
    file: Option<PathBuf>,

    /// Show AST (Abstract Syntax Tree)
    #[arg(long)]
    ast: bool,

    /// Show tokens
    #[arg(long)]
    tokens: bool,

    /// Show execution time
    #[arg(long)]
    time: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Start an interactive REPL
    Repl,
    /// Run a script file
    Run {
        file: PathBuf,
        /// Show AST
        #[arg(long)]
        ast: bool,
        /// Show tokens
        #[arg(long)]
        tokens: bool,
        /// Show execution time
        #[arg(long)]
        time: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Repl) => run_repl(),
        Some(Commands::Run {
            file,
            ast,
            tokens,
            time,
        }) => run_file(&file, ast, tokens, time, false),
        None => {
            if let Some(file) = cli.file {
                run_file(&file, cli.ast, cli.tokens, cli.time, cli.verbose);
            } else {
                run_repl();
            }
        }
    }
}

/// Runs the REPL (Read-Eval-Print Loop)
fn run_repl() {
    println!("{}", "RustX Language REPL v0.2.0".bright_cyan().bold());
    println!(
        "{}",
        "Type ':help' for commands, ':exit' or Ctrl+D to quit\n"
            .bright_black()
    );

    let mut rl = match DefaultEditor::new() {
        Ok(editor) => editor,
        Err(e) => {
            eprintln!("{} {}", "Error initializing REPL:".red().bold(), e);
            return;
        }
    };

    let mut interpreter = Interpreter::new();

    loop {
        let readline = rl.readline(">>> ");
        match readline {
            Ok(line) => {
                let line = line.trim();

                // Handle special commands
                if line.starts_with(':') {
                    match line {
                        ":exit" | ":quit" | ":q" => break,
                        ":help" | ":h" => {
                            print_help();
                            continue;
                        }
                        ":clear" | ":c" => {
                            print!("\x1B[2J\x1B[1;1H"); // Clear screen
                            continue;
                        }
                        ":vars" | ":v" => {
                            println!("{}", "Environment variables:".bright_yellow());
                            println!("{}", "(Variable inspection not yet implemented)".dimmed());
                            continue;
                        }
                        _ => {
                            eprintln!(
                                "{} {}",
                                "Unknown command:".red().bold(),
                                line.bright_white()
                            );
                            println!("Type {} for available commands", ":help".bright_cyan());
                            continue;
                        }
                    }
                }

                if line.is_empty() {
                    continue;
                }

                // Add to history
                let _ = rl.add_history_entry(line);

                // Execute the line
                match execute(line, &mut interpreter, false, false) {
                    Ok(value) => {
                        if value != Value::Null {
                            println!("{}", format!("{}", value).bright_green());
                        }
                    }
                    Err(e) => {
                        eprintln!("{} {}", "Error:".red().bold(), e.bright_white());
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("{}", "^C".dimmed());
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("{}", "^D".dimmed());
                break;
            }
            Err(err) => {
                eprintln!("{} {:?}", "Error:".red().bold(), err);
                break;
            }
        }
    }

    println!("\n{}", "Goodbye!".bright_cyan());
}

/// Runs a script file
fn run_file(path: &PathBuf, show_ast: bool, show_tokens: bool, show_time: bool, verbose: bool) {
    let start_time = if show_time {
        Some(Instant::now())
    } else {
        None
    };

    let source = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!(
                "{} {}",
                "Error reading file:".red().bold(),
                e.to_string().bright_white()
            );
            std::process::exit(1);
        }
    };

    if verbose {
        println!(
            "{} {}",
            "Executing:".bright_blue().bold(),
            path.display().to_string().bright_white()
        );
    }

    let mut interpreter = Interpreter::new();
    match execute(&source, &mut interpreter, show_ast, show_tokens) {
        Ok(value) => {
            if value != Value::Null {
                println!("{}", value);
            }

            if let Some(start) = start_time {
                let duration = start.elapsed();
                println!(
                    "\n{} {:.3}ms",
                    "Execution time:".bright_black(),
                    duration.as_secs_f64() * 1000.0
                );
            }
        }
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e.bright_white());
            std::process::exit(1);
        }
    }
}

/// Executes source code
fn execute(
    source: &str,
    interpreter: &mut Interpreter,
    show_ast: bool,
    show_tokens: bool,
) -> Result<Value, String> {
    // Tokenize
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;

    if show_tokens {
        println!("{}", "=== Tokens ===".bright_yellow().bold());
        for (i, token) in tokens.iter().enumerate() {
            println!("{:3}: {:?}", i, token);
        }
        println!();
    }

    // Parse
    let mut parser = RxParser::new(tokens);
    let ast = parser.parse()?;

    if show_ast {
        println!("{}", "=== AST ===".bright_yellow().bold());
        for (i, stmt) in ast.iter().enumerate() {
            println!("{:3}: {:#?}", i, stmt);
        }
        println!();
    }

    // Interpret
    interpreter.eval_program(ast)
}

/// Prints help information
fn print_help() {
    println!("{}", "RustX REPL Commands:".bright_cyan().bold());
    println!("  {}  - Show this help message", ":help, :h".bright_white());
    println!("  {}  - Exit the REPL", ":exit, :quit, :q".bright_white());
    println!("  {}  - Clear the screen", ":clear, :c".bright_white());
    println!(
        "  {}  - Show environment variables (not implemented)",
        ":vars, :v".bright_white()
    );
    println!("\n{}", "Keyboard Shortcuts:".bright_cyan().bold());
    println!("  {}  - Exit", "Ctrl+D".bright_white());
    println!("  {}  - Interrupt current input", "Ctrl+C".bright_white());
    println!("  {}  - Navigate command history", "Up/Down arrows".bright_white());
}
