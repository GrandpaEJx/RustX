use crate::ast::{BinaryOp, Expr, Stmt, UnaryOp};
use std::collections::{HashSet, HashMap};

// --- Type Inference Logic ---

#[derive(Clone, Copy, PartialEq, Debug)]
enum TypeHint {
    Unknown,
    Int,
}

struct Optimizer {
    vars: HashMap<String, TypeHint>,
    scopes: Vec<HashSet<String>>,
}

impl Optimizer {
    fn new() -> Self {
        Self {
            vars: HashMap::new(),
            scopes: vec![HashSet::new()],
        }
    }

    fn analyze(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            self.analyze_stmt(stmt);
        }
    }
    
    fn analyze_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let { name, value } => {
                let val_type = self.infer_expr(value);
                if let Some(&existing) = self.vars.get(name) {
                    if existing != val_type {
                        self.vars.insert(name.clone(), TypeHint::Unknown);
                    }
                } else {
                    self.vars.insert(name.clone(), val_type);
                }
            }
            Stmt::While { condition, body } => {
                self.analyze_expr(condition);
                if let Expr::Block(stmts) = &**body {
                    self.analyze(stmts);
                }
            }
             Stmt::For { iterator, iterable, body } => {
                let iter_type = if let Expr::Call { callee, .. } = iterable {
                     if let Expr::Ident(n) = &**callee {
                         if n == "range" { TypeHint::Int } else { TypeHint::Unknown }
                     } else { TypeHint::Unknown }
                } else { TypeHint::Unknown };
                
                self.vars.insert(iterator.clone(), iter_type);
                if let Expr::Block(stmts) = &**body {
                    self.analyze(stmts);
                }
            }
            Stmt::Function { params, body, .. } => {
                // Params are unknown unless we do inter-procedural analysis (too complex)
                for p in params { self.vars.insert(p.clone(), TypeHint::Unknown); }
                self.analyze_expr(body);
            }
            Stmt::Expr(e) | Stmt::Return(Some(e)) => self.analyze_expr(e),
            Stmt::Return(None) => {},
            Stmt::Use { .. } | Stmt::Import { .. } | Stmt::RustImport { .. } | Stmt::RustBlock { .. } => {},
        }
    }
    
    fn analyze_expr(&mut self, expr: &Expr) {
         match expr {
            Expr::Block(stmts) => self.analyze(stmts),
            Expr::If { condition, then_branch, else_branch } => {
                self.analyze_expr(condition);
                self.analyze_expr(then_branch);
                if let Some(e) = else_branch { self.analyze_expr(e); }
            }
            _ => {} // Recurse if needed, but for simple var tracking this is enough
         }
    }

    fn infer_expr(&self, expr: &Expr) -> TypeHint {
        match expr {
            Expr::Int(_) => TypeHint::Int,
            Expr::Ident(name) => *self.vars.get(name).unwrap_or(&TypeHint::Unknown),
            Expr::Binary { left, op, right } => {
                let l = self.infer_expr(left);
                let r = self.infer_expr(right);
                match op {
                    BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => {
                        if l == TypeHint::Int && r == TypeHint::Int { TypeHint::Int } else { TypeHint::Unknown }
                    }
                    _ => TypeHint::Unknown
                }
            }
            _ => TypeHint::Unknown,
        }
    }
}

/// Transpiler from RustX AST to Rust code
pub struct Transpiler {
    code: String,
    indent_level: usize,
    scopes: Vec<HashSet<String>>,
}

impl Transpiler {
    pub fn new() -> Self {
        Transpiler {
            code: String::new(),
            indent_level: 0,
            scopes: vec![HashSet::new()],
        }
    }

