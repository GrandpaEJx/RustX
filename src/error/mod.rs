use std::fmt;

#[derive(Debug, Clone)]
pub enum Error {
    ParseError { message: String, line: usize, column: usize },
    RuntimeError(String),
    CompilerError(String),
    LexerError { message: String, line: usize, column: usize },
    Incomplete { line: usize, column: usize },
    UnexpectedEOF { line: usize, column: usize },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ParseError { message, line, column } => write!(f, "Parse Error at line {}, column {}: {}", line, column, message),
            Error::RuntimeError(msg) => write!(f, "Runtime Error: {}", msg),
            Error::CompilerError(msg) => write!(f, "Compiler Error: {}", msg),
            Error::LexerError { message, line, column } => write!(f, "Lexer Error at line {}, column {}: {}", line, column, message),
            Error::Incomplete { line, column } => write!(f, "Incomplete code at line {}, column {}", line, column),
            Error::UnexpectedEOF { line, column } => write!(f, "Unexpected end of file at line {}, column {}", line, column),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
