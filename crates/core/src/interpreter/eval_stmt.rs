//! Statement evaluation logic for the interpreter

use crate::ast::{Expr, Stmt};
use crate::value::Value;
use super::{Interpreter, InterpreterResult, RuntimeError};

impl Interpreter {
    /// Evaluates a statement
    pub(super) fn eval_stmt(&mut self, stmt: Stmt) -> InterpreterResult<Value> {
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
                    _ => return Err(RuntimeError::TypeMismatch { expected: "Array".to_string(), found: format!("{}", iter_value) }),
                }

                Ok(last_value)
            }
            Stmt::Use { module } => {
                //Lazy-load stdlib module on first use
                let stdlib_modules = ["web", "json", "http", "os", "time", "fs", "term"];
                
                if !stdlib_modules.contains(&module.as_str()) {
                    return Err(RuntimeError::ImportError(
                        format!("Unknown stdlib module: '{}'. Available modules: {:?}", module, stdlib_modules)
                    ));
                }
                
                // Load module (this will set it in environment)
                self.load_stdlib_module(&module)?;
                Ok(Value::Null)
            }
            Stmt::Import { path, alias } => {
                // Check if this is a stdlib module import
                let stdlib_modules = ["web", "json", "http", "os", "time", "fs", "term"];
                
                let module = if stdlib_modules.contains(&path.as_str()) {
                    // Load stdlib module from environment
                    self.env.get(&path).map_err(|_| {
                        RuntimeError::ImportError(format!("Stdlib module '{}' not found", path))
                    })?
                } else {
                    // Load from file
                    self.eval_import_file(&path)?
                };
                
                if let Some(name) = alias {
                    self.env.set(name, module);
                } else {
                    return Err(RuntimeError::ImportError("File imports require an alias (e.g., import \"file.rsx\" as name)".to_string()));
                }
                Ok(Value::Null)
            }
            Stmt::RustImport { .. } | Stmt::RustBlock { .. } => {
                Err(RuntimeError::FeatureNotSupported("Rust imports and blocks require JIT compilation. This feature is not available in pure interpreter mode.".to_string()))
            }
        }
    }
}