    pub fn transpile(&mut self, stmts: &[Stmt]) -> String {
        // 1. Collect used modules
        let used_modules = Self::collect_used_modules(stmts);
        
        // 2. Run Analysis
        let mut optimizer = Optimizer::new();
        optimizer.analyze(stmts);
        
        self.code.clear();
        self.code.push_str("use rustx_core::value::Value;\n");
        
        // Only add imports if modules are used
        if !used_modules.is_empty() {
            self.code.push_str("use std::collections::HashMap;\n");
            self.code.push_str("use std::sync::{Arc, OnceLock};\n\n");
            
            // Only declare used module statics
            for module in &used_modules {
                let mod_upper = module.to_uppercase();
                self.code.push_str(&format!("static {}: OnceLock<Value> = OnceLock::new();\n", mod_upper));
            }
            self.code.push_str("\n");
        } else {
            self.code.push_str("\n");
        }
        
        self.code.push_str("#[allow(unreachable_code)]\nfn main() -> Result<(), String> {\n");
        self.indent_level += 1;
        
        // Only initialize used modules
        if !used_modules.is_empty() {
            self.push_line("// Stdlib Init");
            for module in &used_modules {
                self.generate_module_init(module);
            }
        }

        for stmt in stmts {
            self.transpile_stmt_optimized(stmt, &optimizer.vars);
        }
        self.push_line("Ok(())");
        self.indent_level -= 1;
        self.code.push_str("}\n");
        self.code.clone()
    }
    
    fn push_line(&mut self, line: &str) {
        let indent = "    ".repeat(self.indent_level);
        self.code.push_str(&format!("{}{}\n", indent, line));
    }
    
    /// Collects all used stdlib modules from use statements
    fn collect_used_modules(stmts: &[Stmt]) -> HashSet<String> {
        let mut modules = HashSet::new();
        for stmt in stmts {
            if let Stmt::Use { module } = stmt {
                modules.insert(module.clone());
            }
        }
        modules
    }
    
    /// Generates initialization code for a specific stdlib module
    fn generate_module_init(&mut self, module: &str) {
        let mod_upper = module.to_uppercase();
        match module {
            "json" => {
                self.push_line(&format!("    let mut map = HashMap::new(); map.insert(\"parse\".to_string(), Value::NativeFunction(std::sync::Arc::new(rustx_core::stdlib::json::parse))); map.insert(\"stringify\".to_string(), Value::NativeFunction(std::sync::Arc::new(rustx_core::stdlib::json::stringify))); {}.set(Value::Map(map)).ok();", mod_upper));
            }
            "http" => {
                self.push_line(&format!("    let mut map = HashMap::new(); map.insert(\"get\".to_string(), Value::NativeFunction(std::sync::Arc::new(rustx_core::stdlib::http::get))); map.insert(\"post\".to_string(), Value::NativeFunction(std::sync::Arc::new(rustx_core::stdlib::http::post))); {}.set(Value::Map(map)).ok();", mod_upper));
            }
            "os" => {
                self.push_line(&format!("    let mut map = HashMap::new(); map.insert(\"env\".to_string(), Value::NativeFunction(std::sync::Arc::new(rustx_core::stdlib::os::env))); map.insert(\"args\".to_string(), Value::NativeFunction(std::sync::Arc::new(rustx_core::stdlib::os::args))); {}.set(Value::Map(map)).ok();", mod_upper));
            }
            "time" => {
                self.push_line(&format!("    let mut map = HashMap::new(); map.insert(\"now\".to_string(), Value::NativeFunction(std::sync::Arc::new(rustx_core::stdlib::time::now))); map.insert(\"sleep\".to_string(), Value::NativeFunction(std::sync::Arc::new(rustx_core::stdlib::time::sleep))); {}.set(Value::Map(map)).ok();", mod_upper));
            }
            "web" => {
                self.push_line(&format!("    let mut map = HashMap::new(); map.insert(\"app\".to_string(), Value::NativeFunction(std::sync::Arc::new(rustx_core::stdlib::web::app))); map.insert(\"json\".to_string(), Value::NativeFunction(std::sync::Arc::new(rustx_core::stdlib::web::json))); {}.set(Value::Map(map)).ok();", mod_upper));
            }
            "fs" => {
                self.push_line(&format!("    let mut map = HashMap::new(); map.insert(\"read\".to_string(), Value::NativeFunction(std::sync::Arc::new(rustx_core::stdlib::fs::read))); map.insert(\"write\".to_string(), Value::NativeFunction(std::sync::Arc::new(rustx_core::stdlib::fs::write))); {}.set(Value::Map(map)).ok();", mod_upper));
            }
            "term" => {
                self.push_line(&format!("    let mut map = HashMap::new(); map.insert(\"clear\".to_string(), Value::NativeFunction(std::sync::Arc::new(rustx_core::stdlib::term::clear))); map.insert(\"red\".to_string(), Value::NativeFunction(std::sync::Arc::new(rustx_core::stdlib::term::red))); {}.set(Value::Map(map)).ok();", mod_upper));
            }
            _ => {}
        }
    }
    fn enter_scope(&mut self) { self.scopes.push(HashSet::new()); }
    fn exit_scope(&mut self) { self.scopes.pop(); }
    fn is_declared(&self, name: &str) -> bool {
        for scope in self.scopes.iter().rev() { if scope.contains(name) { return true; } } false
    }
    fn declare(&mut self, name: String) { if let Some(scope) = self.scopes.last_mut() { scope.insert(name); } }

