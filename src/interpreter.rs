use crate::error::{Error, Result};
use crate::ast::{Node, Program, VarType, BinaryOperator};
use crate::runtime::{Value, Environment};

pub struct Interpreter {
    pub environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new(),
        }
    }
    
    pub fn run(&mut self, code: &str) -> Result<()> {
        let mut parser = crate::parser::Parser::new(code.to_string());
        let program = parser.parse()?;
        self.interpret_program(&program)
    }
    
    pub fn interpret_program(&mut self, program: &Program) -> Result<()> {
        for statement in &program.statements {
            self.interpret_statement(statement)?;
        }
        Ok(())
    }
    
    fn interpret_statement(&mut self, statement: &Node) -> Result<()> {
        match statement {
            Node::VariableDecl { var_type, name, value } => {
                let evaluated_value = self.interpret_expression(value)?;
                let converted_value = self.convert_value(evaluated_value, var_type)?;
                self.environment.define(name.clone(), converted_value);
                Ok(())
            }
            Node::FunctionCall { name, arguments } => {
                self.call_function(name, arguments)
            }
            Node::ExpressionStmt(expr) => {
                self.interpret_expression(expr)?;
                Ok(())
            }
            _ => Err(Error::RuntimeError(format!("Unexpected statement: {:?}", statement))),
        }
    }
    
    fn interpret_expression(&self, expression: &Node) -> Result<Value> {
        match expression {
            Node::String(s) => Ok(Value::String(s.clone())),
            Node::Integer(i) => Ok(Value::Integer(*i)),
            Node::Float(f) => Ok(Value::Float(*f)),
            Node::Boolean(b) => Ok(Value::Boolean(*b)),
            Node::Identifier(name) => {
                if let Some(value) = self.environment.get(name) {
                    Ok(value)
                } else {
                    Err(Error::RuntimeError(format!("Undefined variable: {}", name)))
                }
            }
            Node::BinaryOp { left, operator, right } => {
                let left_value = self.interpret_expression(left)?;
                let right_value = self.interpret_expression(right)?;
                self.execute_binary_operation(left_value, operator.clone(), right_value)
            }
            _ => Err(Error::RuntimeError(format!("Cannot evaluate expression: {:?}", expression))),
        }
    }
    
    fn execute_binary_operation(&self, left: Value, operator: BinaryOperator, right: Value) -> Result<Value> {
        match operator {
            BinaryOperator::Add => {
                let left_num = left.as_number()?;
                let right_num = right.as_number()?;
                Ok(Value::Float(left_num + right_num))
            }
            BinaryOperator::Subtract => {
                let left_num = left.as_number()?;
                let right_num = right.as_number()?;
                Ok(Value::Float(left_num - right_num))
            }
            BinaryOperator::Multiply => {
                let left_num = left.as_number()?;
                let right_num = right.as_number()?;
                Ok(Value::Float(left_num * right_num))
            }
            BinaryOperator::Divide => {
                let left_num = left.as_number()?;
                let right_num = right.as_number()?;
                if right_num == 0.0 {
                    return Err(Error::RuntimeError("Division by zero".to_string()));
                }
                Ok(Value::Float(left_num / right_num))
            }
            BinaryOperator::Equals => {
                let left_num = left.as_number()?;
                let right_num = right.as_number()?;
                Ok(Value::Boolean((left_num - right_num).abs() < f64::EPSILON))
            }
            BinaryOperator::NotEquals => {
                let left_num = left.as_number()?;
                let right_num = right.as_number()?;
                Ok(Value::Boolean((left_num - right_num).abs() >= f64::EPSILON))
            }
        }
    }
    
    fn convert_value(&self, value: Value, var_type: &VarType) -> Result<Value> {
        match var_type {
            VarType::Str => Ok(Value::String(value.as_string()?)),
            VarType::Int => {
                let num = value.as_number()?;
                Ok(Value::Integer(num as i64))
            }
            VarType::Float => {
                let num = value.as_number()?;
                Ok(Value::Float(num))
            }
            VarType::Bool => Ok(Value::Boolean(value.as_bool())),
            VarType::Auto => Ok(value),
        }
    }
    
    fn call_function(&self, name: &str, arguments: &[Node]) -> Result<()> {
        let mut evaluated_args = Vec::new();
        
        for arg in arguments {
            let value = self.interpret_expression(arg)?;
            evaluated_args.push(value);
        }
        
        self.execute_builtin_function(name, &evaluated_args)
    }
    
    fn execute_builtin_function(&self, name: &str, arguments: &[Value]) -> Result<()> {
        match name {
            "print" => {
                for (i, arg) in arguments.iter().enumerate() {
                    if i > 0 { print!(" "); }
                    print!("{}", arg);
                }
                Ok(())
            }
            "println" => {
                for arg in arguments {
                    println!("{}", arg);
                }
                Ok(())
            }
            "printf" => {
                if arguments.is_empty() {
                    return Err(Error::RuntimeError("printf requires format string".to_string()));
                }
                
                let format_str = arguments[0].as_string()?;
                
                // Handle string interpolation with variable names and numbered placeholders
                let mut result = format_str;
                
                // First handle numbered placeholders like {0}, {1}, etc.
                for (i, arg) in arguments.iter().enumerate().skip(1) {
                    let placeholder = format!("{{{}}}", i - 1);
                    result = result.replace(&placeholder, &arg.to_string());
                }
                
                // Then handle variable interpolation like {variable_name}
                // This is a simple implementation - in a real language you'd have more sophisticated parsing
                let mut i = 0;
                while i < result.len() {
                    if result.chars().nth(i) == Some('{') {
                        // Find the closing brace
                        if let Some(closing_pos) = result[i..].find('}') {
                            let var_name = &result[i+1..i+closing_pos];
                            
                            // Try to get the variable from environment
                            if let Some(value) = self.environment.get(var_name) {
                                let replacement = value.to_string();
                                result = result.replace(&format!("{{{}}}", var_name), &replacement);
                                i += replacement.len(); // Skip past the replacement
                            } else {
                                // Variable not found, leave it as is or replace with empty string
                                result = result.replace(&format!("{{{}}}", var_name), "");
                                i += closing_pos;
                            }
                        } else {
                            i += 1;
                        }
                    } else {
                        i += 1;
                    }
                }
                
                print!("{}", result);
                Ok(())
            }
            _ => Err(Error::RuntimeError(format!("Unknown function: {}", name))),
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}