use crate::ast::{Node, Program, VarType, BinaryOperator};
use crate::error::{Error, Result};

pub struct Compiler {
    output: String,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            output: String::new(),
        }
    }

    pub fn compile(&mut self, program: Program) -> Result<String> {
        // Separate function declarations from other statements
        let mut functions = Vec::new();
        let mut main_statements = Vec::new();
        
        for stmt in program.statements {
            match &stmt {
                Node::FunctionDecl { .. } => functions.push(stmt),
                _ => main_statements.push(stmt),
            }
        }
        
        // Compile function declarations first
        for func in functions {
            self.compile_node(func)?;
            self.output.push_str("\n");
        }
        
        // Then compile main function
        self.output.push_str("fn main() {\n");
        for stmt in main_statements {
            self.compile_node(stmt)?;
        }
        self.output.push_str("}\n");
        Ok(self.output.clone())
    }

    fn compile_node(&mut self, node: Node) -> Result<()> {
        match node {
            Node::VariableDecl { var_type, name, value } => {
                let type_string = self.type_to_string(&var_type);
                
                if type_string.is_empty() {
                     self.output.push_str(&format!("    let {} = ", name));
                } else {
                     self.output.push_str(&format!("    let {}: {} = ", name, type_string));
                }
               
                self.compile_node(*value)?;
                self.output.push_str(";\n");
            },
            Node::ExpressionStmt(expr) => {
                self.output.push_str("    ");
                self.compile_node(*expr)?;
                self.output.push_str(";\n");
            },
            Node::FunctionCall { name, arguments } => {
                let rust_func = match name.as_str() {
                    "print" => "print!",
                    "println" => "println!",
                    "printf" => "println!", 
                    _ => &name,
                };
                
                self.output.push_str(&format!("    {}(", rust_func));
                for (i, arg) in arguments.iter().enumerate() {
                    if i > 0 {
                        self.output.push_str(", ");
                    }
                    self.compile_node(arg.clone())?;
                }
                self.output.push_str(");\n");
            },
            Node::String(s) => self.output.push_str(&format!("\"{}\".to_string()", s)),
            Node::Integer(i) => self.output.push_str(&i.to_string()),
            Node::Float(f) => self.output.push_str(&f.to_string()),
            Node::Boolean(b) => self.output.push_str(&b.to_string()),
            Node::Identifier(s) => self.output.push_str(&s),
            Node::BinaryOp { left, operator, right } => {
                self.compile_node(*left)?;
                let op_str = match operator {
                    BinaryOperator::Add => "+",
                    BinaryOperator::Subtract => "-",
                    BinaryOperator::Multiply => "*",
                    BinaryOperator::Divide => "/",
                    BinaryOperator::Equals => "==",
                    BinaryOperator::NotEquals => "!=",
                };
                self.output.push_str(&format!(" {} ", op_str));
                self.compile_node(*right)?;
            },
            Node::FunctionDecl { name, parameters, return_type, body } => {
                // Generate function signature
                self.output.push_str(&format!("fn {}(", name));
                
                // Generate parameters
                for (i, (param_name, param_type)) in parameters.iter().enumerate() {
                    if i > 0 {
                        self.output.push_str(", ");
                    }
                    let type_str = self.type_to_string(param_type);
                    self.output.push_str(&format!("{}: {}", param_name, type_str));
                }
                
                self.output.push_str(")");
                
                // Generate return type
                if !matches!(return_type, VarType::Void) {
                    let ret_type_str = self.type_to_string(&return_type);
                    self.output.push_str(&format!(" -> {}", ret_type_str));
                }
                
                // Generate function body
                self.output.push_str(" {\n");
                for stmt in body {
                    self.output.push_str("    ");
                    self.compile_node(stmt)?;
                }
                self.output.push_str("}\n");
            },
            Node::Return { value } => {
                self.output.push_str("return");
                if let Some(val) = value {
                    self.output.push_str(" ");
                    self.compile_node(*val)?;
                }
                self.output.push_str(";\n");
            },
            Node::Null => {
                self.output.push_str("()");
            },
            _ => return Err(Error::CompilerError("Unsupported node".to_string())),
        }
        Ok(())
    }
    
    fn type_to_string(&self, var_type: &VarType) -> String {
        match var_type {
            VarType::Str => "String".to_string(),
            VarType::Int => "i64".to_string(),
            VarType::Bool => "bool".to_string(),
            VarType::Float => "f64".to_string(),
            VarType::Auto => "".to_string(),
            VarType::Ref(inner) => format!("&{}", self.type_to_string(inner)),
            VarType::Void => "()".to_string(),
        }
    }
}
