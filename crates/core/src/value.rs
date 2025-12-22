use std::collections::HashMap;
use std::fmt;

use std::sync::Arc;

/// Runtime value types
#[derive(Clone)]
pub enum Value {
    Null,
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Array(Vec<Value>),
    Map(HashMap<String, Value>),
    Function {
        params: Vec<String>,
        body: crate::ast::Expr,
    },
    NativeFunction(Arc<dyn Fn(Vec<Value>) -> Result<Value, String> + Send + Sync>),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
             // For simplicity, delegate to Display or standard debug where possible
             Value::Null => write!(f, "Null"),
             Value::Int(n) => write!(f, "Int({})", n),
             Value::Float(n) => write!(f, "Float({})", n),
             Value::Bool(b) => write!(f, "Bool({})", b),
             Value::String(s) => write!(f, "String({:?})", s),
             Value::Array(a) => write!(f, "Array({:?})", a),
             Value::Map(m) => write!(f, "Map({:?})", m),
             Value::Function { params, body: _ } => write!(f, "Function({:?})", params),
             Value::NativeFunction(_) => write!(f, "NativeFunction"),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Null, Value::Null) => true,
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Array(a), Value::Array(b)) => a == b,
            (Value::Map(a), Value::Map(b)) => a == b,
            (Value::Function { params: pa, body: ba }, Value::Function { params: pb, body: bb }) => pa == pb && ba == bb,
            (Value::NativeFunction(a), Value::NativeFunction(b)) => Arc::ptr_eq(a, b),
            _ => false,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Int(n) => write!(f, "{}", n),
            Value::Float(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
            Value::String(s) => write!(f, "{}", s),
            Value::Array(arr) => {
                write!(f, "[")?;
                for (i, v) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            Value::Map(map) => {
                write!(f, "{{")?;
                for (i, (k, v)) in map.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "\"{}\": {}", k, v)?;
                }
                write!(f, "}}")
            }
            Value::Function { .. } => write!(f, "<function>"),
            Value::NativeFunction(_) => write!(f, "<native_function>"),
        }
    }
}

