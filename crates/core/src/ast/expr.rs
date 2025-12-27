//! Expression nodes for RustX AST

/// Expression enum representing all expression types
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    // Literals
    Int(i64),
    Float(f64),
    String(String),
    TemplateString(String), // Backtick string with {var} interpolation
    Bool(bool),
    Null,

    // Identifier
    Ident(String),

    // Binary operations
    Binary {
        left: Box<Expr>,
        op: super::BinaryOp,
        right: Box<Expr>,
    },

    // Unary operations
    Unary {
        op: super::UnaryOp,
        expr: Box<Expr>,
    },

    // Function call
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },

    // Method call (for chaining)
    MethodCall {
        object: Box<Expr>,
        method: String,
        args: Vec<Expr>,
    },

    // Array literal
    Array(Vec<Expr>),

    // Map literal
    Map(Vec<(String, Expr)>),

    // Index access (array[index] or map[key])
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
    },

    // If expression
    If {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Option<Box<Expr>>,
    },

    // Block expression
    Block(Vec<super::Stmt>),

    // Assignment
    Assign {
        name: String,
        value: Box<Expr>,
    },
}
