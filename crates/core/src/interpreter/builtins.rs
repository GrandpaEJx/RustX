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
}