    fn transpile_stmt_optimized(&mut self, stmt: &Stmt, vars: &HashMap<String, TypeHint>) {
         match stmt {
             Stmt::Let { name, value } => {
                 let is_optimized = matches!(vars.get(name), Some(TypeHint::Int));
                 if is_optimized {
                     let val_code = self.transpile_expr_optimized(value, vars, true);
                     if self.is_declared(name) {
                         self.push_line(&format!("{} = {};", name, val_code));
                     } else {
                         self.declare(name.clone());
                         self.push_line(&format!("let mut {}: i64 = {};", name, val_code));
                     }
                 } else {
                     let val_code = self.transpile_expr_optimized(value, vars, false);
                     if self.is_declared(name) {
                         self.push_line(&format!("{} = {};", name, val_code));
                     } else {
                         self.declare(name.clone());
                         self.push_line(&format!("let mut {} = {};", name, val_code));
                     }
                 }
             }
             Stmt::While { condition, body } => {
                 let is_native_cond = self.is_native_bool_expr(condition, vars);
                 let cond_str = if is_native_cond {
                     self.transpile_native_bool(condition, vars)
                 } else {
                     let c = self.transpile_expr_optimized(condition, vars, false);
                     format!("{}.is_truthy()", c)
                 };
                 
                 self.push_line(&format!("while {} {{", cond_str));
                 self.indent_level += 1;
                 
                 if let Expr::Block(stmts) = &**body {
                     for s in stmts { self.transpile_stmt_optimized(s, vars); }
                 }
                 
                 self.indent_level -= 1;
                 self.push_line("}");
             }
             Stmt::Expr(expr) => {
                 let expr_code = self.transpile_expr_optimized(expr, vars, false);
                 self.push_line(&format!("{};", expr_code));
             }
             Stmt::Return(opt_expr) => {
                if let Some(expr) = opt_expr {
                    let code = self.transpile_expr_optimized(expr, vars, false);
                    self.push_line(&format!("return Ok({});", code));
                } else {
                    self.push_line("return Ok(Value::Null);");
                }
             }
             Stmt::Use { .. } => {
                 // No-op: Use statements are handled during initialization
                 // Modules are tracked and initialized at the top of main()
             }
             Stmt::Import { .. } => {
                self.push_line("// Import ignored in transpiler");
             }
             Stmt::RustImport { crate_name, alias, .. } => {
                let safe_crate_name = crate_name.replace('-', "_");
                if let Some(alias_name) = alias {
                     self.push_line(&format!("use {} as {};", safe_crate_name, alias_name));
                } else {
                     self.push_line(&format!("use {};", safe_crate_name));
                }
             }
             Stmt::RustBlock { code } => {
                self.push_line("/* Embedded Rust Block */");
                for line in code.lines() { self.push_line(line.trim()); }
             }
             Stmt::Function { name, params, body } => {
                 self.push_line(&format!("fn {}(args: Vec<Value>) -> Result<Value, String> {{", name));
                 self.indent_level += 1;
                 self.enter_scope();
                 if !params.is_empty() {
                     self.push_line(&format!("if args.len() != {} {{ return Err(format!(\"Expected {} arguments, got {{}}\", args.len())); }}", params.len(), params.len()));
                     for (i, param) in params.iter().enumerate() {
                         self.declare(param.clone());
                         self.push_line(&format!("let {} = args[{}].clone();", param, i));
                     }
                 }
                 let body_code = self.transpile_expr_optimized(body, vars, false);
                 self.push_line(&format!("Ok({})", body_code));
                 self.exit_scope();
                 self.indent_level -= 1;
                 self.push_line("}");
                 self.declare(name.clone());
                 self.push_line(&format!("let {} = Value::NativeFunction(std::sync::Arc::new({}));", name, name));
             }
             Stmt::For { iterator, iterable, body } => {
                 // For loop always iterates over Value::Array (or Range which returns Array currently).
                 // We could optimize `range(a,b)` to native loop?
                 // But `Value::range` produces an Array.
                 // Optimization: If iterable is range(..), and we just need iterator...
                 // Too complex for now. Standard logic.
                 let iter_code = self.transpile_expr_optimized(iterable, vars, false);
                 self.push_line(&format!("for {}_ref in {}.as_array().unwrap().iter() {{", iterator, iter_code));
                 self.indent_level += 1;
                 self.enter_scope();
                 self.declare(iterator.clone());
                 self.push_line(&format!("let mut {} = {}_ref.clone();", iterator, iterator));
                   if let Expr::Block(stmts) = &**body {
                       for s in stmts { self.transpile_stmt_optimized(s, vars); }
                   }
                 self.exit_scope();
                 self.indent_level -= 1;
                 self.push_line("}");
             }
         }
    }
    
