/// Statement evaluation logic for the interpreter

use crate::ast::{Expr, Stmt};
use crate::value::Value;
use super::Interpreter;

impl Interpreter {
    /// Evaluates a statement
    pub(super) fn eval_stmt(&mut self, stmt: Stmt) -> Result<Value, String> {
        match stmt {
            Stmt::Expr(expr) => self.eval_expr(expr),
            Stmt::Let { name, value } => {
                let val = self.eval_expr(value)?;
                self.env.update(name, val.clone());
                Ok(val)
            }
            Stmt::Function { name, params, body } => {
                let func = Value::Function {
                    params,
                    body: *body,
                };
                self.env.set(name, func.clone());
                Ok(func)
            }
            Stmt::Return(expr) => {
                let val = if let Some(e) = expr {
                    self.eval_expr(e)?
                } else {
                    Value::Null
                };
                self.is_returning = true;
                Ok(val)
            }
            Stmt::While { condition, body } => {
                let mut last_value = Value::Null;
                while self.eval_expr(condition.clone())?.is_truthy() {
                    // Don't push scope here - Block expressions handle their own scoping
                    // but we need assignments to persist across iterations
                    match &*body {
                        Expr::Block(stmts) => {
                            for stmt in stmts {
                                last_value = self.eval_stmt(stmt.clone())?;
                                if self.is_returning {
                                    return Ok(last_value);
                                }
                            }
                        }
                        _ => {
                            last_value = self.eval_expr(*body.clone())?;
                        }
                    }
                }
                Ok(last_value)
            }
            Stmt::For {
                iterator,
                iterable,
                body,
            } => {
                let iter_value = self.eval_expr(iterable)?;
                let mut last_value = Value::Null;

                match iter_value {
                    Value::Array(arr) => {
                        for item in arr {
                            // Handle block bodies without extra scoping
                            // Blocks manage their own scopes
                            match &*body {
                                Expr::Block(stmts) => {
                                    self.env.push_scope();
                                    self.env.set(iterator.clone(), item);
                                    for stmt in stmts {
                                        last_value = self.eval_stmt(stmt.clone())?;
                                        if self.is_returning {
                                            break;
                                        }
                                    }
                                    self.env.pop_scope();
                                }
                                _ => {
                                    // For non-block bodies, we need to scope the iterator
                                    self.env.push_scope();
                                    self.env.set(iterator.clone(), item);
                                    last_value = self.eval_expr(*body.clone())?;
                                    self.env.pop_scope();
                                }
                            }
                            if self.is_returning {
                                break;
                            }
                        }
                    }
                    _ => return Err("For loop requires an array".to_string()),
                }

                Ok(last_value)
            }
            Stmt::Import { .. } => {
                // Import system not yet implemented
                Ok(Value::Null)
            }
        }
    }
}
