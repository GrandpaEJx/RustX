/// Built-in functions for the interpreter

use crate::ast::Expr;
use crate::value::Value;
use super::Interpreter;

impl Interpreter {
    /// Built-in range function: range(end) or range(start, end) or range(start, end, step)
    pub(super) fn builtin_range(&mut self, args: Vec<Expr>) -> Result<Value, String> {
        let (start, end, step) = match args.len() {
            1 => {
                let end = self.eval_expr(args[0].clone())?.as_int()?;
                (0, end, 1)
            }
            2 => {
                let start = self.eval_expr(args[0].clone())?.as_int()?;
                let end = self.eval_expr(args[1].clone())?.as_int()?;
                (start, end, 1)
            }
            3 => {
                let start = self.eval_expr(args[0].clone())?.as_int()?;
                let end = self.eval_expr(args[1].clone())?.as_int()?;
                let step = self.eval_expr(args[2].clone())?.as_int()?;
                if step == 0 {
                    return Err("range step cannot be zero".to_string());
                }
                (start, end, step)
            }
            _ => return Err("range() takes 1, 2, or 3 arguments".to_string()),
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
    pub(super) fn builtin_print(&mut self, args: Vec<Expr>) -> Result<Value, String> {
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

    /// Built-in len function - returns length of array or string
    pub(super) fn builtin_len(&mut self, args: Vec<Expr>) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("len() takes exactly 1 argument".to_string());
        }

        let val = self.eval_expr(args[0].clone())?;
        match val {
            Value::Array(arr) => Ok(Value::Int(arr.len() as i64)),
            Value::String(s) => Ok(Value::Int(s.len() as i64)),
            Value::Map(map) => Ok(Value::Int(map.len() as i64)),
            _ => Err("len() requires an array, string, or map".to_string()),
        }
    }

    /// Built-in type function - returns type name as string
    pub(super) fn builtin_type(&mut self, args: Vec<Expr>) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("type() takes exactly 1 argument".to_string());
        }

