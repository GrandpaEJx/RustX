use crate::error::{Error, Result};
use crate::runtime::Value;
use super::Interpreter;

impl Interpreter {
    pub fn add_values(&self, left: Value, right: Value) -> Result<Value> {
        match (left, right) {
            (Value::Integer(l), Value::Integer(r)) => Ok(Value::Integer(l + r)),
            (Value::Float(l), Value::Float(r)) => Ok(Value::Float(l + r)),
            (Value::Integer(l), Value::Float(r)) => Ok(Value::Float(l as f64 + r)),
            (Value::Float(l), Value::Integer(r)) => Ok(Value::Float(l + r as f64)),
            (Value::String(l), Value::String(r)) => Ok(Value::String(l + &r)),
            (Value::String(l), Value::Integer(r)) => Ok(Value::String(l + &r.to_string())),
            (Value::String(l), Value::Float(r)) => Ok(Value::String(l + &r.to_string())),
            (Value::Integer(l), Value::String(r)) => Ok(Value::String(l.to_string() + &r)),
            (Value::Float(l), Value::String(r)) => Ok(Value::String(l.to_string() + &r)),
            _ => Err(Error::RuntimeError("Cannot add these types".to_string())),
        }
    }

    pub fn subtract_values(&self, left: Value, right: Value) -> Result<Value> {
        match (left, right) {
            (Value::Integer(l), Value::Integer(r)) => Ok(Value::Integer(l - r)),
            (Value::Float(l), Value::Float(r)) => Ok(Value::Float(l - r)),
            (Value::Integer(l), Value::Float(r)) => Ok(Value::Float(l as f64 - r)),
            (Value::Float(l), Value::Integer(r)) => Ok(Value::Float(l - r as f64)),
            _ => Err(Error::RuntimeError(
                "Cannot subtract these types".to_string(),
            )),
        }
    }

    pub fn multiply_values(&self, left: Value, right: Value) -> Result<Value> {
        match (left, right) {
            (Value::Integer(l), Value::Integer(r)) => Ok(Value::Integer(l * r)),
            (Value::Float(l), Value::Float(r)) => Ok(Value::Float(l * r)),
            (Value::Integer(l), Value::Float(r)) => Ok(Value::Float(l as f64 * r)),
            (Value::Float(l), Value::Integer(r)) => Ok(Value::Float(l * r as f64)),
            _ => Err(Error::RuntimeError(
                "Cannot multiply these types".to_string(),
            )),
        }
    }

    pub fn divide_values(&self, left: Value, right: Value) -> Result<Value> {
        match (left, right) {
            (Value::Integer(l), Value::Integer(r)) => {
                if r == 0 {
                    Err(Error::RuntimeError("Division by zero".to_string()))
                } else {
                    Ok(Value::Integer(l / r))
                }
            }
            (Value::Float(l), Value::Float(r)) => {
                if r == 0.0 {
                    Err(Error::RuntimeError("Division by zero".to_string()))
                } else {
                    Ok(Value::Float(l / r))
                }
            }
            (Value::Integer(l), Value::Float(r)) => {
                if r == 0.0 {
                    Err(Error::RuntimeError("Division by zero".to_string()))
                } else {
                    Ok(Value::Float(l as f64 / r))
                }
            }
            (Value::Float(l), Value::Integer(r)) => {
                if r == 0 {
                    Err(Error::RuntimeError("Division by zero".to_string()))
                } else {
                    Ok(Value::Float(l / r as f64))
                }
            }
            _ => Err(Error::RuntimeError("Cannot divide these types".to_string())),
        }
    }
}