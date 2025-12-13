/// Interpreter for RustX language

mod environment;
mod eval_expr;
mod eval_stmt;
mod eval_ops;
mod builtins;

pub use environment::Environment;

use crate::ast::Stmt;
use crate::value::Value;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::fs;
use std::collections::HashMap;

/// Interpreter for RustX
pub struct Interpreter {
    pub env: Environment,
    pub is_returning: bool,
}

impl Interpreter {
    /// Creates a new interpreter
    pub fn new() -> Self {
        let mut interpreter = Interpreter {
            env: Environment::new(),
            is_returning: false,
        };
        interpreter.init_builtins();
        interpreter
    }

    /// Initializes built-in functions
    fn init_builtins(&mut self) {
        // Built-in print function (placeholder - will be enhanced later)
        // For now, we'll add range as a special case in eval_call
    }

    /// Evaluates a program (list of statements)
    pub fn eval_program(&mut self, statements: Vec<Stmt>) -> Result<Value, String> {
        let mut last_value = Value::Null;

        for stmt in statements {
            last_value = self.eval_stmt(stmt)?;
            if self.is_returning {
                break;
            }
        }

        Ok(last_value)
    }

    /// Helper to apply a function (Value) to arguments (Values)
    pub(super) fn apply_function(&mut self, func: Value, args: Vec<Value>) -> Result<Value, String> {
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

                for (param, arg) in params.iter().zip(args.into_iter()) {
                    self.env.set(param.clone(), arg);
                }

                let result = self.eval_expr(body)?;
                // Verify return state is consumed here
                if self.is_returning {
                    self.is_returning = false;
                }
                self.env.pop_scope();

                Ok(result)
            }
            _ => Err("Not a callable function".to_string()),
        }
    }

    /// Evaluates an imported file and returns its exports
    pub(super) fn eval_import_file(&mut self, path: &str) -> Result<Value, String> {
        let source = fs::read_to_string(path).map_err(|e| format!("Failed to read import '{}': {}", path, e))?;
        
        // Tokenize
        let mut lexer = Lexer::new(&source);
        let tokens = lexer.tokenize()?;
        
        // Parse
        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;
        
        // created new interpreter for isolation
        let mut module_interpreter = Interpreter::new();
        module_interpreter.eval_program(ast)?;
        
        // Extract exports
        let exports = module_interpreter.env.get_exports();
        Ok(Value::Map(exports))
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn eval(input: &str) -> Result<Value, String> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;
        let mut interpreter = Interpreter::new();
        interpreter.eval_program(ast)
    }

    #[test]
    fn test_arithmetic() {
        assert_eq!(eval("10 + 20").unwrap(), Value::Int(30));
        assert_eq!(eval("10 * 2 + 5").unwrap(), Value::Int(25));
        assert_eq!(eval("(10 + 5) * 2").unwrap(), Value::Int(30));
    }

    #[test]
    fn test_variables() {
        assert_eq!(eval("x = 10\nx + 5").unwrap(), Value::Int(15));
    }

    #[test]
    fn test_function() {
        let input = "fn add(a, b) => a + b\nadd(10, 20)";
        assert_eq!(eval(input).unwrap(), Value::Int(30));
    }

    #[test]
    fn test_if_expr() {
        assert_eq!(eval("if true { 10 } else { 20 }").unwrap(), Value::Int(10));
        assert_eq!(eval("if false { 10 } else { 20 }").unwrap(), Value::Int(20));
    }

    #[test]
    fn test_array() {
        let input = "arr = [1, 2, 3]\narr[1]";
        assert_eq!(eval(input).unwrap(), Value::Int(2));
    }
}
