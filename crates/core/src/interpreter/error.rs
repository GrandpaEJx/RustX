use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeError {
    TypeMismatch { expected: String, found: String },
    UndefinedVariable(String),
    UnknownMethod(String),
    ArgumentError(String),
    IOError(String),
    ImportError(String),
    FeatureNotSupported(String),
    Generic(String),
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RuntimeError::TypeMismatch { expected, found } => {
                write!(f, "TypeMismatch: Expected {}, but found {}", expected, found)
            }
            RuntimeError::UndefinedVariable(name) => {
                write!(f, "UndefinedVariable: Variable '{}' is not defined", name)
            }
            RuntimeError::UnknownMethod(name) => {
                write!(f, "UnknownMethod: Method '{}' not found", name)
            }
            RuntimeError::ArgumentError(msg) => {
                write!(f, "ArgumentError: {}", msg)
            }
            RuntimeError::IOError(msg) => {
                write!(f, "IOError: {}", msg)
            }
            RuntimeError::ImportError(msg) => {
                write!(f, "ImportError: {}", msg)
            }
            RuntimeError::Generic(msg) => {
                write!(f, "RuntimeError: {}", msg)
            }
            RuntimeError::FeatureNotSupported(msg) => {
                write!(f, "FeatureNotSupported: {}", msg)
            }
        }
    }
}

impl From<String> for RuntimeError {
    fn from(msg: String) -> Self {
        RuntimeError::Generic(msg)
    }
}

impl From<&str> for RuntimeError {
    fn from(msg: &str) -> Self {
        RuntimeError::Generic(msg.to_string())
    }
}
