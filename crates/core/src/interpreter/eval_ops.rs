/// Binary and unary operation evaluation for the interpreter

use crate::ast::{BinaryOp, Expr, UnaryOp};
use crate::value::Value;
use super::Interpreter;

impl Interpreter {
    /// Evaluates a binary operation
    pub(super) fn eval_binary(&mut self, left: Expr, op: BinaryOp, right: Expr) -> Result<Value, String> {
        let left_val = self.eval_expr(left)?;
        let right_val = self.eval_expr(right)?;

        match op {
            BinaryOp::Add => match (&left_val, &right_val) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 + b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + *b as f64)),
                (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
                _ => Err("Invalid operands for +".to_string()),
            },
            BinaryOp::Sub => match (&left_val, &right_val) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 - b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a - *b as f64)),
                _ => Err("Invalid operands for -".to_string()),
            },
            BinaryOp::Mul => match (&left_val, &right_val) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 * b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a * *b as f64)),
                _ => Err("Invalid operands for *".to_string()),
            },
            BinaryOp::Div => match (&left_val, &right_val) {
                (Value::Int(a), Value::Int(b)) => {
                    if *b == 0 {
                        Err("Division by zero".to_string())
                    } else {
                        Ok(Value::Int(a / b))
                    }
                }
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 / b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a / *b as f64)),
                _ => Err("Invalid operands for /".to_string()),
            },
            BinaryOp::Mod => match (&left_val, &right_val) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a % b)),
                _ => Err("Invalid operands for %".to_string()),
            },
            BinaryOp::Eq => Ok(Value::Bool(left_val == right_val)),
            BinaryOp::NotEq => Ok(Value::Bool(left_val != right_val)),
            BinaryOp::Lt => match (&left_val, &right_val) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a < b)),
                _ => Err("Invalid operands for <".to_string()),
            },
            BinaryOp::Gt => match (&left_val, &right_val) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a > b)),
                _ => Err("Invalid operands for >".to_string()),
            },
            BinaryOp::LtEq => match (&left_val, &right_val) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a <= b)),
                _ => Err("Invalid operands for <=".to_string()),
            },
            BinaryOp::GtEq => match (&left_val, &right_val) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a >= b)),
                _ => Err("Invalid operands for >=".to_string()),
            },
            BinaryOp::And => Ok(Value::Bool(left_val.is_truthy() && right_val.is_truthy())),
            BinaryOp::Or => Ok(Value::Bool(left_val.is_truthy() || right_val.is_truthy())),
        }
    }

    /// Evaluates a unary operation
    pub(super) fn eval_unary(&mut self, op: UnaryOp, expr: Expr) -> Result<Value, String> {
        let val = self.eval_expr(expr)?;

        match op {
            UnaryOp::Not => Ok(Value::Bool(!val.is_truthy())),
            UnaryOp::Neg => match val {
                Value::Int(n) => Ok(Value::Int(-n)),
                Value::Float(f) => Ok(Value::Float(-f)),
                _ => Err("Invalid operand for negation".to_string()),
            },
        }
    }
}
