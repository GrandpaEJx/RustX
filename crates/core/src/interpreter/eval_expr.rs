//! Expression evaluation logic for the interpreter

use crate::ast::Expr;
use crate::value::Value;
use std::collections::HashMap;
use super::{Interpreter, InterpreterResult, RuntimeError};

impl Interpreter {
    /// Evaluates an expression
    pub(super) fn eval_expr(&mut self, expr: Expr) -> InterpreterResult<Value> {
        match expr {
            Expr::Int(n) => Ok(Value::Int(n)),
            Expr::Float(f) => Ok(Value::Float(f)),
            Expr::String(s) => Ok(Value::String(s)),
            Expr::TemplateString(template) => {
                // Interpolate variables in template string
                let mut result = String::new();
                let mut chars = template.chars().peekable();
                
                while let Some(ch) = chars.next() {
                    if ch == '{' {
                        // Extract variable name
                        let mut var_name = String::new();
                        while let Some(&next_ch) = chars.peek() {
                            if next_ch == '}' {
                                chars.next(); // consume '}'
                                break;
                            }
                            var_name.push(chars.next().unwrap());
                        }
                        
                        // Get variable value and append to result
                        if !var_name.is_empty() {
                            match self.env.get(&var_name) {
                                Ok(value) => result.push_str(&format!("{}", value)),
                                Err(_) => result.push_str(&format!("{{{}}}", var_name)), // Keep as-is if not found
                            }
                        }
                    } else {
                        result.push(ch);
                    }
                }
                
                Ok(Value::String(result))
            }
            Expr::Bool(b) => Ok(Value::Bool(b)),
            Expr::Null => Ok(Value::Null),
            Expr::Ident(name) => self.env.get(&name).map_err(|_| RuntimeError::UndefinedVariable(name)),
            Expr::Binary { left, op, right } => self.eval_binary(*left, op, *right),
            Expr::Unary { op, expr } => self.eval_unary(op, *expr),
            Expr::Call { callee, args } => self.eval_call(*callee, args),
            Expr::MethodCall { object, method, args } => {
                // Evaluate the object
                let mut obj_value = self.eval_expr(*object)?;
                
                // Handle callback-based methods manually (Value doesn't support them)
                 match method.as_str() {
                    "map" => {
                        if args.len() != 1 { return Err(RuntimeError::ArgumentError("map() requires 1 argument: callback".to_string())); }
                        return self.logic_map(obj_value, args[0].clone());
                    },
                    "filter" => {
                        if args.len() != 1 { return Err(RuntimeError::ArgumentError("filter() requires 1 argument: callback".to_string())); }
                        return self.logic_filter(obj_value, args[0].clone());
                    },
                    "reduce" => {
                        if args.is_empty() || args.len() > 2 { return Err(RuntimeError::ArgumentError("reduce() requires 1 or 2 arguments: callback, [initial]".to_string())); }
                        let initial = if args.len() == 2 { Some(args[1].clone()) } else { None };
                        return self.logic_reduce(obj_value, args[0].clone(), initial);
                    },
                    _ => {}
                 }

                // Evaluate arguments
                let mut arg_values = Vec::new();
                for arg in args {
                    arg_values.push(self.eval_expr(arg)?);
                }
                
                // Use centralized call_method
                obj_value.call_method(&method, arg_values).map_err(RuntimeError::from)
            }
            Expr::Array(elements) => {
                let mut arr = Vec::new();
                for elem in elements {
                    arr.push(self.eval_expr(elem)?);
                }
                Ok(Value::Array(arr))
            }
            Expr::Map(pairs) => {
                let mut map = HashMap::new();
                for (key, value) in pairs {
                    map.insert(key, self.eval_expr(value)?);
                }
                Ok(Value::Map(map))
            }
            Expr::Index { object, index } => self.eval_index(*object, *index),
            Expr::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if self.eval_expr(*condition)?.is_truthy() {
                    self.eval_expr(*then_branch)
                } else if let Some(else_expr) = else_branch {
                    self.eval_expr(*else_expr)
                } else {
                    Ok(Value::Null)
                }
            }
            Expr::Block(statements) => {
                self.env.push_scope();
                let mut last_value = Value::Null;

                for stmt in statements {
                    last_value = self.eval_stmt(stmt)?;
                    if self.is_returning {
                        break;
                    }
                }

                self.env.pop_scope();
                Ok(last_value)
            }
            Expr::Assign { name, value } => {
                let val = self.eval_expr(*value)?;
                self.env.update(name, val.clone());
                Ok(val)
            }
        }
    }

    /// Evaluates index access
    pub(super) fn eval_index(&mut self, object: Expr, index: Expr) -> InterpreterResult<Value> {
        let obj_val = self.eval_expr(object)?;
        let idx_val = self.eval_expr(index)?;

        match (obj_val, idx_val) {
            (Value::Array(arr), Value::Int(idx)) => {
                let index = if idx < 0 {
                    (arr.len() as i64 + idx) as usize
                } else {
                    idx as usize
                };

                arr.get(index)
                    .cloned()
                    .ok_or_else(|| RuntimeError::from("Array index out of bounds"))
            }
            (Value::Map(map), Value::String(key)) => map
                .get(&key)
                .cloned()
                .ok_or_else(|| RuntimeError::from(format!("Key '{}' not found in map", key))),
            _ => Err(RuntimeError::TypeMismatch { expected: "Array or Map".to_string(), found: "other".to_string() }),
        }
    }

    /// Evaluates a function call
    pub(super) fn eval_call(&mut self, callee: Expr, args: Vec<Expr>) -> InterpreterResult<Value> {
        // Check for built-in functions first
        if let Expr::Ident(name) = &callee {
            match name.as_str() {
                "range" => return self.builtin_range(args),
                "print" => return self.builtin_print(args),
                "len" => return self.builtin_len(args),
                "type" => return self.builtin_type(args),
                "push" => return self.builtin_push(args),
                "pop" => return self.builtin_pop(args),
                // String functions
                "split" => return self.builtin_split(args),
                "join" => return self.builtin_join(args),
                "trim" => return self.builtin_trim(args),
                "upper" => return self.builtin_upper(args),
                "lower" => return self.builtin_lower(args),
                // Math functions
                "abs" => return self.builtin_abs(args),
                "min" => return self.builtin_min(args),
                "max" => return self.builtin_max(args),
                "floor" => return self.builtin_floor(args),
                "ceil" => return self.builtin_ceil(args),
                "round" => return self.builtin_round(args),
                // Array functions
                "map" => return self.builtin_map(args),
                "filter" => return self.builtin_filter(args),
                "reduce" => return self.builtin_reduce(args),
                "reverse" => return self.builtin_reverse(args),
                "sort" => return self.builtin_sort(args),
                _ => {}
            }
        }

        // Evaluate arguments first
        let mut arg_values = Vec::new();
        for arg in args {
            arg_values.push(self.eval_expr(arg)?);
        }

        let func = self.eval_expr(callee)?;
        self.apply_function(func, arg_values)
    }
}
