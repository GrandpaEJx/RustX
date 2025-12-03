use crate::error::{Error, Result};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
    Function {
        name: String,
        params: Vec<(String, crate::ast::VarType)>,
        body: Vec<crate::ast::Node>,
    },
}

impl Value {
    pub fn as_string(&self) -> Result<String> {
        match self {
            Value::String(s) => Ok(s.clone()),
            Value::Integer(i) => Ok(i.to_string()),
            Value::Float(f) => Ok(f.to_string()),
            Value::Boolean(b) => Ok(b.to_string()),
            Value::Null => Ok("null".to_string()),
            Value::Function { .. } => Ok("<function>".to_string()),
        }
    }

    pub fn as_number(&self) -> Result<f64> {
        match self {
            Value::Integer(i) => Ok(*i as f64),
            Value::Float(f) => Ok(*f),
            Value::String(s) => s.parse::<f64>().map_err(|_| {
                Error::RuntimeError(format!("Cannot convert string '{}' to number", s))
            }),
            Value::Boolean(b) => Ok(if *b { 1.0 } else { 0.0 }),
            Value::Null => Ok(0.0),
            Value::Function { .. } => Err(Error::RuntimeError(
                "Cannot convert function to number".to_string(),
            )),
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Integer(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Null => false,
            Value::Function { .. } => true,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Null, Value::Null) => true,
            _ => false, // Functions are never equal to anything
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(s) => write!(f, "{}", s),
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),
            Value::Function { name, .. } => write!(f, "<function {}>", name),
        }
    }
}
