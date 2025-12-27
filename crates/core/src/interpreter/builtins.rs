//! Built-in functions for the interpreter

use super::{Interpreter, InterpreterResult, RuntimeError};
use crate::ast::Expr;
use crate::value::Value;

impl Interpreter {
    /// Built-in range function: range(end) or range(start, end) or range(start, end, step)
    pub(super) fn builtin_range(&mut self, args: Vec<Expr>) -> InterpreterResult<Value> {
        let (start, end, step) = match args.len() {
            1 => {
                let end = self
                    .eval_expr(args[0].clone())?
                    .as_int()
                    .map_err(RuntimeError::from)?;
                (0, end, 1)
            }
            2 => {
                let start = self
                    .eval_expr(args[0].clone())?
                    .as_int()
                    .map_err(RuntimeError::from)?;
                let end = self
                    .eval_expr(args[1].clone())?
                    .as_int()
                    .map_err(RuntimeError::from)?;
                (start, end, 1)
            }
            3 => {
                let start = self
                    .eval_expr(args[0].clone())?
                    .as_int()
                    .map_err(RuntimeError::from)?;
                let end = self
                    .eval_expr(args[1].clone())?
                    .as_int()
                    .map_err(RuntimeError::from)?;
                let step = self
                    .eval_expr(args[2].clone())?
                    .as_int()
                    .map_err(RuntimeError::from)?;
                if step == 0 {
                    return Err(RuntimeError::ArgumentError(
                        "range step cannot be zero".to_string(),
                    ));
                }
                (start, end, step)
            }
            _ => {
                return Err(RuntimeError::ArgumentError(
                    "range() takes 1, 2, or 3 arguments".to_string(),
                ))
            }
        };

        let mut result = Vec::new();
        if step > 0 {
            let mut i = start;
            while i < end {
                result.push(Value::Int(i));
                i += step;
            }
        } else {
            let mut i = start;
            while i > end {
                result.push(Value::Int(i));
                i += step;
            }
        }

        Ok(Value::Array(result))
    }

    /// Built-in print function
    pub(super) fn builtin_print(&mut self, args: Vec<Expr>) -> InterpreterResult<Value> {
        for (i, arg) in args.iter().enumerate() {
            if i > 0 {
                print!(" ");
            }
            let val = self.eval_expr(arg.clone())?;
            print!("{}", val);
        }
        println!();
        Ok(Value::Null)
    }

    /// Built-in len function
    pub(super) fn builtin_len(&mut self, args: Vec<Expr>) -> InterpreterResult<Value> {
        if args.len() != 1 {
            return Err(RuntimeError::ArgumentError(
                "len() takes exactly 1 argument".to_string(),
            ));
        }
        let val = self.eval_expr(args[0].clone())?;
        Ok(Value::Int(val.len().map_err(RuntimeError::from)?))
    }

    /// Built-in type function
    pub(super) fn builtin_type(&mut self, args: Vec<Expr>) -> InterpreterResult<Value> {
        if args.len() != 1 {
            return Err(RuntimeError::ArgumentError(
                "type() takes exactly 1 argument".to_string(),
            ));
        }
        let val = self.eval_expr(args[0].clone())?;
        Ok(Value::String(val.type_name()))
    }

    /// Built-in push function
    pub(super) fn builtin_push(&mut self, args: Vec<Expr>) -> InterpreterResult<Value> {
        if args.len() != 2 {
            return Err(RuntimeError::ArgumentError(
                "push() takes exactly 2 arguments: array and value".to_string(),
            ));
        }

        // Get the array name
        let array_name = if let Expr::Ident(name) = &args[0] {
            name.clone()
        } else {
            return Err(RuntimeError::ArgumentError(
                "push() first argument must be an array variable".to_string(),
            ));
        };

        // Get current array value
        let mut arr_val = self.env.get(&array_name).map_err(RuntimeError::from)?;

        // Evaluate and push the new value
        let new_val = self.eval_expr(args[1].clone())?;
        let result = arr_val.push(new_val).map_err(RuntimeError::from)?;

        // Update the array in environment
        self.env.update(array_name, result.clone());
        Ok(result)
    }

