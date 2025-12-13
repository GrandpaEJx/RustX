//! Binary and unary operation evaluation for the interpreter

use crate::ast::{BinaryOp, Expr, UnaryOp};
use crate::value::Value;
use super::{Interpreter, InterpreterResult, RuntimeError};

impl Interpreter {
    /// Evaluates a binary operation
    pub(super) fn eval_binary(&mut self, left: Expr, op: BinaryOp, right: Expr) -> InterpreterResult<Value> {
        let left_val = self.eval_expr(left)?;
        let right_val = self.eval_expr(right)?;

        match op {
            BinaryOp::Add => left_val.add(&right_val).map_err(RuntimeError::from),
            BinaryOp::Sub => left_val.sub(&right_val).map_err(RuntimeError::from),
            BinaryOp::Mul => left_val.mul(&right_val).map_err(RuntimeError::from),
            BinaryOp::Div => left_val.div(&right_val).map_err(RuntimeError::from),
            BinaryOp::Mod => left_val.rem(&right_val).map_err(RuntimeError::from),
            BinaryOp::Eq => left_val.eq_op(&right_val).map_err(RuntimeError::from),
            BinaryOp::NotEq => left_val.neq_op(&right_val).map_err(RuntimeError::from),
            BinaryOp::Lt => left_val.lt(&right_val).map_err(RuntimeError::from),
            BinaryOp::Gt => left_val.gt(&right_val).map_err(RuntimeError::from),
            BinaryOp::LtEq => left_val.le(&right_val).map_err(RuntimeError::from),
            BinaryOp::GtEq => left_val.ge(&right_val).map_err(RuntimeError::from),
            BinaryOp::And => left_val.logic_and(&right_val).map_err(RuntimeError::from),
            BinaryOp::Or => left_val.logic_or(&right_val).map_err(RuntimeError::from),
        }
    }

    /// Evaluates a unary operation
    pub(super) fn eval_unary(&mut self, op: UnaryOp, expr: Expr) -> InterpreterResult<Value> {
        let val = self.eval_expr(expr)?;

        match op {
            UnaryOp::Not => val.not().map_err(RuntimeError::from),
            UnaryOp::Neg => val.neg().map_err(RuntimeError::from),
        }
    }
}
