use crate::ast::{BinaryOperator, Node, Program, VarType};
use crate::error::{Error, Result};
use crate::runtime::{Environment, Value};

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, program: Program) -> Result<Value> {
        let mut result = Value::Null;

        for statement in program.statements {
            result = self.interpret_node(statement)?;
        }

        Ok(result)
    }

    fn interpret_node(&mut self, node: Node) -> Result<Value> {
        match node {
            Node::VariableDecl { name, value, .. } => {
                let val = self.interpret_node(*value)?;
                self.environment.define(name, val);
                Ok(Value::Null)
            }

            Node::ExpressionStmt(expr) => self.interpret_node(*expr),

            Node::FunctionCall { name, arguments } => self.call_function(&name, arguments),

            Node::String(s) => Ok(Value::String(s)),
            Node::Integer(i) => Ok(Value::Integer(i)),
            Node::Float(f) => Ok(Value::Float(f)),
            Node::Boolean(b) => Ok(Value::Boolean(b)),
            Node::Identifier(name) => match self.environment.get(&name) {
                Some(value) => Ok(value),
                None => Err(Error::RuntimeError(format!(
                    "Variable '{}' not found",
                    name
                ))),
            },

            Node::BinaryOp {
                left,
                operator,
                right,
            } => {
                let left_val = self.interpret_node(*left)?;
                let right_val = self.interpret_node(*right)?;

                match operator {
                    BinaryOperator::Add => self.add_values(left_val, right_val),
                    BinaryOperator::Subtract => self.subtract_values(left_val, right_val),
                    BinaryOperator::Multiply => self.multiply_values(left_val, right_val),
                    BinaryOperator::Divide => self.divide_values(left_val, right_val),
                    BinaryOperator::Equals => Ok(Value::Boolean(left_val == right_val)),
                    BinaryOperator::NotEquals => Ok(Value::Boolean(left_val != right_val)),
                }
            }

            Node::FunctionDecl {
                name,
                parameters,
                return_type: _,
                body,
            } => {
                // Store function definition for later use
                let func_value = Value::Function {
                    name,
                    params: parameters,
                    body,
                };
                Ok(func_value)
            }

            Node::Return { value } => {
                if let Some(val) = value {
                    self.interpret_node(*val)
                } else {
                    Ok(Value::Null)
                }
            }

            Node::Null => Ok(Value::Null),

            _ => Err(Error::RuntimeError("Unsupported node type".to_string())),
        }
    }

    fn call_function(&mut self, name: &str, arguments: Vec<(String, Node)>) -> Result<Value> {
        match name {
            "print" => {
                if let Some((_, arg)) = arguments.first() {
                    let value = self.interpret_node(arg.clone())?;
                    print!("{}", value);
                    Ok(Value::Null)
                } else {
                    Ok(Value::Null)
                }
            }
            "println" => {
                if let Some((_, arg)) = arguments.first() {
                    let value = self.interpret_node(arg.clone())?;
                    println!("{}", value);
                    Ok(Value::Null)
                } else {
                    println!();
                    Ok(Value::Null)
                }
            }
            "printf" => {
                if let Some((format_arg, _)) = arguments.first() {
                    if format_arg == "format" || format_arg.is_empty() {
                        if let Some((_, format_node)) = arguments.first() {
                            let format_str = self.interpret_node(format_node.clone())?;
                            if let Value::String(format_string) = format_str {
                                let mut args = Vec::new();
                                for (_, arg) in &arguments[1..] {
                                    args.push(self.interpret_node(arg.clone())?);
                                }
                                let result = self.format_string(&format_string, &args)?;
                                print!("{}", result);
                                Ok(Value::Null)
                            } else {
                                Err(Error::RuntimeError(
                                    "printf requires a string format".to_string(),
                                ))
                            }
                        } else {
                            Ok(Value::Null)
                        }
                    } else {
                        // Handle named arguments like printf("Hello {name}")
                        let format_str = format_arg.clone();
                        let mut values = Vec::new();
                        for (_, arg) in &arguments {
                            values.push(self.interpret_node(arg.clone())?);
                        }
                        let result = self.interpolate_string(&format_str, &values)?;
                        print!("{}", result);
                        Ok(Value::Null)
                    }
                } else {
                    Ok(Value::Null)
                }
            }
            _ => {
                // Check if it's a user-defined function
                // For now, return null for user-defined functions
                Ok(Value::Null)
            }
        }
    }

    fn add_values(&self, left: Value, right: Value) -> Result<Value> {
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

    fn subtract_values(&self, left: Value, right: Value) -> Result<Value> {
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

    fn multiply_values(&self, left: Value, right: Value) -> Result<Value> {
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

    fn divide_values(&self, left: Value, right: Value) -> Result<Value> {
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

    fn format_string(&self, format: &str, args: &[Value]) -> Result<String> {
        let mut result = String::new();
        let mut arg_index = 0;
        let mut chars = format.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '{' {
                if let Some(_ch2) = chars.next() {
                    if let Some(arg) = args.get(arg_index) {
                        result.push_str(&arg.to_string());
                        arg_index += 1;
                    }
                }
            } else {
                result.push(ch);
            }
        }

        Ok(result)
    }

    fn interpolate_string(&self, template: &str, values: &[Value]) -> Result<String> {
        let mut result = String::new();
        let mut chars = template.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '{' {
                if let Some(_ch2) = chars.next() {
                    // For now, just use the first value or empty string
                    if let Some(value) = values.first() {
                        result.push_str(&value.to_string());
                    }
                }
            } else {
                result.push(ch);
            }
        }

        Ok(result)
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}