    /// Built-in pop function
    pub(super) fn builtin_pop(&mut self, args: Vec<Expr>) -> InterpreterResult<Value> {
        if args.len() != 1 {
            return Err(RuntimeError::ArgumentError(
                "pop() takes exactly 1 argument: array".to_string(),
            ));
        }

        // Get the array name
        let array_name = if let Expr::Ident(name) = &args[0] {
            name.clone()
        } else {
            return Err(RuntimeError::ArgumentError(
                "pop() argument must be an array variable".to_string(),
            ));
        };

        // Get current array value
        let mut arr_val = self.env.get(&array_name).map_err(RuntimeError::from)?;

        // Pop
        let popped = arr_val.pop().map_err(RuntimeError::from)?;

        // Update the array in environment (arr_val is modified in place? No, Value::Array is ref counted? No, it's Clone)
        // Wait, Value::pop modifies specific `Value::Array`.
        // My `Value::pop` implementation:
        // Value::Array(arr) => { arr.pop(); Ok(Value::Array(arr.clone())) } is wrong for pop return!
        // `pop` should return the popped element, but also modify the array.
        // `Value` is pass-by-value (Clone) in Rust unless using RefCell.
        // RustX semantics: Arrays are... copied?
        // If `arr = [1]`, `push(arr, 2)` updates `arr`.
        // My implementation in `value.rs`:
        // pub fn pop(&mut self) -> Result<Value, String> { ... arr.pop()... }
        // The `self` is modified properly.
        // So here I need to update the environment with the modified array.

        // Re-eval logic:
        // `arr_val` is a local copy of Value from env.
        // `arr_val.pop()` returns the popped item AND modifies `arr_val`.
        // I need to update env with `arr_val` and return `popped`.

        // Checking `value.rs` implementation of pop again...
        // `arr.pop()` returns `Option<Value>`.
        // My wrapper: `Ok(popped)`. It modifies `self` (arr).
        // Good.

        self.env.update(array_name, arr_val);
        Ok(popped)
    }

    // === STRING FUNCTIONS ===

    pub(super) fn builtin_split(&mut self, args: Vec<Expr>) -> InterpreterResult<Value> {
        if args.len() != 2 {
            return Err(RuntimeError::ArgumentError(
                "split() takes 2 arguments".to_string(),
            ));
        }
        let s = self.eval_expr(args[0].clone())?;
        let d = self.eval_expr(args[1].clone())?;
        s.split(&d).map_err(RuntimeError::from)
    }

    pub(super) fn builtin_join(&mut self, args: Vec<Expr>) -> InterpreterResult<Value> {
        // join(array, delimiter) -> Value::join is not implemented on Array, but on String?
        // Usually array.join(delim).
        // My implementation: built-in `join(arr, delim)`.
        // `Value` implementation: I added `split` on String. Did I add `join` on Array?
        // No, I missed `join`. I added `split`.
        // I should add `join` to `Value::Array`.
        // For now I'll keep logic here or fix `Value`.
        // Let's fix `Value` in `value.rs` next step. Keep logic here for now?
        // Plan: Use logic here for join to save time or go back?
        // Going back is cleaner.

        if args.len() != 2 {
            return Err(RuntimeError::ArgumentError(
                "join() takes 2 arguments".to_string(),
            ));
        }
        let arr = self.eval_expr(args[0].clone())?;
        let d = self.eval_expr(args[1].clone())?;

        // Manual logic for now to avoid context switch loop
        match (arr, d) {
            (Value::Array(a), Value::String(del)) => {
                let strings: Vec<String> = a.iter().map(|v| format!("{}", v)).collect();
                Ok(Value::String(strings.join(&del)))
            }
            (Value::Array(_), _) => Err(RuntimeError::TypeMismatch {
                expected: "String".to_string(),
                found: "other".to_string(),
            }),
            (_, _) => Err(RuntimeError::TypeMismatch {
                expected: "Array".to_string(),
                found: "other".to_string(),
            }),
        }
    }

    pub(super) fn builtin_trim(&mut self, args: Vec<Expr>) -> InterpreterResult<Value> {
        if args.len() != 1 {
            return Err(RuntimeError::ArgumentError(
                "trim() takes 1 argument".to_string(),
            ));
        }
        self.eval_expr(args[0].clone())?
            .trim()
            .map_err(RuntimeError::from)
    }

    pub(super) fn builtin_upper(&mut self, args: Vec<Expr>) -> InterpreterResult<Value> {
        if args.len() != 1 {
            return Err(RuntimeError::ArgumentError(
                "upper() takes 1 argument".to_string(),
            ));
        }
        self.eval_expr(args[0].clone())?
            .upper()
            .map_err(RuntimeError::from)
    }

