use crate::ast::{BinaryOp, Expr, Stmt, UnaryOp};
use std::collections::HashSet;

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
            scopes: vec![HashSet::new()], // Global scope
        }
    }
}

impl Default for Transpiler {
    fn default() -> Self {
        Self::new()
    }
}

impl Transpiler {
    pub fn transpile(&mut self, stmts: &[Stmt]) -> String {
        self.code.clear();
        self.code.push_str("use rustx_core::value::Value;\n");
        self.code.push_str("use std::collections::HashMap;\n");
        self.code.push_str("use std::sync::{Arc, OnceLock};\n\n");

        self.code.push_str("static JSON: OnceLock<Value> = OnceLock::new();\n");
        self.code.push_str("static HTTP: OnceLock<Value> = OnceLock::new();\n");
        self.code.push_str("static OS: OnceLock<Value> = OnceLock::new();\n");
        self.code.push_str("static TIME: OnceLock<Value> = OnceLock::new();\n");
        self.code.push_str("static WEB: OnceLock<Value> = OnceLock::new();\n\n");

        self.code.push_str("#[allow(unreachable_code)]\nfn main() -> Result<(), String> {\n");
        self.indent_level += 1;
        
        // Initialize Standard Library
        self.push_line("// Stdlib Init");
        
        // JSON
        self.push_line("    let mut map = HashMap::new();");
        self.push_line("    map.insert(\"parse\".to_string(), Value::NativeFunction(std::sync::Arc::new(rustx_core::stdlib::json::parse)));");
        self.push_line("    map.insert(\"stringify\".to_string(), Value::NativeFunction(std::sync::Arc::new(rustx_core::stdlib::json::stringify)));");
        self.push_line("    JSON.set(Value::Map(map)).ok();");

        // HTTP
        self.push_line("    let mut map = HashMap::new();");
        self.push_line("    map.insert(\"get\".to_string(), Value::NativeFunction(std::sync::Arc::new(rustx_core::stdlib::http::get)));");
        self.push_line("    map.insert(\"post\".to_string(), Value::NativeFunction(std::sync::Arc::new(rustx_core::stdlib::http::post)));");
        self.push_line("    HTTP.set(Value::Map(map)).ok();");
        
        // OS
        self.push_line("    let mut map = HashMap::new();");
        self.push_line("    map.insert(\"env\".to_string(), Value::NativeFunction(std::sync::Arc::new(rustx_core::stdlib::os::env)));");
        self.push_line("    map.insert(\"args\".to_string(), Value::NativeFunction(std::sync::Arc::new(rustx_core::stdlib::os::args)));");
        self.push_line("    OS.set(Value::Map(map)).ok();");
        
        // Time
        self.push_line("    let mut map = HashMap::new();");
        self.push_line("    map.insert(\"now\".to_string(), Value::NativeFunction(std::sync::Arc::new(rustx_core::stdlib::time::now)));");
        self.push_line("    map.insert(\"sleep\".to_string(), Value::NativeFunction(std::sync::Arc::new(rustx_core::stdlib::time::sleep)));");
        self.push_line("    TIME.set(Value::Map(map)).ok();");
        
        // Web
        self.push_line("    let mut map = HashMap::new();");
        self.push_line("    map.insert(\"app\".to_string(), Value::NativeFunction(std::sync::Arc::new(rustx_core::stdlib::web::app)));");
        self.push_line("    map.insert(\"json\".to_string(), Value::NativeFunction(std::sync::Arc::new(rustx_core::stdlib::web::json)));");
        self.push_line("    WEB.set(Value::Map(map)).ok();");
        
        for stmt in stmts {
            self.transpile_stmt(stmt);
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

    fn enter_scope(&mut self) {
        self.scopes.push(HashSet::new());
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    fn is_declared(&self, name: &str) -> bool {
        for scope in self.scopes.iter().rev() {
            if scope.contains(name) {
                return true;
            }
        }
        false
    }

    fn declare(&mut self, name: String) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name);
        }
    }

