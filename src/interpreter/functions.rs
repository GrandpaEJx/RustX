use crate::ast::Node;
use crate::error::Result;
use crate::runtime::Value;
use super::Interpreter;

impl Interpreter {
    pub fn call_function(&mut self, name: &str, arguments: Vec<(String, Node)>) -> Result<Value> {
        // First, interpret all arguments to values
        let mut args_values = Vec::new();
        for (_, arg) in &arguments {
            args_values.push(self.interpret_node(arg.clone())?);
        }

        // Check if it's an external function
        if let Some(func) = self.external_functions.get(name) {
            return func(args_values);
        }

        // Otherwise, handle built-in functions
        match name {
            "print" => {
                if let Some(value) = args_values.first() {
                    print!("{}", value);
                }
                Ok(Value::Null)
            }
            "println" => {
                if let Some(value) = args_values.first() {
                    println!("{}", value);
                } else {
                    println!();
                }
                Ok(Value::Null)
            }
            "printf" => {
                if let Some(format_value) = args_values.first() {
                    if let Value::String(format_string) = format_value {
                        let result = self.format_string(&format_string, &args_values[1..])?;
                        print!("{}", result);
                    }
                }
                Ok(Value::Null)
            }
            _ => {
                // Check if it's a user-defined function
                // For now, return null for user-defined functions
                Ok(Value::Null)
            }
        }
    }
}