    fn is_int_expr(&self, expr: &Expr, vars: &HashMap<String, TypeHint>) -> bool {
        match expr {
            Expr::Int(_) => true,
            Expr::Ident(n) => matches!(vars.get(n), Some(TypeHint::Int)),
            Expr::Binary { left, .. } => self.is_int_expr(left, vars), // Simplified: Assume if left is int, right must be implicit or checked before calling this? No.
            // Correct logic:
            // Expr::Binary needs Both?
            // Actually `infer_expr` logic in Optimizer did this.
            _ => false 
        }
    }
    
    fn is_native_bool_expr(&self, expr: &Expr, vars: &HashMap<String, TypeHint>) -> bool {
        match expr {
             Expr::Binary { left, op, right } => {
                 let l_int = matches!(self.infer_expr_opt(left, vars), TypeHint::Int);
                 let r_int = matches!(self.infer_expr_opt(right, vars), TypeHint::Int);
                 if l_int && r_int {
                     matches!(op, BinaryOp::Lt | BinaryOp::Gt | BinaryOp::LtEq | BinaryOp::GtEq | BinaryOp::Eq | BinaryOp::NotEq)
                 } else { false }
             }
             _ => false
        }
    }

    // Helper reusing Optimizer Logic (duplicated to avoid ownership issues or just use match)
    fn infer_expr_opt(&self, expr: &Expr, vars: &HashMap<String, TypeHint>) -> TypeHint {
        match expr {
            Expr::Int(_) => TypeHint::Int,
            Expr::Ident(name) => *vars.get(name).unwrap_or(&TypeHint::Unknown),
            Expr::Binary { left, op, right } => {
                 let l = self.infer_expr_opt(left, vars);
                 let r = self.infer_expr_opt(right, vars);
                 match op {
                    BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => {
                        if l == TypeHint::Int && r == TypeHint::Int { TypeHint::Int } else { TypeHint::Unknown }
                    }
                    _ => TypeHint::Unknown 
                }
            }
            _ => TypeHint::Unknown
        }
    }

