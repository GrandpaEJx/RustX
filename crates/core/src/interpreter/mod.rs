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
        // Core builtins
        self.env.set("print".to_string(), Value::NativeFunction(std::sync::Arc::new(|args: Vec<Value>| {
            let text = args.iter().map(|arg| arg.to_string()).collect::<Vec<_>>().join(" ");
            println!("{}", text);
            Ok(Value::Null)
        })));

        // JSON
        let mut json_mod = HashMap::new();
        json_mod.insert("parse".to_string(), Value::NativeFunction(std::sync::Arc::new(crate::stdlib::json::parse)));
        json_mod.insert("stringify".to_string(), Value::NativeFunction(std::sync::Arc::new(crate::stdlib::json::stringify)));
        self.env.set("json".to_string(), Value::Map(json_mod));
        
        // HTTP
        let mut http_mod = HashMap::new();
        http_mod.insert("get".to_string(), Value::NativeFunction(std::sync::Arc::new(crate::stdlib::http::get)));
        http_mod.insert("post".to_string(), Value::NativeFunction(std::sync::Arc::new(crate::stdlib::http::post)));
        self.env.set("http".to_string(), Value::Map(http_mod));
        
        // OS
        let mut os_mod = HashMap::new();
        os_mod.insert("env".to_string(), Value::NativeFunction(std::sync::Arc::new(crate::stdlib::os::env)));
        os_mod.insert("args".to_string(), Value::NativeFunction(std::sync::Arc::new(crate::stdlib::os::args)));
        self.env.set("os".to_string(), Value::Map(os_mod));
        
        // Time
        let mut time_mod = HashMap::new();
        time_mod.insert("now".to_string(), Value::NativeFunction(std::sync::Arc::new(crate::stdlib::time::now)));
        time_mod.insert("sleep".to_string(), Value::NativeFunction(std::sync::Arc::new(crate::stdlib::time::sleep)));
        self.env.set("time".to_string(), Value::Map(time_mod));
        
        // Web
        let mut web_mod = HashMap::new();
        web_mod.insert("app".to_string(), Value::NativeFunction(std::sync::Arc::new(crate::stdlib::web::app)));
        web_mod.insert("json".to_string(), Value::NativeFunction(std::sync::Arc::new(crate::stdlib::web::json)));
        self.env.set("web".to_string(), Value::Map(web_mod));

        // FS
        let mut fs_mod = HashMap::new();
        fs_mod.insert("read".to_string(), Value::NativeFunction(std::sync::Arc::new(crate::stdlib::fs::read)));
        fs_mod.insert("write".to_string(), Value::NativeFunction(std::sync::Arc::new(crate::stdlib::fs::write)));
        fs_mod.insert("append".to_string(), Value::NativeFunction(std::sync::Arc::new(crate::stdlib::fs::append)));
        fs_mod.insert("exists".to_string(), Value::NativeFunction(std::sync::Arc::new(crate::stdlib::fs::exists)));
        fs_mod.insert("remove".to_string(), Value::NativeFunction(std::sync::Arc::new(crate::stdlib::fs::remove)));
        self.env.set("fs".to_string(), Value::Map(fs_mod));

        // Term
        let mut term_mod = HashMap::new();
        term_mod.insert("red".to_string(), Value::NativeFunction(std::sync::Arc::new(crate::stdlib::term::red)));
        term_mod.insert("green".to_string(), Value::NativeFunction(std::sync::Arc::new(crate::stdlib::term::green)));
        term_mod.insert("blue".to_string(), Value::NativeFunction(std::sync::Arc::new(crate::stdlib::term::blue)));
        term_mod.insert("yellow".to_string(), Value::NativeFunction(std::sync::Arc::new(crate::stdlib::term::yellow)));
        term_mod.insert("cyan".to_string(), Value::NativeFunction(std::sync::Arc::new(crate::stdlib::term::cyan)));
        term_mod.insert("magenta".to_string(), Value::NativeFunction(std::sync::Arc::new(crate::stdlib::term::magenta)));
        term_mod.insert("white".to_string(), Value::NativeFunction(std::sync::Arc::new(crate::stdlib::term::white)));
        term_mod.insert("bg_red".to_string(), Value::NativeFunction(std::sync::Arc::new(crate::stdlib::term::bg_red)));
        term_mod.insert("bg_green".to_string(), Value::NativeFunction(std::sync::Arc::new(crate::stdlib::term::bg_green)));
        term_mod.insert("bg_blue".to_string(), Value::NativeFunction(std::sync::Arc::new(crate::stdlib::term::bg_blue)));
        term_mod.insert("bg_yellow".to_string(), Value::NativeFunction(std::sync::Arc::new(crate::stdlib::term::bg_yellow)));
        term_mod.insert("bg_cyan".to_string(), Value::NativeFunction(std::sync::Arc::new(crate::stdlib::term::bg_cyan)));
        term_mod.insert("bg_magenta".to_string(), Value::NativeFunction(std::sync::Arc::new(crate::stdlib::term::bg_magenta)));
        term_mod.insert("bg_white".to_string(), Value::NativeFunction(std::sync::Arc::new(crate::stdlib::term::bg_white)));
        term_mod.insert("bold".to_string(), Value::NativeFunction(std::sync::Arc::new(crate::stdlib::term::bold)));
        term_mod.insert("dim".to_string(), Value::NativeFunction(std::sync::Arc::new(crate::stdlib::term::dim)));
        term_mod.insert("italic".to_string(), Value::NativeFunction(std::sync::Arc::new(crate::stdlib::term::italic)));
        term_mod.insert("clear".to_string(), Value::NativeFunction(std::sync::Arc::new(crate::stdlib::term::clear)));
        term_mod.insert("underline".to_string(), Value::NativeFunction(std::sync::Arc::new(crate::stdlib::term::underline)));
        self.env.set("term".to_string(), Value::Map(term_mod));
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
            Value::NativeFunction(f) => {
                 f(args).map_err(RuntimeError::from)
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
