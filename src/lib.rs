// RustX - A minimal scripting language
pub mod error;
pub mod lexer;
pub mod parser;
pub mod interpreter;
pub mod builtins;
pub mod ast;
pub mod runtime;

pub use error::{Error, Result};
pub use lexer::Lexer;
pub use parser::Parser;
pub use interpreter::Interpreter;
pub use ast::{Node, Program};
pub use runtime::{Value, Environment};

pub fn run_file(path: &str) -> Result<()> {
    let code = std::fs::read_to_string(path).map_err(|e| Error::RuntimeError(e.to_string()))?;
    let mut interpreter = Interpreter::new();
    interpreter.run(&code)
}

pub fn run_repl() -> Result<()> {
    use std::io::{self, Write};
    
    println!("RustX REPL - Type 'exit' to quit");
    
    let mut interpreter = Interpreter::new();
    let mut buffer = String::new();
    
    loop {
        print!("rustx> ");
        io::stdout().flush().map_err(|e| Error::RuntimeError(e.to_string()))?;
        
        let mut line = String::new();
        io::stdin().read_line(&mut line).map_err(|e| Error::RuntimeError(e.to_string()))?;
        
        let line = line.trim();
        
        if line == "exit" || line == "quit" {
            break;
        }
        
        if line.is_empty() {
            continue;
        }
        
        buffer.push_str(line);
        buffer.push('\n');
        
        // Try to parse and execute
        match interpreter.run(&buffer) {
            Ok(_) => {
                buffer.clear();
            }
            Err(Error::Incomplete) => {
                // Continue reading
                continue;
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                buffer.clear();
            }
        }
    }
    
    Ok(())
}