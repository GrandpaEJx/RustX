use crate::error::{Error, Result};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
}

impl Value {
    pub fn as_string(&self) -> Result<String> {
        match self {
            Value::String(s) => Ok(s.clone()),
            Value::Integer(i) => Ok(i.to_string()),
            Value::Float(f) => Ok(f.to_string()),
            Value::Boolean(b) => Ok(b.to_string()),
            Value::Null => Ok("null".to_string()),
        }
    }
    
    pub fn as_number(&self) -> Result<f64> {
        match self {
            Value::Integer(i) => Ok(*i as f64),
            Value::Float(f) => Ok(*f),
            Value::String(s) => s.parse::<f64>().map_err(|_| 
                Error::RuntimeError(format!("Cannot convert string '{}' to number", s))),
            Value::Boolean(b) => Ok(if *b { 1.0 } else { 0.0 }),
            Value::Null => Ok(0.0),
        }
    }
    
    pub fn as_bool(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Integer(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Null => false,
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
        }
    }
}

pub struct Environment {
    variables: HashMap<String, Value>,
    parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            variables: HashMap::new(),
            parent: None,
        }
    }
    
    pub fn with_parent(parent: Box<Environment>) -> Self {
        Environment {
            variables: HashMap::new(),
            parent: Some(parent),
        }
    }
    
    pub fn define(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }
    
    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(value) = self.variables.get(name) {
            Some(value.clone())
        } else if let Some(parent) = &self.parent {
            parent.get(name)
        } else {
            None
        }
    }
    
    pub fn assign(&mut self, name: String, value: Value) {
        if self.variables.contains_key(&name) {
            self.variables.insert(name, value);
        } else if let Some(parent) = &mut self.parent {
            parent.assign(name, value);
        } else {
            self.variables.insert(name, value);
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.variables.is_empty() && self.parent.is_none()
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}