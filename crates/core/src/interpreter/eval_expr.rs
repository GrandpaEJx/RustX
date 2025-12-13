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
            Expr::MethodCall { object, method, args } => {
                // Evaluate the object
                let obj_value = self.eval_expr(*object)?;
                
                // Call the appropriate method based on the method name
                match method.as_str() {
                    // String methods
                    "upper" => match obj_value {
                        Value::String(s) => Ok(Value::String(s.to_uppercase())),
                        _ => Err("upper() can only be called on strings".to_string()),
                    },
                    "lower" => match obj_value {
                        Value::String(s) => Ok(Value::String(s.to_lowercase())),
                        _ => Err("lower() can only be called on strings".to_string()),
                    },
                    "trim" => match obj_value {
                        Value::String(s) => Ok(Value::String(s.trim().to_string())),
                        _ => Err("trim() can only be called on strings".to_string()),
                    },
                    "split" => {
                        if args.len() != 1 {
                            return Err("split() requires exactly 1 argument".to_string());
                        }
                        match obj_value {
                            Value::String(s) => {
                                let delimiter = match self.eval_expr(args[0].clone())? {
                                    Value::String(d) => d,
                                    _ => return Err("split() delimiter must be a string".to_string()),
                                };
                                let parts: Vec<Value> = s.split(&delimiter)
                                    .map(|p| Value::String(p.to_string()))
                                    .collect();
                                Ok(Value::Array(parts))
                            }
                            _ => Err("split() can only be called on strings".to_string()),
                        }
                    },
                    // Array methods
                    "len" => match obj_value {
                        Value::Array(arr) => Ok(Value::Int(arr.len() as i64)),
                        Value::String(s) => Ok(Value::Int(s.len() as i64)),
                        Value::Map(map) => Ok(Value::Int(map.len() as i64)),
                        _ => Err("len() can only be called on arrays, strings, or maps".to_string()),
                    },
                    "map" => {
                        if args.len() != 1 { return Err("map() requires 1 argument: callback".to_string()); }
                        self.logic_map(obj_value, args[0].clone())
                    },
                    "filter" => {
                        if args.len() != 1 { return Err("filter() requires 1 argument: callback".to_string()); }
                        self.logic_filter(obj_value, args[0].clone())
                    },
                    "reduce" => {
                        if args.len() < 1 || args.len() > 2 { return Err("reduce() requires 1 or 2 arguments: callback, [initial]".to_string()); }
                        let initial = if args.len() == 2 { Some(args[1].clone()) } else { None };
                        self.logic_reduce(obj_value, args[0].clone(), initial)
                    },
                    "reverse" => {
                        if !args.is_empty() { return Err("reverse() takes no arguments".to_string()); }
                        self.logic_reverse(obj_value)
                    },
                    "sort" => {
                        if !args.is_empty() { return Err("sort() takes no arguments".to_string()); }
                        self.logic_sort(obj_value)
                    },
                    // Math methods
                    "abs" => match obj_value {
                        Value::Int(n) => Ok(Value::Int(n.abs())),
                        Value::Float(f) => Ok(Value::Float(f.abs())),
                        _ => Err("abs() can only be called on numbers".to_string()),
                    },
                    "floor" => match obj_value {
                        Value::Float(f) => Ok(Value::Int(f.floor() as i64)),
                        Value::Int(n) => Ok(Value::Int(n)),
                        _ => Err("floor() can only be called on numbers".to_string()),
                    },
                    "ceil" => match obj_value {
                        Value::Float(f) => Ok(Value::Int(f.ceil() as i64)),
                        Value::Int(n) => Ok(Value::Int(n)),
                        _ => Err("ceil() can only be called on numbers".to_string()),
                    },
                    "round" => match obj_value {
                        Value::Float(f) => Ok(Value::Int(f.round() as i64)),
                        Value::Int(n) => Ok(Value::Int(n)),
                        _ => Err("round() can only be called on numbers".to_string()),
                    },
                    _ => {
                        // Fallback: Check if object is a map and method is a key
                        if let Value::Map(ref map) = obj_value {
                            if let Some(val) = map.get(&method) {
                                // If arguments are provided for a non-method map entry, it might be a function call on that entry?
                                // Standard Dot Access "lib.func(args)" vs "lib.func"
                                // If "lib.func" is called as method, args are provided.
                                // If "lib.PI" is access, args are likely empty (checked by parser? No, MethodCall ALWAYS has args Vec)
                                // Parser wraps property access like `lib.PI` as MethodCall with empty args ??
                                // Wait, parser likely produces `MethodCall` if `.` follows.
                                // If `lib.PI` is just access, args is empty.
                                if args.is_empty() {
                                    return Ok(val.clone());
                                } else {
                                    // It's a method call simulation: `lib.func(arg)`
                                    // `val` is the function.
                                    // Use `apply_function`? Or `eval_call` logic?
                                    // `apply_function` takes Values. `args` here are `Expr`.
                                    // So we need to evaluate args.
                                    // This reuses the logic from `eval_call` basically.
                                    // Wait, `eval_expr` logic for `Expr::Call` handles `Expr` args.
                                    // Here we are inside `Expr::MethodCall`.
                                    
                                     // Evaluate arguments first
                                    let mut arg_values = Vec::new();
                                    for arg in args {
                                        arg_values.push(self.eval_expr(arg)?);
                                    }
                                    return self.apply_function(val.clone(), arg_values);
                                }
                            }
                        }
                        Err(format!("Unknown method or property: {}", method))
                    }
                }
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
