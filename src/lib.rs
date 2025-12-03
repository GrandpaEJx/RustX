//! # RustX - A Minimal Scripting Language
//!
//! RustX is a lightweight, easy-to-use scripting language built in Rust with a clean,
//! modular architecture. It features simple syntax, type system, and powerful features
//! for both scripting and embedding in Rust applications.
//!
//! ## Features
//!
//! - **Simple Syntax**: Clean, readable code with optional semicolons
//! - **Type System**: Str, Int, Bool, Float with automatic conversion
//! - **Variable Declarations**: `Str name = "Rust X"`, `Int a = 10`
//! - **String Interpolation**: `printf("Hello {name}")`
//! - **Arithmetic Operations**: `+`, `-`, `*`, `/` with proper precedence
//! - **Built-in Functions**: `print()`, `println()`, `printf()`
//! - **REPL**: Interactive read-eval-print loop
//!
//! ## Example
//!
//! ```rust
//! use rustx::run_code;
//!
//! fn main() {
//!     run_code(r#"
//!         Int x = 42
//!         println(x)
//!     "#).unwrap();
//! }
//! ```

pub mod ast;
pub mod builtins;
pub mod compiler;
pub mod error;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod runtime;
pub mod transpiler;

pub use ast::{Node, Program};
pub use compiler::Compiler;
pub use error::{Error, Result};
pub use interpreter::Interpreter;
pub use lexer::Lexer;
pub use parser::Parser;
pub use runtime::{Environment, Value};
pub use transpiler::Transpiler;

/// Compiles a RustX script file to Rust code
///
/// # Arguments
/// * `path` - Path to the .rsx file to compile
///
/// # Returns
/// Returns the compiled Rust code as a String
///
/// # Errors
/// Returns an error if the file cannot be read or if compilation fails
///
/// # Example
/// ```rust,no_run
/// use rustx::compile_file;
///
/// match compile_file("script.rsx") {
///     Ok(rust_code) => println!("{}", rust_code),
///     Err(e) => println!("Error: {}", e),
/// }
/// ```
pub fn compile_file(path: &str) -> Result<String> {
    let code = std::fs::read_to_string(path).map_err(|e| Error::RuntimeError(e.to_string()))?;
    let mut parser = Parser::new(code)?;
    let program = parser.parse()?;
    let mut compiler = Compiler::new();
    compiler.compile(program)
}

/// Executes a RustX script file
///
/// # Arguments
/// * `path` - Path to the .rsx file to execute
///
/// # Returns
/// Returns `Ok(())` if execution succeeds
///
/// # Errors
/// Returns an error if the file cannot be read or if execution fails
///
/// # Example
/// ```rust,no_run
/// use rustx::run_file;
///
/// if let Err(e) = run_file("script.rsx") {
///     println!("Error: {}", e);
/// }
/// ```
pub fn run_file(path: &str) -> Result<()> {
    let code = std::fs::read_to_string(path).map_err(|e| Error::RuntimeError(e.to_string()))?;
    let mut parser = Parser::new(code)?;
    let program = parser.parse()?;
    let mut interpreter = Interpreter::new();
    interpreter.interpret(program)?;
    Ok(())
}

pub fn convert_to_rs(path: &str) -> Result<String> {
    compile_file(path)
}

/// Executes RustX code from a string
///
/// This is the main function for embedding RustX in Rust applications.
/// It parses and executes the provided code string.
///
/// # Arguments
/// * `code` - The RustX code to execute as a string
///
/// # Returns
/// Returns `Ok(())` if execution succeeds
///
/// # Errors
/// Returns an error if parsing or execution fails
///
/// # Example
/// ```rust
/// use rustx::run_code;
///
/// fn main() {
///     run_code(r#"
///         Int x = 42
///         println(x)
///     "#).unwrap();
/// }
/// ```
pub fn run_code(code: &str) -> Result<()> {
    let mut parser = Parser::new(code.to_string())?;
    let program = parser.parse()?;
    let mut interpreter = Interpreter::new();
    interpreter.interpret(program)?;
    Ok(())
}

pub fn convert_rs_to_rsx(code: &str) -> Result<String> {
    let transpiler = Transpiler::new();
    transpiler.transpile(code)
}
