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
            BinaryOp::Add => left_val.add(&right_val),
            BinaryOp::Sub => left_val.sub(&right_val),
            BinaryOp::Mul => left_val.mul(&right_val),
            BinaryOp::Div => left_val.div(&right_val),
            BinaryOp::Mod => left_val.rem(&right_val),
            BinaryOp::Eq => left_val.eq_op(&right_val),
            BinaryOp::NotEq => left_val.neq_op(&right_val),
            BinaryOp::Lt => left_val.lt(&right_val),
            BinaryOp::Gt => left_val.gt(&right_val),
            BinaryOp::LtEq => left_val.le(&right_val),
            BinaryOp::GtEq => left_val.ge(&right_val),
            BinaryOp::And => left_val.logic_and(&right_val),
            BinaryOp::Or => left_val.logic_or(&right_val),
        }
    }

    /// Evaluates a unary operation
    pub(super) fn eval_unary(&mut self, op: UnaryOp, expr: Expr) -> Result<Value, String> {
        let val = self.eval_expr(expr)?;

        match op {
            UnaryOp::Not => val.not(),
            UnaryOp::Neg => val.neg(),
        }
    }
}
