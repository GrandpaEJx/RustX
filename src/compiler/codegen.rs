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
        self.output.push_str("fn main() {\n");
        for stmt in program.statements {
            self.compile_node(stmt)?;
        }
        self.output.push_str("}\n");
        Ok(self.output.clone())
    }

    fn compile_node(&mut self, node: Node) -> Result<()> {
        match node {
            Node::VariableDecl { var_type, name, value } => {
                let type_string = match var_type {
                    VarType::Str => "String",
                    VarType::Int => "i64",
                    VarType::Bool => "bool",
                    VarType::Float => "f64",
                    VarType::Auto => "", 
                };
                
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
            _ => return Err(Error::CompilerError("Unsupported node".to_string())),
        }
        Ok(())
    }
}
