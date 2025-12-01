use crate::runtime::Value;
use crate::error::{Error, Result};

#[derive(Debug, Clone)]
pub enum Node {
    Program(Vec<Node>),
    
    // Statements
    VariableDecl {
        var_type: VarType,
        name: String,
        value: Box<Node>,
    },
    
    Assignment {
        name: String,
        value: Box<Node>,
    },
    
    ExpressionStmt(Box<Node>),
    
    // Expressions
    BinaryOp {
        left: Box<Node>,
        operator: BinaryOperator,
        right: Box<Node>,
    },
    
    FunctionCall {
        name: String,
        arguments: Vec<Node>,
    },
    
    // Literals
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Identifier(String),
    
    // Built-in type declarations
    TypeAnnotation(String),
}

#[derive(Debug, Clone)]
pub enum VarType {
    Str,
    Int,
    Bool,
    Float,
    Auto,
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equals,
    NotEquals,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Node>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            statements: Vec::new(),
        }
    }
    
    pub fn add_statement(&mut self, stmt: Node) {
        self.statements.push(stmt);
    }
}

impl Node {
    // Node evaluation will be implemented in the interpreter module
}