    fn transpile_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let { name, value } => {
                let val_code = self.transpile_expr_string(value);
                if self.is_declared(name) {
                    self.push_line(&format!("{} = {};", name, val_code));
                } else {
                    self.declare(name.clone());
                    self.push_line(&format!("let mut {} = {};", name, val_code));
                }
            }
            Stmt::Expr(expr) => {
                let expr_code = self.transpile_expr_string(expr);
                self.push_line(&format!("{};", expr_code));
            }
            Stmt::Function { name, params, body } => {
                 self.push_line(&format!("fn {}(args: Vec<Value>) -> Result<Value, String> {{", name));
                 self.indent_level += 1;
                 
                 self.enter_scope();
                 
                 // Check arg count
                 if !params.is_empty() {
                     self.push_line(&format!("if args.len() != {} {{ return Err(format!(\"Expected {} arguments, got {{}}\", args.len())); }}", params.len(), params.len()));
                     for (i, param) in params.iter().enumerate() {
                         self.declare(param.clone());
                         self.push_line(&format!("let {} = args[{}].clone();", param, i));
                     }
                 }
                 
                 let body_code = self.transpile_expr_string(body);
                 self.push_line(&format!("Ok({})", body_code));
                 
                 self.exit_scope();
                 self.indent_level -= 1;
                 self.push_line("}");
                 
                 // Register as Value variable
                 self.declare(name.clone());
                 self.push_line(&format!("let {} = Value::NativeFunction(std::sync::Arc::new({}));", name, name));
            }
            Stmt::Return(opt_expr) => {
                if let Some(expr) = opt_expr {
                    let code = self.transpile_expr_string(expr);
                    self.push_line(&format!("return Ok({});", code));
                } else {
                    self.push_line("return Ok(Value::Null);");
                }
            }
            Stmt::While { condition, body } => {
                let cond_code = self.transpile_expr_string(condition);
                self.push_line(&format!("while {}.is_truthy() {{", cond_code));
                self.indent_level += 1;
                
                 if let Expr::Block(stmts) = &**body {
                     let block_inner = self.transpile_block_body(stmts, false);
                     self.push_line(&block_inner);
                 }
                
                self.indent_level -= 1;
                self.push_line("}");
            }
            Stmt::For { iterator, iterable, body } => {
                 let iter_code = self.transpile_expr_string(iterable);
                 self.push_line(&format!("for {}_ref in {}.as_array().unwrap().iter() {{", iterator, iter_code));
                 self.indent_level += 1;
                 self.enter_scope();
                 self.declare(iterator.clone());
                 self.push_line(&format!("let mut {} = {}_ref.clone();", iterator, iterator));
                 
                 if let Expr::Block(stmts) = &**body {
                     let block_inner = self.transpile_block_body(stmts, false);
                     self.push_line(&block_inner);
                 }
                 
                 self.exit_scope();
                 self.indent_level -= 1;
                 self.push_line("}");
            }
            Stmt::Import { .. } => {
                self.push_line("// Import ignored in transpiler (handled by JIT/runtime)");
            }
            Stmt::RustImport { crate_name, alias, .. } => {
                // We emit a use declaration. Note: This assumes the crate is available
                // (which the CLI ensures via JIT compilation).
                let safe_crate_name = crate_name.replace('-', "_");
                if let Some(alias_name) = alias {
                     self.push_line(&format!("use {} as {};", safe_crate_name, alias_name));
                } else {
                     self.push_line(&format!("use {};", safe_crate_name));
                }
            }
            Stmt::RustBlock { code } => {
                self.push_line("/* Embedded Rust Block */");
                // Split lines to maintain indentation
                for line in code.lines() {
                     self.push_line(line.trim());
                }
            }
        }
    }

    fn transpile_expr_string(&mut self, expr: &Expr) -> String {
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
                     let args = args.iter().map(|a| a.to_string()).collect::<Vec<_>>().join(", ");
                     format!("Value::String(format!(\"{}\", {}))", format_str, args)
                 }
            }
            Expr::Array(items) => {
                let items_code: Vec<String> = items.iter().map(|i| self.transpile_expr_string(i)).collect();
                format!("Value::Array(vec![{}])", items_code.join(", "))
            }
            Expr::Map(entries) => {
                 let mut inserts = Vec::new();
                 for (key, val) in entries {
                      let val_code = self.transpile_expr_string(val);
                      inserts.push(format!("map.insert(\"{}\".to_string(), {});", key, val_code));
                 }
                 format!("Value::Map({{ let mut map = HashMap::new(); {} map }})", inserts.join(" "))
            },
            Expr::Binary { left, op, right } => {
                let l = self.transpile_expr_string(left);
                let r = self.transpile_expr_string(right);
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
                let e = self.transpile_expr_string(expr);
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
                let body = self.transpile_block_body(stmts, true);
                output.push_str(&body);
                self.exit_scope();
                self.indent_level -= 1;
                output.push_str(&format!("{}}}", "    ".repeat(self.indent_level)));
                output
            }
            Expr::If { condition, then_branch, else_branch } => {
                let cond = self.transpile_expr_string(condition);
                let then_code = self.transpile_expr_string(then_branch);
                if let Some(else_expr) = else_branch {
                    let else_code = self.transpile_expr_string(else_expr);
                    format!("if {}.is_truthy() {{ {} }} else {{ {} }}", cond, then_code, else_code)
                } else {
                    format!("if {}.is_truthy() {{ {}; Value::Null }} else {{ Value::Null }}", cond, then_code)
                }
            }
            Expr::Call { callee, args } => {
                if let Expr::Ident(name) = &**callee {
                   match name.as_str() {
                       "print" => {
                           let args_code: Vec<String> = args.iter().map(|a| self.transpile_expr_string(a)).collect();
                           let prints = args_code.iter().map(|a| format!("print!(\"{{}} \", {});", a)).collect::<Vec<_>>().join("\n");
                           return format!("{{ {} println!(); Value::Null }}", prints);
                       },
                       "range" => {
                            // range(end) or range(start, end)
                            let args_code: Vec<String> = args.iter().map(|a| self.transpile_expr_string(a)).collect();
                            if args_code.len() == 1 {
                                return format!("Value::range(0, {}.as_int()?, 1)?", args_code[0]);
                            } else if args_code.len() == 2 {
                                return format!("Value::range({}.as_int()?, {}.as_int()?, 1)?", args_code[0], args_code[1]);
                            } else if args_code.len() == 3 {
                                return format!("Value::range({}.as_int()?, {}.as_int()?, {}.as_int()?)?", args_code[2], args_code[0], args_code[1]);
                            }
                       },
                       "len" => return format!("Value::Int({}.len()?)", self.transpile_expr_string(&args[0])),
                       "push" => return format!("{}.push({})?", self.transpile_expr_string(&args[0]), self.transpile_expr_string(&args[1])),
                       "pop" => return format!("{}.pop()?", self.transpile_expr_string(&args[0])),
                       "split" => return format!("{}.split(&{})?", self.transpile_expr_string(&args[0]), self.transpile_expr_string(&args[1])),
                       "upper" => return format!("{}.upper()?", self.transpile_expr_string(&args[0])),
                       "lower" => return format!("{}.lower()?", self.transpile_expr_string(&args[0])),
                       "trim" => return format!("{}.trim()?", self.transpile_expr_string(&args[0])),
                       "abs" => return format!("{}.abs()?", self.transpile_expr_string(&args[0])),
                       "floor" => return format!("{}.floor()?", self.transpile_expr_string(&args[0])),
                       "ceil" => return format!("{}.ceil()?", self.transpile_expr_string(&args[0])),
                       "round" => return format!("{}.round()?", self.transpile_expr_string(&args[0])),
                       _ => {} // Fallback to generic call
                   }
                }
                
                // Generic call
                let callee_code = self.transpile_expr_string(callee);
                let args_code_vec: Vec<String> = args.iter().map(|a| self.transpile_expr_string(a)).collect();
                let args_code = if args_code_vec.is_empty() {
                    "vec![]".to_string()
                } else {
                    format!("vec![{}]", args_code_vec.join(", "))
                };
                
                format!("{}.call({})?", callee_code, args_code)
            }
            Expr::MethodCall { object, method, args } => {
                let obj_code = self.transpile_expr_string(object);
                
                // Collect args
                let mut args_code_vec = Vec::new();
                for arg in args {
                    args_code_vec.push(self.transpile_expr_string(arg));
                }
                let args_code = if args_code_vec.is_empty() {
                    "vec![]".to_string()
                } else {
                    format!("vec![{}]", args_code_vec.join(", "))
                };

                // Use the new centralized call_method
                format!("{}.call_method(\"{}\", {})?", obj_code, method, args_code)
            }
            _ => "Value::Null".to_string(),
        }
    }
    
    // Returns the body of a block as string. 
    fn transpile_block_body(&mut self, stmts: &[Stmt], return_last: bool) -> String {
         let old_code = std::mem::take(&mut self.code);
         let old_indent = self.indent_level;
         
         let len = stmts.len();
         if len == 0 {
             if return_last {
                 self.push_line("Value::Null");
             }
         } else {
             for (i, stmt) in stmts.iter().enumerate() {
                 if return_last && i == len - 1 {
                     match stmt {
                         Stmt::Expr(e) => {
                             let expr_code = self.transpile_expr_string(e);
                              self.push_line(&expr_code);
                         }
                         Stmt::Return(_) => { 
                             self.transpile_stmt(stmt);
                         }
                         _ => {
                             self.transpile_stmt(stmt);
                             self.push_line("Value::Null"); 
                         }
                     }
                 } else {
                     self.transpile_stmt(stmt);
                 }
             }
         }
         
         let buf = std::mem::replace(&mut self.code, old_code);
         self.indent_level = old_indent; 
         buf
    }
}