        let val = self.eval_expr(args[0].clone())?;
        let type_name = match val {
            Value::Int(_) => "int",
            Value::Float(_) => "float",
            Value::String(_) => "string",
            Value::Bool(_) => "bool",
            Value::Array(_) => "array",
            Value::Map(_) => "map",
            Value::Function { .. } => "function",
            Value::Null => "null",
        };
        Ok(Value::String(type_name.to_string()))
    }

    /// Built-in push function - adds element to end of array (mutates the array)
    pub(super) fn builtin_push(&mut self, args: Vec<Expr>) -> Result<Value, String> {
        if args.len() != 2 {
            return Err("push() takes exactly 2 arguments: array and value".to_string());
        }

        // Get the array name
        let array_name = if let Expr::Ident(name) = &args[0] {
            name.clone()
        } else {
            return Err("push() first argument must be an array variable".to_string());
        };

        // Get current array value
        let mut arr = match self.env.get(&array_name)? {
            Value::Array(a) => a,
            _ => return Err("push() requires an array as first argument".to_string()),
        };

        // Evaluate and push the new value
        let new_val = self.eval_expr(args[1].clone())?;
        arr.push(new_val);

        // Update the array in environment
        self.env.update(array_name, Value::Array(arr.clone()));
        Ok(Value::Array(arr))
    }

    /// Built-in pop function - removes and returns last element from array
    pub(super) fn builtin_pop(&mut self, args: Vec<Expr>) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("pop() takes exactly 1 argument: array".to_string());
        }

        // Get the array name
        let array_name = if let Expr::Ident(name) = &args[0] {
            name.clone()
        } else {
            return Err("pop() argument must be an array variable".to_string());
        };

        // Get current array value
        let mut arr = match self.env.get(&array_name)? {
            Value::Array(a) => a,
            _ => return Err("pop() requires an array".to_string()),
        };

        // Pop the last value
        let popped = arr.pop().ok_or_else(|| "Cannot pop from empty array".to_string())?;

        // Update the array in environment
        self.env.update(array_name, Value::Array(arr));
        Ok(popped)
    }

    // === STRING FUNCTIONS ===

    /// Built-in split function - splits string into array
    pub(super) fn builtin_split(&mut self, args: Vec<Expr>) -> Result<Value, String> {
        if args.len() != 2 {
            return Err("split() takes exactly 2 arguments: string and delimiter".to_string());
        }

        let string = match self.eval_expr(args[0].clone())? {
            Value::String(s) => s,
            _ => return Err("split() first argument must be a string".to_string()),
        };

        let delimiter = match self.eval_expr(args[1].clone())? {
            Value::String(d) => d,
            _ => return Err("split() second argument must be a string".to_string()),
        };

        let parts: Vec<Value> = string
            .split(&delimiter)
            .map(|s| Value::String(s.to_string()))
            .collect();

        Ok(Value::Array(parts))
    }

    /// Built-in join function - joins array into string
    pub(super) fn builtin_join(&mut self, args: Vec<Expr>) -> Result<Value, String> {
        if args.len() != 2 {
            return Err("join() takes exactly 2 arguments: array and delimiter".to_string());
        }

        let arr = match self.eval_expr(args[0].clone())? {
            Value::Array(a) => a,
            _ => return Err("join() first argument must be an array".to_string()),
        };

        let delimiter = match self.eval_expr(args[1].clone())? {
            Value::String(d) => d,
            _ => return Err("join() second argument must be a string".to_string()),
        };

        let strings: Vec<String> = arr.iter().map(|v| format!("{}", v)).collect();
        Ok(Value::String(strings.join(&delimiter)))
    }

    /// Built-in trim function - removes whitespace from both ends
    pub(super) fn builtin_trim(&mut self, args: Vec<Expr>) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("trim() takes exactly 1 argument".to_string());
        }

        match self.eval_expr(args[0].clone())? {
            Value::String(s) => Ok(Value::String(s.trim().to_string())),
            _ => Err("trim() requires a string".to_string()),
        }
    }

    /// Built-in upper function - converts to uppercase
    pub(super) fn builtin_upper(&mut self, args: Vec<Expr>) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("upper() takes exactly 1 argument".to_string());
        }

        match self.eval_expr(args[0].clone())? {
            Value::String(s) => Ok(Value::String(s.to_uppercase())),
            _ => Err("upper() requires a string".to_string()),
        }
    }

    /// Built-in lower function - converts to lowercase
    pub(super) fn builtin_lower(&mut self, args: Vec<Expr>) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("lower() takes exactly 1 argument".to_string());
        }

        match self.eval_expr(args[0].clone())? {
            Value::String(s) => Ok(Value::String(s.to_lowercase())),
            _ => Err("lower() requires a string".to_string()),
        }
    }

    // === MATH FUNCTIONS ===

    /// Built-in abs function - absolute value
    pub(super) fn builtin_abs(&mut self, args: Vec<Expr>) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("abs() takes exactly 1 argument".to_string());
        }

        match self.eval_expr(args[0].clone())? {
            Value::Int(n) => Ok(Value::Int(n.abs())),
            Value::Float(f) => Ok(Value::Float(f.abs())),
            _ => Err("abs() requires a number".to_string()),
        }
    }

    /// Built-in min function - minimum of two numbers
    pub(super) fn builtin_min(&mut self, args: Vec<Expr>) -> Result<Value, String> {
        if args.len() != 2 {
            return Err("min() takes exactly 2 arguments".to_string());
        }

        let a = self.eval_expr(args[0].clone())?;
        let b = self.eval_expr(args[1].clone())?;

        match (a, b) {
            (Value::Int(x), Value::Int(y)) => Ok(Value::Int(x.min(y))),
            (Value::Float(x), Value::Float(y)) => Ok(Value::Float(x.min(y))),
            (Value::Int(x), Value::Float(y)) => Ok(Value::Float((x as f64).min(y))),
            (Value::Float(x), Value::Int(y)) => Ok(Value::Float(x.min(y as f64))),
            _ => Err("min() requires two numbers".to_string()),
        }
    }

    /// Built-in max function - maximum of two numbers
    pub(super) fn builtin_max(&mut self, args: Vec<Expr>) -> Result<Value, String> {
        if args.len() != 2 {
            return Err("max() takes exactly 2 arguments".to_string());
        }

        let a = self.eval_expr(args[0].clone())?;
        let b = self.eval_expr(args[1].clone())?;

        match (a, b) {
            (Value::Int(x), Value::Int(y)) => Ok(Value::Int(x.max(y))),
            (Value::Float(x), Value::Float(y)) => Ok(Value::Float(x.max(y))),
            (Value::Int(x), Value::Float(y)) => Ok(Value::Float((x as f64).max(y))),
            (Value::Float(x), Value::Int(y)) => Ok(Value::Float(x.max(y as f64))),
            _ => Err("max() requires two numbers".to_string()),
        }
    }

    /// Built-in floor function - rounds down
    pub(super) fn builtin_floor(&mut self, args: Vec<Expr>) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("floor() takes exactly 1 argument".to_string());
        }

        match self.eval_expr(args[0].clone())? {
            Value::Float(f) => Ok(Value::Int(f.floor() as i64)),
            Value::Int(n) => Ok(Value::Int(n)),
            _ => Err("floor() requires a number".to_string()),
        }
    }

    /// Built-in ceil function - rounds up
    pub(super) fn builtin_ceil(&mut self, args: Vec<Expr>) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("ceil() takes exactly 1 argument".to_string());
        }

        match self.eval_expr(args[0].clone())? {
            Value::Float(f) => Ok(Value::Int(f.ceil() as i64)),
            Value::Int(n) => Ok(Value::Int(n)),
            _ => Err("ceil() requires a number".to_string()),
        }
    }

    /// Built-in round function - rounds to nearest integer
    pub(super) fn builtin_round(&mut self, args: Vec<Expr>) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("round() takes exactly 1 argument".to_string());
        }

        match self.eval_expr(args[0].clone())? {
            Value::Float(f) => Ok(Value::Int(f.round() as i64)),
            Value::Int(n) => Ok(Value::Int(n)),
            _ => Err("round() requires a number".to_string()),
        }
    }
}

