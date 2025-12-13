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
}