impl Value {
    /// Converts value to boolean for conditional evaluation
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Null => false,
            Value::Bool(b) => *b,
            Value::Int(n) => *n != 0,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Array(a) => !a.is_empty(),
            Value::Map(m) => !m.is_empty(),
            Value::Function { .. } => true,
            Value::NativeFunction(_) => true,
        }
    }

    /// Creates a range array
    pub fn range(start: i64, end: i64, step: i64) -> Result<Value, String> {
        if step == 0 {
             return Err("step cannot be zero".to_string());
        }
        let mut arr = Vec::new();
        if step > 0 {
            let mut i = start;
            while i < end {
                arr.push(Value::Int(i));
                i += step;
            }
        } else {
            let mut i = start;
            while i > end {
                arr.push(Value::Int(i));
                i += step;
            }
        }
        Ok(Value::Array(arr))
    }

    /// Attempts to convert value to i64
    pub fn as_int(&self) -> Result<i64, String> {
        match self {
            Value::Int(n) => Ok(*n),
            Value::Float(f) => Ok(*f as i64),
            _ => Err(format!("Cannot convert {} to integer", self)),
        }
    }

    /// Attempts to convert value to f64
    pub fn as_float(&self) -> Result<f64, String> {
        match self {
            Value::Int(n) => Ok(*n as f64),
            Value::Float(f) => Ok(*f),
            _ => Err(format!("Cannot convert {} to float", self)),
        }
    }

    /// Access as array
    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Array(arr) => Some(arr),
            _ => None,
        }
    }

    /// Access as mutable array
    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Value>> {
        match self {
            Value::Array(arr) => Some(arr),
            _ => None,
        }
    }
    
    // === Type Info ===
    
    pub fn type_name(&self) -> String {
        match self {
            Value::Int(_) => "int",
            Value::Float(_) => "float",
            Value::String(_) => "string",
            Value::Bool(_) => "bool",
            Value::Array(_) => "array",
            Value::Map(_) => "map",
            Value::Function { .. } => "function",
            Value::NativeFunction(_) => "native_function",
            Value::Null => "null",
        }.to_string()
    }
    
    pub fn len(&self) -> Result<i64, String> {
        match self {
            Value::Array(arr) => Ok(arr.len() as i64),
            Value::String(s) => Ok(s.len() as i64),
            Value::Map(map) => Ok(map.len() as i64),
            _ => Err(format!("Type {} does not support len()", self.type_name())),
        }
    }

    pub fn is_empty(&self) -> Result<bool, String> {
        self.len().map(|l| l == 0)
    }
    
    // === Array Methods ===
    
    pub fn push(&mut self, value: Value) -> Result<Value, String> {
        match self {
            Value::Array(arr) => {
                arr.push(value);
                Ok(Value::Array(arr.clone()))
            },
            _ => Err(format!("Type {} does not support push()", self.type_name())),
        }
    }

    pub fn pop(&mut self) -> Result<Value, String> {
        match self {
            Value::Array(arr) => {
                arr.pop().ok_or_else(|| "Cannot pop from empty array".to_string())
            },
            _ => Err(format!("Type {} does not support pop()", self.type_name())),
        }
    }
    
    pub fn reverse_in_place(&mut self) -> Result<Value, String> {
         match self {
            Value::Array(arr) => {
                arr.reverse();
                Ok(Value::Array(arr.clone()))
            },
            _ => Err(format!("Type {} does not support reverse()", self.type_name())),
        }
    }
    
    pub fn sort_in_place(&mut self) -> Result<Value, String> {
         match self {
            Value::Array(arr) => {
                arr.sort_by(|a, b| {
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => x.cmp(y),
                        (Value::Float(x), Value::Float(y)) => x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal),
                        (Value::String(x), Value::String(y)) => x.cmp(y),
                         _ => format!("{:?}", a).cmp(&format!("{:?}", b)),
                    }
                });
                Ok(Value::Array(arr.clone()))
            },
            _ => Err(format!("Type {} does not support sort()", self.type_name())),
        }
    }
    
    // === String Methods ===
    
    pub fn split(&self, delimiter: &Value) -> Result<Value, String> {
         match (self, delimiter) {
            (Value::String(s), Value::String(d)) => {
                let parts: Vec<Value> = s.split(d)
                    .map(|p| Value::String(p.to_string()))
                    .collect();
                Ok(Value::Array(parts))
            },
            (Value::String(_), _) => Err("split requires string delimiter".to_string()),
            _ => Err(format!("Type {} does not support split()", self.type_name())),
        }
    }
    
    pub fn trim(&self) -> Result<Value, String> {
        match self {
            Value::String(s) => Ok(Value::String(s.trim().to_string())),
             _ => Err(format!("Type {} does not support trim()", self.type_name())),
        }
    }
    
    pub fn upper(&self) -> Result<Value, String> {
        match self {
            Value::String(s) => Ok(Value::String(s.to_uppercase())),
             _ => Err(format!("Type {} does not support upper()", self.type_name())),
        }
    }
    
    pub fn lower(&self) -> Result<Value, String> {
        match self {
            Value::String(s) => Ok(Value::String(s.to_lowercase())),
             _ => Err(format!("Type {} does not support lower()", self.type_name())),
        }
    }

    // === Math Methods ===
    
    pub fn abs(&self) -> Result<Value, String> {
        match self {
            Value::Int(n) => Ok(Value::Int(n.abs())),
            Value::Float(f) => Ok(Value::Float(f.abs())),
            _ => Err(format!("Type {} does not support abs()", self.type_name())),
        }
    }
    
    pub fn floor(&self) -> Result<Value, String> {
        match self {
             Value::Float(f) => Ok(Value::Int(f.floor() as i64)),
             Value::Int(n) => Ok(Value::Int(*n)),
             _ => Err(format!("Type {} does not support floor()", self.type_name())),
        }
    }
    
    pub fn ceil(&self) -> Result<Value, String> {
        match self {
             Value::Float(f) => Ok(Value::Int(f.ceil() as i64)),
             Value::Int(n) => Ok(Value::Int(*n)),
             _ => Err(format!("Type {} does not support ceil()", self.type_name())),
        }
    }
    
    pub fn round(&self) -> Result<Value, String> {
        match self {
             Value::Float(f) => Ok(Value::Int(f.round() as i64)),
             Value::Int(n) => Ok(Value::Int(*n)),
             _ => Err(format!("Type {} does not support round()", self.type_name())),
        }
    }

    // Arithmetic Operations
    pub fn add(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 + b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + *b as f64)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            _ => Err(format!("Invalid operands for +: {:?} and {:?}", self, other)),
        }
    }

    pub fn sub(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 - b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a - *b as f64)),
            _ => Err(format!("Invalid operands for -: {:?} and {:?}", self, other)),
        }
    }

    pub fn mul(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 * b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a * *b as f64)),
            _ => Err(format!("Invalid operands for *: {:?} and {:?}", self, other)),
        }
    }

    pub fn div(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
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
            _ => Err(format!("Invalid operands for /: {:?} and {:?}", self, other)),
        }
    }

    pub fn rem(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a % b)),
            _ => Err(format!("Invalid operands for %: {:?} and {:?}", self, other)),
        }
    }

    // Comparison Operations
    pub fn eq_op(&self, other: &Value) -> Result<Value, String> {
        Ok(Value::Bool(self == other))
    }

    pub fn neq_op(&self, other: &Value) -> Result<Value, String> {
        Ok(Value::Bool(self != other))
    }

    pub fn lt(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a < b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool((*a as f64) < *b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(*a < (*b as f64))),
             _ => Err(format!("Invalid operands for <: {:?} and {:?}", self, other)),
        }
    }

    pub fn gt(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a > b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool((*a as f64) > *b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(*a > (*b as f64))),
             _ => Err(format!("Invalid operands for >: {:?} and {:?}", self, other)),
        }
    }

    pub fn le(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a <= b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool((*a as f64) <= *b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(*a <= (*b as f64))),
             _ => Err(format!("Invalid operands for <=: {:?} and {:?}", self, other)),
        }
    }

    pub fn ge(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a >= b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool((*a as f64) >= *b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(*a >= (*b as f64))),
             _ => Err(format!("Invalid operands for >=: {:?} and {:?}", self, other)),
        }
    }

    // Logic Operations
    pub fn logic_and(&self, other: &Value) -> Result<Value, String> {
        Ok(Value::Bool(self.is_truthy() && other.is_truthy()))
    }

    pub fn logic_or(&self, other: &Value) -> Result<Value, String> {
        Ok(Value::Bool(self.is_truthy() || other.is_truthy()))
    }

    // Unary Operations
    pub fn not(&self) -> Result<Value, String> {
         Ok(Value::Bool(!self.is_truthy()))
    }

    pub fn neg(&self) -> Result<Value, String> {
        match self {
            Value::Int(n) => Ok(Value::Int(-n)),
            Value::Float(f) => Ok(Value::Float(-f)),
            _ => Err(format!("Invalid operand for negation: {:?}", self)),
        }
    }

    /// Calls a method on the value
    pub fn call_method(&mut self, method: &str, args: Vec<Value>) -> Result<Value, String> {
        match method {
             "len" => {
                 if !args.is_empty() { return Err("len() takes no arguments".to_string()); }
                 Ok(Value::Int(self.len()?))
             }
             "push" => {
                 if args.len() != 1 { return Err("push() takes 1 argument".to_string()); }
                 self.push(args[0].clone())
             }
             "pop" => {
                 if !args.is_empty() { return Err("pop() takes no arguments".to_string()); }
                 self.pop()
             }
             "upper" => {
                 if !args.is_empty() { return Err("upper() takes no arguments".to_string()); }
                 self.upper()
             }
             "lower" => {
                 if !args.is_empty() { return Err("lower() takes no arguments".to_string()); }
                 self.lower()
             }
             "trim" => {
                 if !args.is_empty() { return Err("trim() takes no arguments".to_string()); }
                 self.trim()
             }
             "split" => {
                 if args.len() != 1 { return Err("split() takes 1 argument".to_string()); }
                 self.split(&args[0])
             }
             "abs" => {
                 if !args.is_empty() { return Err("abs() takes no arguments".to_string()); }
                 self.abs()
             }
             "floor" => {
                 if !args.is_empty() { return Err("floor() takes no arguments".to_string()); }
                 self.floor()
             }
             "ceil" => {
                 if !args.is_empty() { return Err("ceil() takes no arguments".to_string()); }
                 self.ceil()
             }
             "round" => {
                 if !args.is_empty() { return Err("round() takes no arguments".to_string()); }
                 self.round()
             }
             "reverse" => {
                 if !args.is_empty() { return Err("reverse() takes no arguments".to_string()); }
                 self.reverse_in_place()
             }
             "sort" => {
                 if !args.is_empty() { return Err("sort() takes no arguments".to_string()); }
                 self.sort_in_place()
             }
             // Add others here (reverse, sort, map, filter...) 
             // Logic for map/filter uses callbacks which is complex in shared Value struct 
             // because it requires executing the callback. `Value` doesn't have Interpreter context.
             // BUT for JIT transpiled code, the callback is a Function/NativeFunction.
             // NativeFunction can be executed. Function (AST) needs Interpreter.
             // This is the tricky part. `eval_expr` had access to `Interpreter`.
             // `Value::call_method` does NOT have access to Interpreter.
             // So `Value::call_method` cannot execute AST functions!
             // It can only execute NativeFunctions.
             // stdlib functions are NativeFunctions. So `json.parse` works.
             // But `map(fn(x) => x+1)`? Transpiled code handles closures differently?
             // Transpiler compiles AST functions to ... `Value::Function` or Rust closure?
             // Transpiler compiles `fn` to `Value::Function` currently (Step 517, line 89-107 transpiles `Stmt::Function` to Rust function, but `Expr::Function` not handled?). 
             // Actually `Stmt::Function` defines a named function.
             // Closures? Not supported yet?
             // `stdlib_test.rsx` doesn't use `map`/`filter` with callbacks.
             // It uses `json.parse` (NativeFunction).
             // So for `json.parse`, `call_method` can fallback to map lookup.
             
             _ => {
                 // Map method lookup
                 if let Value::Map(map) = self {
                     if let Some(val) = map.get(method) {
                         match val {
                             Value::NativeFunction(f) => f(args),
                             Value::Function { .. } => {
                                 // We cannot execute AST function here without Interpreter.
                                 // For Transpiler/JIT, AST functions should probably not exist in this form?
                                 // Or Transpiler should compile them to NativeFunctions (fn pointers)?
                                 // For now, return Error if trying to call AST function from JIT via call_method.
                                 Err("Cannot call interpreted function from compiled code (yet)".to_string())
                             }
                             _ => Ok(val.clone()), // Property access if not a function? No, `method(...)` implies call.
                             // But wait, `json.parse` is a look up which returns a function, then called?
                             // Expr::MethodCall syntax `obj.method(args)` implies immediate call.
                             // My fallback logic checks if it's a function.
                         }
                     } else {
                         Err(format!("Unknown method '{}'", method))
                     }
                 } else {
                     Err(format!("Type {} has no method '{}'", self.type_name(), method))
                 }
             }
        }
    }
    pub fn call(&self, args: Vec<Value>) -> Result<Value, String> {
         match self {
             Value::NativeFunction(f) => f(args),
             Value::Function { .. } => Err("Cannot call interpreted function from compiled code".to_string()),
             _ => Err(format!("Type {} is not callable", self.type_name())),
         }
    }

    pub fn get_index(&self, index: &Value) -> Result<Value, String> {
        match self {
            Value::Array(arr) => {
                 let i = index.as_int()?;
                 if i < 0 {
                     return Err("Index cannot be negative".to_string());
                 }
                 arr.get(i as usize).cloned().ok_or_else(|| "Index out of bounds".to_string())
            }
            Value::Map(map) => {
                 let key = match index {
                     Value::String(s) => s.clone(),
                     _ => return Err("Map key must be a string".to_string()),
                 };
                 map.get(&key).cloned().ok_or_else(|| format!("Key '{}' not found", key))
            }
            Value::String(s) => {
                 let i = index.as_int()?;
                 if i < 0 {
                     return Err("Index cannot be negative".to_string());
                 }
                 s.chars().nth(i as usize).map(|c| Value::String(c.to_string())).ok_or_else(|| "Index out of bounds".to_string())
            }
            _ => Err(format!("Type {} is not indexable", self.type_name())),
        }
    }
}

