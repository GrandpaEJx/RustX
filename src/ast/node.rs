use super::types::{BinaryOperator, VarType};

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

    FunctionDecl {
        name: String,
        parameters: Vec<(String, VarType)>, // (param_name, param_type)
        return_type: VarType,
        body: Vec<Node>,
    },

    Return {
        value: Option<Box<Node>>,
    },

    Block(Vec<Node>),

    // Expressions
    BinaryOp {
        left: Box<Node>,
        operator: BinaryOperator,
        right: Box<Node>,
    },

    FunctionCall {
        name: String,
        arguments: Vec<(String, Node)>, // (param_name, value) for named args
    },

    // Literals
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Identifier(String),
    Null,

    // Built-in type declarations
    TypeAnnotation(String),
}

impl Node {
    // Node evaluation will be implemented in the compiler module
}
