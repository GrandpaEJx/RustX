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

    pub fn transpile(&mut self, stmts: &[Stmt]) -> String {
        self.code.clear();
        self.code.push_str("use rustx_core::value::Value;\n");

        self.code.push_str("#[allow(unreachable_code)]\nfn main() -> Result<(), String> {\n");
        self.indent_level += 1;
        
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
                 // Transpile as inner function to support recursion
                 // Note: Inner functions cannot capture environment, strictly pure or arg-based
                 
                 let params_str = params.iter().map(|p| format!("{}: Value", p)).collect::<Vec<_>>().join(", ");
                 
                 self.push_line(&format!("fn {}({}) -> Result<Value, String> {{", name, params_str));
                 self.indent_level += 1;
                 
                 // We don't need to declare params in scope because they are args
                 // But we might need to track them if we check scope?
                 self.enter_scope();
                 for param in params {
                     self.declare(param.clone());
                 }
                 
                 let body_code = self.transpile_expr_string(body);
                 self.push_line(&format!("Ok({})", body_code));
                 
                 self.exit_scope();
                 self.indent_level -= 1;
                 self.push_line("}");
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
                
                // Body
                 if let Expr::Block(stmts) = &**body {
                     // We just execute stmts, ignore return value
                     let block_inner = self.transpile_block_body(stmts, false);
                     self.push_line(&block_inner);
                 }
                
                self.indent_level -= 1;
                self.push_line("}");
            }
            Stmt::For { iterator, iterable, body } => {
                 let iter_code = self.transpile_expr_string(iterable);
                 // Need to handle different iterables?
                 // Assume array for now -> .as_array().unwrap().iter()
                 // Use `into_iter()` on clone? `Value` is cloneable.
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
                self.push_line("// Import ignored");
            }
        }
    }

    fn transpile_expr_string(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Int(n) => format!("Value::Int({})", n),
            Expr::Float(f) => format!("Value::Float({:?})", f),
            Expr::String(s) => format!("Value::String(\"{}\".to_string())", s),
            Expr::Bool(b) => format!("Value::Bool({})", b),
            Expr::Null => "Value::Null".to_string(),
            Expr::Ident(name) => format!("{}.clone()", name),
            Expr::TemplateString(s) => {
                // Parse template string "Hello {name}!" -> format!("Hello {}!", name)
                let mut format_str = String::new();
                let mut args = Vec::new();
                let mut chars = s.chars().peekable();
                
                while let Some(ch) = chars.next() {
                    if ch == '{' {
                        if let Some(&next) = chars.peek() {
                            if next == '{' {
                                // Escaped {{
                                format_str.push_str("{{"); // Rust format escape is {{
                                chars.next(); 
                            } else {
                                // Variable start
                                let mut var_name = String::new();
                                while let Some(&c) = chars.peek() {
                                    if c == '}' {
                                        chars.next(); // consume }
                                        break;
                                    }
                                    var_name.push(chars.next().unwrap());
                                }
                                
                                if !var_name.is_empty() {
                                    format_str.push_str("{}");
                                    args.push(var_name);
                                }
                            }
                        } else {
                             // Trailing {
                             format_str.push('{');
                        }
                    } else if ch == '}' {
                         if let Some(&next) = chars.peek() {
                            if next == '}' {
                                // Escaped }}
                                format_str.push_str("}}"); 
                                chars.next();
                            } else {
                                format_str.push('}');
                            }
                         } else {
                             format_str.push('}');
                         }
                    } else {
                        format_str.push(ch);
                    }
                }
                
                if args.is_empty() {
                    format!("Value::String(\"{}\".to_string())", format_str)
                } else {
                    let args = args.iter().map(|a| format!("{}", a)).collect::<Vec<_>>().join(", ");
                    format!("Value::String(format!(\"{}\", {}))", format_str, args)
                }
            }
            Expr::Array(items) => {
                let items_code: Vec<String> = items.iter().map(|i| self.transpile_expr_string(i)).collect();
                format!("Value::Array(vec![{}])", items_code.join(", "))
            }
            Expr::Map(_pairs) => {
                 // HashMap construction 
                 // Value::Map(HashMap::from([("k".to_string(), v), ...]))
                 // simplified
                 "Value::Null /* Map todo */".to_string()
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
                // indentation?
                
                // We need to use the helper but capture output differently or format it.
                // Simple approach: Use helper that modifies self.code, store it, clear it, restore it?
                // The helper `transpile_block_body` does exactly that.
                
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
                            // range(start, end) compile to Array
                            // implement simple range for 1 or 2 args
                            // Value::Array((start..end).map(Value::Int).collect())
                            // Todo: implement run-time helper for this
                            return "Value::Array(vec![]) /* range todo */".to_string();
                       },
                       _ => {}
                   }
                }
                
                let callee_code = self.transpile_expr_string(callee);
                let args_code = args.iter().map(|a| self.transpile_expr_string(a)).collect::<Vec<_>>().join(", ");
                // Closure Call
                format!("{}({})?", callee_code, args_code)
            }
            Expr::MethodCall { object: _, method: _, args: _ } => {
                 // Implement method calls as Value helper calls if I added them?
                 // I haven't added `upper`, `push` etc to Value yet.
                 // So maybe compile to runtime helper? 
                 // For now: placeholder
                 "Value::Null /* method todo */".to_string()
            }
            _ => "Value::Null".to_string(),
        }
    }
    
    // Returns the body of a block as string. 
    // `return_last`: if true, checks if last stmt is expression and returns it without semicolon.
    fn transpile_block_body(&mut self, stmts: &[Stmt], return_last: bool) -> String {
         let old_code = std::mem::take(&mut self.code);
         let old_indent = self.indent_level;
         
         // Fix indent for the capture? No, `push_line` uses current indent.
         
         let len = stmts.len();
         if len == 0 {
             if return_last {
                 self.push_line("Value::Null");
             }
         } else {
             for (i, stmt) in stmts.iter().enumerate() {
                 if return_last && i == len - 1 {
                     // Last statement, try to return it
                     match stmt {
                         Stmt::Expr(e) => {
                             let expr_code = self.transpile_expr_string(e);
                              self.push_line(&expr_code);
                         }
                         Stmt::Return(_) => { 
                             self.transpile_stmt(stmt);
                             // Return stmt already returns, so technically block returns ! which works?
                             // But we need block to evaluate to value...
                             // Convert `return x` to `x`? No.
                             // Logic error: `return` exits function, not block.
                         }
                         _ => {
                             self.transpile_stmt(stmt);
                             // Return Null
                             self.push_line("Value::Null"); 
                         }
                     }
                 } else {
                     self.transpile_stmt(stmt);
                 }
             }
         }
         
         let buf = std::mem::replace(&mut self.code, old_code);
         self.indent_level = old_indent; // restore indent if it drifted (shouldn't)
         buf
    }
}
