//! Interpreter for RustX language

mod environment;
mod eval_expr;
mod eval_stmt;
mod eval_ops;
mod builtins;
pub mod error;

pub use environment::Environment;
pub use error::RuntimeError;

use crate::ast::Stmt;
use crate::value::Value;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::cell::RefCell;
use std::fs;

/// Interpreter for RustX
pub struct Interpreter {
    pub env: Environment,
    pub is_returning: bool,
    pub module_cache: Rc<RefCell<HashMap<String, Value>>>,
    pub loading_modules: Rc<RefCell<HashSet<String>>>,
}

pub type InterpreterResult<T> = Result<T, RuntimeError>;

impl Interpreter {
    /// Creates a new interpreter
    pub fn new() -> Self {
        let mut interpreter = Interpreter {
            env: Environment::new(),
            is_returning: false,
            module_cache: Rc::new(RefCell::new(HashMap::new())),
            loading_modules: Rc::new(RefCell::new(HashSet::new())),
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
    pub fn eval_program(&mut self, statements: Vec<Stmt>) -> InterpreterResult<Value> {
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
    pub(super) fn apply_function(&mut self, func: Value, args: Vec<Value>) -> InterpreterResult<Value> {
        match func {
            Value::Function { params, body } => {
                if params.len() != args.len() {
                    return Err(RuntimeError::ArgumentError(format!(
                        "Expected {} arguments, got {}",
                        params.len(),
                        args.len()
                    )));
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
            _ => Err(RuntimeError::TypeMismatch {
                expected: "Function".to_string(),
                found: format!("{}", func),
            }),
        }
    }

    /// Evaluates an imported file and returns its exports
    pub(super) fn eval_import_file(&mut self, path: &str) -> InterpreterResult<Value> {
        let canonical_path = fs::canonicalize(path)
            .map_err(|e| RuntimeError::ImportError(format!("Failed to resolve path '{}': {}", path, e)))?
            .to_string_lossy()
            .to_string();

        // Check cache first
        if let Some(cached_module) = self.module_cache.borrow().get(&canonical_path) {
            return Ok(cached_module.clone());
        }

        // Check for cycles
        if self.loading_modules.borrow().contains(&canonical_path) {
            return Err(RuntimeError::ImportError(format!("Circular dependency detected: {}", canonical_path)));
        }

        // Mark as loading
        self.loading_modules.borrow_mut().insert(canonical_path.clone());

        // Read file
        let source_result = fs::read_to_string(&canonical_path);
        
        let source: String = match source_result {
            Ok(s) => s,
            Err(e) => {
                self.loading_modules.borrow_mut().remove(&canonical_path);
                return Err(RuntimeError::ImportError(format!("Failed to read import '{}': {}", canonical_path, e)));
            }
        };
        
        // Tokenize
        let mut lexer = Lexer::new(&source);
        let tokens: Vec<_> = match lexer.tokenize() {
            Ok(t) => t,
            Err(e) => {
                self.loading_modules.borrow_mut().remove(&canonical_path);
                return Err(RuntimeError::from(e));
            }
        };
        
        // Parse
        let mut parser = Parser::new(tokens);
        let ast = match parser.parse() {
             Ok(a) => a,
             Err(e) => {
                self.loading_modules.borrow_mut().remove(&canonical_path);
                return Err(RuntimeError::from(e));
             }
        };
        
        // Interprete with shared cache
        let mut module_interpreter = Interpreter {
            env: Environment::new(),
            is_returning: false,
            module_cache: Rc::clone(&self.module_cache),
            loading_modules: Rc::clone(&self.loading_modules),
        };
        module_interpreter.init_builtins();
        
        if let Err(e) = module_interpreter.eval_program(ast) {
             self.loading_modules.borrow_mut().remove(&canonical_path);
             return Err(e);
        }
        
        // Extract exports
        let exports = Value::Map(module_interpreter.env.get_exports());
        
        // Store in cache and remove from loading
        self.module_cache.borrow_mut().insert(canonical_path.clone(), exports.clone());
        self.loading_modules.borrow_mut().remove(&canonical_path);
        
        Ok(exports)
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
        interpreter.eval_program(ast).map_err(|e| e.to_string())
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
