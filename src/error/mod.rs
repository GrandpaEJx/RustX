use std::fmt;

#[derive(Debug, Clone)]
pub enum Error {
    ParseError(String),
    RuntimeError(String),
    CompilerError(String),
    LexerError(String),
    Incomplete,
    UnexpectedEOF,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ParseError(msg) => write!(f, "Parse Error: {}", msg),
            Error::RuntimeError(msg) => write!(f, "Runtime Error: {}", msg),
            Error::CompilerError(msg) => write!(f, "Compiler Error: {}", msg),
            Error::LexerError(msg) => write!(f, "Lexer Error: {}", msg),
            Error::Incomplete => write!(f, "Incomplete code"),
            Error::UnexpectedEOF => write!(f, "Unexpected end of file"),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
