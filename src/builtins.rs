use crate::error::Result;
use crate::runtime::Value;

/// Built-in functions for RustX scripting language
pub struct Builtins;

impl Builtins {
    /// Print function without newline
    pub fn print(args: &[Value]) -> Result<()> {
        for arg in args {
            print!("{}", arg);
        }
        Ok(())
    }
    
    /// Print function with newline
    pub fn println(args: &[Value]) -> Result<()> {
        for arg in args {
            println!("{}", arg);
        }
        Ok(())
    }
    
    /// Printf function with format string support
    pub fn printf(args: &[Value]) -> Result<()> {
        if args.is_empty() {
            return Ok(());
        }
        
        let format_str = args[0].as_string()?;
        let mut result = format_str;
        
        // Replace {0}, {1}, etc. with arguments
        for (i, arg) in args.iter().enumerate().skip(1) {
            let placeholder = format!("{{{}}}", i - 1);
            result = result.replace(&placeholder, &arg.to_string());
        }
        
        print!("{}", result);
        Ok(())
    }
    
    /// Get the length of a string
    pub fn length(args: &[Value]) -> Result<Value> {
        if args.len() != 1 {
            return Err(crate::error::Error::RuntimeError(
                "length function expects exactly one argument".to_string()
            ));
        }
        
        match &args[0] {
            Value::String(s) => Ok(Value::Integer(s.len() as i64)),
            _ => Err(crate::error::Error::RuntimeError(
                "length function expects a string argument".to_string()
            )),
        }
    }
    
    /// Convert value to string
    pub fn to_string(args: &[Value]) -> Result<Value> {
        if args.len() != 1 {
            return Err(crate::error::Error::RuntimeError(
                "to_string function expects exactly one argument".to_string()
            ));
        }
        
        Ok(Value::String(args[0].as_string()?))
    }
    
    /// Check if value is null
    pub fn is_null(args: &[Value]) -> Result<Value> {
        if args.len() != 1 {
            return Err(crate::error::Error::RuntimeError(
                "is_null function expects exactly one argument".to_string()
            ));
        }
        
        Ok(Value::Boolean(matches!(args[0], Value::Null)))
    }
}