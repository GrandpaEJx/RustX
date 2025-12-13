/// Expression evaluation logic for the interpreter

use crate::ast::Expr;
use crate::value::Value;
use std::collections::HashMap;
use super::Interpreter;

impl Interpreter {
    /// Evaluates an expression
    pub(super) fn eval_expr(&mut self, expr: Expr) -> Result<Value, String> {
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
            Expr::Ident(name) => self.env.get(&name),
            Expr::Binary { left, op, right } => self.eval_binary(*left, op, *right),
            Expr::Unary { op, expr } => self.eval_unary(op, *expr),
            Expr::Call { callee, args } => self.eval_call(*callee, args),
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
    pub(super) fn eval_index(&mut self, object: Expr, index: Expr) -> Result<Value, String> {
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
                    .ok_or_else(|| "Array index out of bounds".to_string())
            }
            (Value::Map(map), Value::String(key)) => map
                .get(&key)
                .cloned()
                .ok_or_else(|| format!("Key '{}' not found in map", key)),
            _ => Err("Invalid index operation".to_string()),
        }
    }

    /// Evaluates a function call
    pub(super) fn eval_call(&mut self, callee: Expr, args: Vec<Expr>) -> Result<Value, String> {
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
                _ => {}
            }
        }

        let func = self.eval_expr(callee)?;

        match func {
            Value::Function { params, body } => {
                if params.len() != args.len() {
                    return Err(format!(
                        "Expected {} arguments, got {}",
                        params.len(),
                        args.len()
                    ));
                }

                self.env.push_scope();

                for (param, arg) in params.iter().zip(args.iter()) {
                    let arg_val = self.eval_expr(arg.clone())?;
                    self.env.set(param.clone(), arg_val);
                }

                let result = self.eval_expr(body)?;
                self.env.pop_scope();

                Ok(result)
            }
            _ => Err("Not a callable function".to_string()),
        }
    }
}
