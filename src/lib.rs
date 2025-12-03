pub mod ast;
pub mod builtins;
pub mod compiler;
pub mod error;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod runtime;

pub use ast::{Node, Program};
pub use compiler::Compiler;
pub use error::{Error, Result};
pub use interpreter::Interpreter;
pub use lexer::Lexer;
pub use parser::Parser;
pub use runtime::{Environment, Value};

pub fn compile_file(path: &str) -> Result<String> {
    let code = std::fs::read_to_string(path).map_err(|e| Error::RuntimeError(e.to_string()))?;
    let mut parser = Parser::new(code);
    let program = parser.parse()?;
    let mut compiler = Compiler::new();
    compiler.compile(program)
}

pub fn run_file(path: &str) -> Result<()> {
    let code = std::fs::read_to_string(path).map_err(|e| Error::RuntimeError(e.to_string()))?;
    let mut parser = Parser::new(code);
    let program = parser.parse()?;
    let mut interpreter = Interpreter::new();
    interpreter.interpret(program)?;
    Ok(())
}

pub fn convert_to_rs(path: &str) -> Result<String> {
    compile_file(path)
}
