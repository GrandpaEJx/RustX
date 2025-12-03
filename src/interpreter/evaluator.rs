use crate::ast::{BinaryOperator, Node};
use crate::error::{Error, Result};
use crate::runtime::Value;
use super::Interpreter;

impl Interpreter {
    pub fn interpret_node(&mut self, node: Node) -> Result<Value> {
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
}