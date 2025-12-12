use crate::ast::{BinaryOp, Expr, Stmt, UnaryOp};
use crate::value::Value;
use std::collections::HashMap;

/// Environment for variable storage
#[derive(Debug, Clone)]
pub struct Environment {
    scopes: Vec<HashMap<String, Value>>,
}

impl Environment {
    /// Creates a new environment
    pub fn new() -> Self {
        Environment {
            scopes: vec![HashMap::new()],
        }
    }

    /// Pushes a new scope
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// Pops the current scope
    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    /// Sets a variable in the current scope
    pub fn set(&mut self, name: String, value: Value) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, value);
        }
    }

    /// Gets a variable value
    pub fn get(&self, name: &str) -> Result<Value, String> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Ok(value.clone());
            }
        }
        Err(format!("Undefined variable: {}", name))
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

/// Interpreter for RustX
pub struct Interpreter {
    env: Environment,
}

impl Interpreter {
    /// Creates a new interpreter
    pub fn new() -> Self {
        Interpreter {
            env: Environment::new(),
        }
    }

    /// Evaluates a program (list of statements)
    pub fn eval_program(&mut self, statements: Vec<Stmt>) -> Result<Value, String> {
        let mut last_value = Value::Null;

        for stmt in statements {
            last_value = self.eval_stmt(stmt)?;
        }

        Ok(last_value)
    }

    /// Evaluates a statement
    fn eval_stmt(&mut self, stmt: Stmt) -> Result<Value, String> {
        match stmt {
            Stmt::Expr(expr) => self.eval_expr(expr),
            Stmt::Let { name, value } => {
                let val = self.eval_expr(value)?;
                self.env.set(name, val.clone());
                Ok(val)
            }
            Stmt::Function { name, params, body } => {
                let func = Value::Function {
                    params,
                    body: *body,
                };
                self.env.set(name, func.clone());
                Ok(func)
            }
            Stmt::Return(expr) => {
                if let Some(e) = expr {
                    self.eval_expr(e)
                } else {
                    Ok(Value::Null)
                }
            }
            Stmt::While { condition, body } => {
                let mut last_value = Value::Null;
                while self.eval_expr(condition.clone())?.is_truthy() {
                    last_value = self.eval_expr(*body.clone())?;
                }
                Ok(last_value)
            }
            Stmt::For {
                iterator,
                iterable,
                body,
            } => {
                let iter_value = self.eval_expr(iterable)?;
                let mut last_value = Value::Null;

                match iter_value {
                    Value::Array(arr) => {
                        for item in arr {
                            self.env.push_scope();
                            self.env.set(iterator.clone(), item);
                            last_value = self.eval_expr(*body.clone())?;
                            self.env.pop_scope();
                        }
                    }
                    _ => return Err("For loop requires an array".to_string()),
                }

                Ok(last_value)
            }
            Stmt::Import { .. } => {
                // Import system not yet implemented
                Ok(Value::Null)
            }
        }
    }

    /// Evaluates an expression
    fn eval_expr(&mut self, expr: Expr) -> Result<Value, String> {
        match expr {
            Expr::Int(n) => Ok(Value::Int(n)),
            Expr::Float(f) => Ok(Value::Float(f)),
            Expr::String(s) => Ok(Value::String(s)),
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
                self.env.set(name, val.clone());
                Ok(val)
            }
        }
    }

    /// Evaluates a binary operation
    fn eval_binary(&mut self, left: Expr, op: BinaryOp, right: Expr) -> Result<Value, String> {
        let left_val = self.eval_expr(left)?;
        let right_val = self.eval_expr(right)?;

        match op {
            BinaryOp::Add => match (&left_val, &right_val) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 + b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + *b as f64)),
                (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
                _ => Err("Invalid operands for +".to_string()),
            },
            BinaryOp::Sub => match (&left_val, &right_val) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 - b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a - *b as f64)),
                _ => Err("Invalid operands for -".to_string()),
            },
            BinaryOp::Mul => match (&left_val, &right_val) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 * b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a * *b as f64)),
                _ => Err("Invalid operands for *".to_string()),
            },
            BinaryOp::Div => match (&left_val, &right_val) {
                (Value::Int(a), Value::Int(b)) => {
                    if *b == 0 {
                        Err("Division by zero".to_string())
                    } else {
                        Ok(Value::Int(a / b))
                    }
                }
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 / b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a / *b as f64)),
                _ => Err("Invalid operands for /".to_string()),
            },
            BinaryOp::Mod => match (&left_val, &right_val) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a % b)),
                _ => Err("Invalid operands for %".to_string()),
            },
            BinaryOp::Eq => Ok(Value::Bool(left_val == right_val)),
            BinaryOp::NotEq => Ok(Value::Bool(left_val != right_val)),
            BinaryOp::Lt => match (&left_val, &right_val) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a < b)),
                _ => Err("Invalid operands for <".to_string()),
            },
            BinaryOp::Gt => match (&left_val, &right_val) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a > b)),
                _ => Err("Invalid operands for >".to_string()),
            },
            BinaryOp::LtEq => match (&left_val, &right_val) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a <= b)),
                _ => Err("Invalid operands for <=".to_string()),
            },
            BinaryOp::GtEq => match (&left_val, &right_val) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a >= b)),
                _ => Err("Invalid operands for >=".to_string()),
            },
            BinaryOp::And => Ok(Value::Bool(left_val.is_truthy() && right_val.is_truthy())),
            BinaryOp::Or => Ok(Value::Bool(left_val.is_truthy() || right_val.is_truthy())),
        }
    }

    /// Evaluates a unary operation
    fn eval_unary(&mut self, op: UnaryOp, expr: Expr) -> Result<Value, String> {
        let val = self.eval_expr(expr)?;

        match op {
            UnaryOp::Not => Ok(Value::Bool(!val.is_truthy())),
            UnaryOp::Neg => match val {
                Value::Int(n) => Ok(Value::Int(-n)),
                Value::Float(f) => Ok(Value::Float(-f)),
                _ => Err("Invalid operand for negation".to_string()),
            },
        }
    }

    /// Evaluates a function call
    fn eval_call(&mut self, callee: Expr, args: Vec<Expr>) -> Result<Value, String> {
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

    /// Evaluates index access
    fn eval_index(&mut self, object: Expr, index: Expr) -> Result<Value, String> {
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