    pub(super) fn builtin_lower(&mut self, args: Vec<Expr>) -> InterpreterResult<Value> {
        if args.len() != 1 {
            return Err(RuntimeError::ArgumentError(
                "lower() takes 1 argument".to_string(),
            ));
        }
        self.eval_expr(args[0].clone())?
            .lower()
            .map_err(RuntimeError::from)
    }

    // === MATH FUNCTIONS ===

    pub(super) fn builtin_abs(&mut self, args: Vec<Expr>) -> InterpreterResult<Value> {
        if args.len() != 1 {
            return Err(RuntimeError::ArgumentError(
                "abs() takes 1 argument".to_string(),
            ));
        }
        self.eval_expr(args[0].clone())?
            .abs()
            .map_err(RuntimeError::from)
    }

    pub(super) fn builtin_min(&mut self, args: Vec<Expr>) -> InterpreterResult<Value> {
        if args.len() != 2 {
            return Err(RuntimeError::ArgumentError(
                "min() takes 2 arguments".to_string(),
            ));
        }
        // min not implemented on Value for 2 args directly as `min(other)`. I didn't add it.
        // Wait, I think I did? `value.rs`: pub fn min... NO. Only `lt`, `gt`.
        // I checks `value.rs` again... yes, I didn't add min/max to Value.
        // I added `abs`, `floor`.
        // I missed `min`, `max`, `join`.
        // I will implement them here manually for now.

        let a = self.eval_expr(args[0].clone())?;
        let b = self.eval_expr(args[1].clone())?;
        match (a, b) {
            (Value::Int(x), Value::Int(y)) => Ok(Value::Int(x.min(y))),
            (Value::Float(x), Value::Float(y)) => Ok(Value::Float(x.min(y))),
            (Value::Int(x), Value::Float(y)) => Ok(Value::Float((x as f64).min(y))),
            (Value::Float(x), Value::Int(y)) => Ok(Value::Float(x.min(y as f64))),
            _ => Err(RuntimeError::TypeMismatch {
                expected: "Number".to_string(),
                found: "other".to_string(),
            }),
        }
    }

    pub(super) fn builtin_max(&mut self, args: Vec<Expr>) -> InterpreterResult<Value> {
        if args.len() != 2 {
            return Err(RuntimeError::ArgumentError(
                "max() takes 2 arguments".to_string(),
            ));
        }
        let a = self.eval_expr(args[0].clone())?;
        let b = self.eval_expr(args[1].clone())?;
        match (a, b) {
            (Value::Int(x), Value::Int(y)) => Ok(Value::Int(x.max(y))),
            (Value::Float(x), Value::Float(y)) => Ok(Value::Float(x.max(y))),
            (Value::Int(x), Value::Float(y)) => Ok(Value::Float((x as f64).max(y))),
            (Value::Float(x), Value::Int(y)) => Ok(Value::Float(x.max(y as f64))),
            _ => Err(RuntimeError::TypeMismatch {
                expected: "Number".to_string(),
                found: "other".to_string(),
            }),
        }
    }

    pub(super) fn builtin_floor(&mut self, args: Vec<Expr>) -> InterpreterResult<Value> {
        if args.len() != 1 {
            return Err(RuntimeError::ArgumentError(
                "floor() takes 1 argument".to_string(),
            ));
        }
        self.eval_expr(args[0].clone())?
            .floor()
            .map_err(RuntimeError::from)
    }

    pub(super) fn builtin_ceil(&mut self, args: Vec<Expr>) -> InterpreterResult<Value> {
        if args.len() != 1 {
            return Err(RuntimeError::ArgumentError(
                "ceil() takes 1 argument".to_string(),
            ));
        }
        self.eval_expr(args[0].clone())?
            .ceil()
            .map_err(RuntimeError::from)
    }

    pub(super) fn builtin_round(&mut self, args: Vec<Expr>) -> InterpreterResult<Value> {
        if args.len() != 1 {
            return Err(RuntimeError::ArgumentError(
                "round() takes 1 argument".to_string(),
            ));
        }
        self.eval_expr(args[0].clone())?
            .round()
            .map_err(RuntimeError::from)
    }

    // === ARRAY FUNCTIONS (Keep logic here for map/filter/reduce as outlined in plan) ===