    fn transpile_expr_optimized(&mut self, expr: &Expr, vars: &HashMap<String, TypeHint>, expect_native: bool) -> String {
        let is_native_int = matches!(self.infer_expr_opt(expr, vars), TypeHint::Int);
        
        if is_native_int {
            // Emitting Native Rust Code
            let native_str = self.transpile_native_int(expr, vars);
            if expect_native {
                native_str
            } else {
                format!("Value::Int({})", native_str)
            }
        } else {
            // Emitting Value Code
            let val_str = self.transpile_value_expr(expr, vars);
            if expect_native {
                format!("{}.as_int()?", val_str)
            } else {
                val_str
            }
        }
    }
    
    fn transpile_native_int(&mut self, expr: &Expr, vars: &HashMap<String, TypeHint>) -> String {
        match expr {
            Expr::Int(n) => format!("{}", n),
            Expr::Ident(name) => name.clone(),
            Expr::Binary { left, op, right } => {
                let l = self.transpile_expr_optimized(left, vars, true);
                let r = self.transpile_expr_optimized(right, vars, true);
                match op {
                    BinaryOp::Add => format!("({} + {})", l, r),
                    BinaryOp::Sub => format!("({} - {})", l, r),
                    BinaryOp::Mul => format!("({} * {})", l, r),
                    BinaryOp::Div => format!("({} / {})", l, r),
                    BinaryOp::Mod => format!("({} % {})", l, r),
                    _ => format!("/* Error: Non-Int Op in Native path */")
                }
            }
            _ => format!("/* Unreachable Native */")
        }
    }

    fn transpile_native_bool(&mut self, expr: &Expr, vars: &HashMap<String, TypeHint>) -> String {
        match expr {
             Expr::Binary { left, op, right } => {
                 let l = self.transpile_expr_optimized(left, vars, true);
                 let r = self.transpile_expr_optimized(right, vars, true);
                 match op {
                     BinaryOp::Lt => format!("({} < {})", l, r),
                     BinaryOp::Gt => format!("({} > {})", l, r),
                     BinaryOp::LtEq => format!("({} <= {})", l, r),
                     BinaryOp::GtEq => format!("({} >= {})", l, r),
                     BinaryOp::Eq => format!("({} == {})", l, r),
                     BinaryOp::NotEq => format!("({} != {})", l, r),
                     _ => "false".to_string() 
                 }
             }
             _ => "false".to_string()
        }
    }

