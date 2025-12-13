use clap::{Parser, Subcommand};
use colored::Colorize;
use rustx_core::{Interpreter, Lexer, Parser as RxParser, Value};
use rustx_core::compiler::transpiler::Transpiler;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::fs;
use std::path::PathBuf;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use std::process::Command as SysCommand;

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
    /// Compile a script to an executable
    Build {
        file: PathBuf,
        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,
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
        Some(Commands::Build { file, output }) => build_file(&file, output),
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

/// Compiles a script file
fn build_file(path: &PathBuf, output: Option<PathBuf>) {
    let start_time = Instant::now();
    println!("{} {}", "Compiling:".bright_blue().bold(), path.display());

    let source = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{} {}", "Error reading file:".red().bold(), e);
            std::process::exit(1);
        }
    };

    // 1. Transpile to Rust
    let mut lexer = Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
             eprintln!("{} {}", "Lexer Error:".red().bold(), e);
             std::process::exit(1);
        }
    };
    
    let mut parser = RxParser::new(tokens);
    let ast = match parser.parse() {
        Ok(a) => a,
        Err(e) => {
             eprintln!("{} {}", "Parser Error:".red().bold(), e);
             std::process::exit(1);
        }
    };
    
    let mut transpiler = Transpiler::new();
    let rust_code = transpiler.transpile(&ast);
    
    // 2. Setup Build Environment
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let build_dir = std::env::temp_dir().join(format!("rustx_build_{}", timestamp));
    
    if let Err(e) = fs::create_dir_all(&build_dir) {
         eprintln!("{} {}", "Error creating temp dir:".red().bold(), e);
         std::process::exit(1);
    }
    
    println!("{} {}", "Build setup:".dimmed(), build_dir.display());
    
    // Initialize cargo project
    let status = SysCommand::new("cargo")
        .arg("init")
        .arg("--bin")
        .arg("--name")
        .arg("app")
        .current_dir(&build_dir)
        .output();
        
    if let Err(e) = status {
         eprintln!("{} {}", "Error initializing cargo:".red().bold(), e);
         std::process::exit(1);
    }
    
    // Add dependency
    // Hardcoded absolute path to core
    let core_path = "/home/grandpa/me/code/rust/RustX/crates/core";
    let cargo_toml_path = build_dir.join("Cargo.toml");
    let mut cargo_toml = fs::read_to_string(&cargo_toml_path).unwrap();
    cargo_toml.push_str(&format!("\nrustx_core = {{ path = \"{}\" }}\n", core_path));
    fs::write(&cargo_toml_path, cargo_toml).unwrap();
    
    // Write source
    let main_rs_path = build_dir.join("src/main.rs");
    if let Err(e) = fs::write(&main_rs_path, rust_code) {
         eprintln!("{} {}", "Error writing source:".red().bold(), e);
         std::process::exit(1);
    }
    
    // 3. Compile
    println!("{}", "Compiling binary...".bright_yellow());
    let compile_status = SysCommand::new("cargo")
        .arg("build")
        .arg("--release")
        .current_dir(&build_dir)
        .output();
        
    match compile_status {
        Ok(res) => {
            if !res.status.success() {
                eprintln!("{}", "Compilation failed:".red().bold());
                eprintln!("{}", String::from_utf8_lossy(&res.stderr));
                std::process::exit(1);
            }
        }
        Err(e) => {
             eprintln!("{} {}", "Failed to run cargo build:".red().bold(), e);
             std::process::exit(1);
        }
    }
    
    // 4. Move Output
    let binary = build_dir.join("target/release/app");
    let output_path = output.unwrap_or_else(|| {
        let file_stem = path.file_stem().unwrap().to_str().unwrap();
        PathBuf::from(file_stem)
    });
    
    if let Err(e) = fs::copy(&binary, &output_path) {
         eprintln!("{} {}", "Error moving binary:".red().bold(), e);
         std::process::exit(1);
    }
    
    println!("{} {} ({:.2}s)", "Successfully built:".bright_green().bold(), output_path.display(), start_time.elapsed().as_secs_f32());
    
    // Cleanup (optional, maybe keep for debugging?)
    // fs::remove_dir_all(&build_dir).ok();
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