    pub(super) fn builtin_map(&mut self, args: Vec<Expr>) -> InterpreterResult<Value> {
        if args.len() != 2 {
            return Err(RuntimeError::ArgumentError(
                "map() takes exactly 2 arguments: array and callback".to_string(),
            ));
        }
        let arr = self.eval_expr(args[0].clone())?;
        self.logic_map(arr, args[1].clone())
    }

    pub(super) fn builtin_filter(&mut self, args: Vec<Expr>) -> InterpreterResult<Value> {
        if args.len() != 2 {
            return Err(RuntimeError::ArgumentError(
                "filter() takes exactly 2 arguments: array and callback".to_string(),
            ));
        }
        let arr = self.eval_expr(args[0].clone())?;
        self.logic_filter(arr, args[1].clone())
    }

    pub(super) fn builtin_reduce(&mut self, args: Vec<Expr>) -> InterpreterResult<Value> {
        if args.len() < 2 || args.len() > 3 {
            return Err(RuntimeError::ArgumentError(
                "reduce() takes 2 or 3 arguments".to_string(),
            ));
        }
        let arr = self.eval_expr(args[0].clone())?;
        let initial = if args.len() == 3 {
            Some(args[2].clone())
        } else {
            None
        };
        self.logic_reduce(arr, args[1].clone(), initial)
    }

    pub(super) fn builtin_reverse(&mut self, args: Vec<Expr>) -> InterpreterResult<Value> {
        if args.len() != 1 {
            return Err(RuntimeError::ArgumentError(
                "reverse() takes 1 argument".to_string(),
            ));
        }
        let mut arr = self.eval_expr(args[0].clone())?;
        // reverse_in_place modifies and returns array
        arr.reverse_in_place().map_err(RuntimeError::from)
    }

    pub(super) fn builtin_sort(&mut self, args: Vec<Expr>) -> InterpreterResult<Value> {
        if args.len() != 1 {
            return Err(RuntimeError::ArgumentError(
                "sort() takes 1 argument".to_string(),
            ));
        }
        let mut arr = self.eval_expr(args[0].clone())?;
        arr.sort_in_place().map_err(RuntimeError::from)
    }

    // Logic helpers for map/filter/reduce (remain here as they use apply_function)
    pub(super) fn logic_map(&mut self, arr: Value, callback: Expr) -> InterpreterResult<Value> {
        let items = match arr {
            Value::Array(a) => a,
            _ => {
                return Err(RuntimeError::TypeMismatch {
                    expected: "Array".to_string(),
                    found: "other".to_string(),
                })
            }
        };

        let callback_val = self.eval_expr(callback)?;
        let mut new_arr = Vec::new();
        for item in items {
            new_arr.push(self.apply_function(callback_val.clone(), vec![item])?);
        }
        Ok(Value::Array(new_arr))
    }

    pub(super) fn logic_filter(&mut self, arr: Value, callback: Expr) -> InterpreterResult<Value> {
        let items = match arr {
            Value::Array(a) => a,
            _ => {
                return Err(RuntimeError::TypeMismatch {
                    expected: "Array".to_string(),
                    found: "other".to_string(),
                })
            }
        };

        let callback_val = self.eval_expr(callback)?;
        let mut new_arr = Vec::new();
        for item in items {
            let result = self.apply_function(callback_val.clone(), vec![item.clone()])?;
            if result.is_truthy() {
                new_arr.push(item);
            }
        }
        Ok(Value::Array(new_arr))
    }

    pub(super) fn logic_reduce(
        &mut self,
        arr: Value,
        callback: Expr,
        initial: Option<Expr>,
    ) -> InterpreterResult<Value> {
        let items = match arr {
            Value::Array(a) => a,
            _ => {
                return Err(RuntimeError::TypeMismatch {
                    expected: "Array".to_string(),
                    found: "other".to_string(),
                })
            }
        };

        let mut iterator = items.into_iter();
        let mut accumulator = if let Some(init_expr) = initial {
            self.eval_expr(init_expr)?
        } else {
            iterator.next().ok_or_else(|| {
                RuntimeError::from("reduce() on empty array with no initial value")
            })?
        };

        let callback_val = self.eval_expr(callback)?;
        for item in iterator {
            accumulator = self.apply_function(callback_val.clone(), vec![accumulator, item])?;
        }
        Ok(accumulator)
    }
}