    fn transpile_value_expr(&mut self, expr: &Expr, vars: &HashMap<String, TypeHint>) -> String {
        // This is the fallback for non-optimized expressions (Strings, Objects, Mixed ops)
         match expr {
            Expr::Int(n) => format!("Value::Int({})", n),
            Expr::Float(f) => format!("Value::Float({:?})", f),
            Expr::String(s) => format!("Value::String({:?}.to_string())", s),
            Expr::Bool(b) => format!("Value::Bool({})", b),
            Expr::Null => "Value::Null".to_string(),
            Expr::Ident(name) => {
                if self.is_declared(name) {
                    format!("{}.clone()", name)
                } else {
                    match name.as_str() {
                        "json" => "JSON.get().unwrap().clone()".to_string(),
                        "http" => "HTTP.get().unwrap().clone()".to_string(),
                        "os" => "OS.get().unwrap().clone()".to_string(),
                        "time" => "TIME.get().unwrap().clone()".to_string(),
                        "web" => "WEB.get().unwrap().clone()".to_string(),
                        _ => name.clone()
                    }
                }
            },
            Expr::Binary { left, op, right } => {
                // If we are here, it means it's NOT (Int op Int).
                // It could be (Int op String) or (Value op Value).
                let l = self.transpile_expr_optimized(left, vars, false);
                let r = self.transpile_expr_optimized(right, vars, false);
                match op {
                    BinaryOp::Add => format!("{}.add(&{})?", l, r),
                    BinaryOp::Sub => format!("{}.sub(&{})?", l, r),
                    BinaryOp::Mul => format!("{}.mul(&{})?", l, r),
                    BinaryOp::Div => format!("{}.div(&{})?", l, r),
                    BinaryOp::Mod => format!("{}.rem(&{})?", l, r),
                    BinaryOp::Eq => format!("{}.eq_op(&{})?", l, r),
                    BinaryOp::NotEq => format!("{}.neq_op(&{})?", l, r),
                    BinaryOp::Lt => format!("{}.lt(&{})?", l, r),
                    BinaryOp::Gt => format!("{}.gt(&{})?", l, r),
                    BinaryOp::LtEq => format!("{}.le(&{})?", l, r),
                    BinaryOp::GtEq => format!("{}.ge(&{})?", l, r),
                    BinaryOp::And => format!("{}.logic_and(&{})?", l, r),
                    BinaryOp::Or => format!("{}.logic_or(&{})?", l, r),
                }
            }
            Expr::Unary { op, expr } => {
                let e = self.transpile_expr_optimized(expr, vars, false);
                match op {
                    UnaryOp::Not => format!("{}.not()?", e),
                    UnaryOp::Neg => format!("{}.neg()?", e),
                }
            }
            Expr::Block(stmts) => {
                let mut output = String::new();
                output.push_str("{\n");
                self.indent_level += 1;
                self.enter_scope();
                
                 // Inline logic of transpile_block_body here to use optimized logic
                 let len = stmts.len();
                 if len == 0 {
                    output.push_str("    ".repeat(self.indent_level).as_str());
                    output.push_str("Value::Null\n");
                 } else {
                     // We need to capture the output of push_line calls.
                     // Temporarily swap self.code? complex.
                     // Just use a temporary transpiler? No, scopes.
                     // I will just append to output string manually?
                     // My `push_line` appends to `self.code`.
                     // I'll assume `Expr::Block` is rare in hot paths or handled via `transpile_block_body` which uses `transpile_stmt` (which I routed to proper one).
                     // Wait, `transpile_block_body` calls `transpile_stmt`.
                     // I need `transpile_block_body` to call `transpile_stmt_optimized`.
                     // I'll implement `transpile_block_body_optimized`.
                     let body = self.transpile_block_body_optimized(stmts, true, vars);
                     output.push_str(&body);
                 }
                 
                self.exit_scope();
                self.indent_level -= 1;
                output.push_str(&format!("{}}}", "    ".repeat(self.indent_level)));
                output
            }
            Expr::TemplateString(s) => {
                let mut format_str = String::new();
                 let mut args = Vec::new();
                 let mut chars = s.chars().peekable();
                 
                 while let Some(ch) = chars.next() {
                     if ch == '{' {
                         if let Some(&next) = chars.peek() {
                             if next == '{' {
                                 format_str.push_str("{{");
                                 chars.next(); 
                             } else {
                                 let mut var_name = String::new();
                                 while let Some(&c) = chars.peek() {
                                     if c == '}' { chars.next(); break; }
                                     var_name.push(chars.next().unwrap());
                                 }
                                 if !var_name.is_empty() {
                                     format_str.push_str("{}");
                                     args.push(var_name);
                                 }
                             }
                         } else {
                              format_str.push('{');
                         }
                     } else if ch == '}' {
                          if let Some(&next) = chars.peek() {
                             if next == '}' { format_str.push_str("}}"); chars.next(); } else { format_str.push('}'); }
                          } else { format_str.push('}'); }
                     } else {
                         format_str.push(ch);
                     }
                 }
                 
                 if args.is_empty() {
                     format!("Value::String(\"{}\".to_string())", format_str)
                 } else {
                     let args = args.iter().map(|a| self.transpile_expr_optimized(&Expr::Ident(a.clone()), vars, false)).collect::<Vec<_>>().join(", ");
                     format!("Value::String(format!(\"{}\", {}))", format_str, args)
                 }
            },
            Expr::Array(items) => {
                let items_code: Vec<String> = items.iter().map(|i| self.transpile_expr_optimized(i, vars, false)).collect();
                format!("Value::Array(vec![{}])", items_code.join(", "))
            },
            Expr::Map(entries) => {
                 let mut inserts = Vec::new();
                 for (key, val) in entries {
                      let val_code = self.transpile_expr_optimized(val, vars, false);
                      inserts.push(format!("map.insert(\"{}\".to_string(), {});", key, val_code));
                 }
                 format!("Value::Map({{ let mut map = HashMap::new(); {} map }})", inserts.join(" "))
            },
            Expr::Index { object, index } => {
                 let obj_code = self.transpile_expr_optimized(object, vars, false);
                 let idx_code = self.transpile_expr_optimized(index, vars, false);
                 format!("{}.get_index(&{})?", obj_code, idx_code)
            },
            Expr::MethodCall { object, method, args } => {
                let obj_code = self.transpile_expr_optimized(object, vars, false);
                let args_code: Vec<String> = args.iter().map(|a| self.transpile_expr_optimized(a, vars, false)).collect();
                let args_str = if args_code.is_empty() { "vec![]".to_string() } else { format!("vec![{}]", args_code.join(", ")) };
                format!("{}.call_method(\"{}\", {})?", obj_code, method, args_str)
            },
             Expr::Call { callee, args } => {
                 let args_code: Vec<String> = args.iter().map(|a| self.transpile_expr_optimized(a, vars, false)).collect();
                 
                 if let Expr::Ident(name) = &**callee {
                    match name.as_str() {
                        "print" => {
                            let prints = args_code.iter().map(|a| format!("print!(\"{{}} \", {});", a)).collect::<Vec<_>>().join("\n");
                            return format!("{{ {} println!(); Value::Null }}", prints);
                        },
                         "input" => {
                             if args_code.is_empty() {
                                 return "rustx_core::stdlib::io::input(Value::Null)".to_string();
                             } else {
                                 return format!("rustx_core::stdlib::io::input({})", args_code[0]);
                             }
                        },
                        "len" => {
                            if args_code.len() == 1 {
                                return format!("{}.len().map(|l| Value::Int(l as i64))?", args_code[0]);
                            }
                        },
                        _ => {
                            if !self.is_declared(name) {
                                // Native Rust Function Call (e.g. from rust {} block)
                                let args_str = args_code.join(", ");
                                return format!("{}({})?", name, args_str);
                            }
                        }
                    }
                 }
                 
                 // Generic Value Call
                 let callee_code = self.transpile_expr_optimized(callee, vars, false);
                 let args_str = if args_code.is_empty() { "vec![]".to_string() } else { format!("vec![{}]", args_code.join(", ")) };
                 format!("{}.call({})?", callee_code, args_str)
             }
             Expr::If { condition, then_branch, else_branch } => {
                let cond = self.transpile_expr_optimized(condition, vars, false);
                let then_code = self.transpile_expr_optimized(then_branch, vars, false);
                if let Some(else_expr) = else_branch {
                    let else_code = self.transpile_expr_optimized(else_expr, vars, false);
                    format!("if {}.is_truthy() {{ {} }} else {{ {} }}", cond, then_code, else_code)
                } else {
                    format!("if {}.is_truthy() {{ {}; Value::Null }} else {{ Value::Null }}", cond, then_code)
                }
            }
             _ => "Value::Null".to_string(), // Catch-all for others
         }
    }
    
    fn transpile_block_body_optimized(&mut self, stmts: &[Stmt], return_last: bool, vars: &HashMap<String, TypeHint>) -> String {
         let old_code = std::mem::take(&mut self.code);
         let old_indent = self.indent_level;
         let len = stmts.len();
         if len == 0 {
             if return_last { self.push_line("Value::Null"); }
         } else {
             for (i, stmt) in stmts.iter().enumerate() {
                 if return_last && i == len - 1 {
                     match stmt {
                         Stmt::Expr(e) => {
                             let expr_code = self.transpile_expr_optimized(e, vars, false);
                              self.push_line(&expr_code);
                         }
                         Stmt::Return(_) => { self.transpile_stmt_optimized(stmt, vars); }
                         _ => {
                             self.transpile_stmt_optimized(stmt, vars);
                             self.push_line("Value::Null"); 
                         }
                     }
                 } else {
                     self.transpile_stmt_optimized(stmt, vars);
                 }
             }
         }
         let buf = std::mem::replace(&mut self.code, old_code);
         self.indent_level = old_indent; 
         buf
    }
}

impl Default for Transpiler { fn default() -> Self { Self::new() } }
