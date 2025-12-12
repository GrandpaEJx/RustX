/// Abstract Syntax Tree node types for RustX

/// Expression nodes
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    // Literals
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Null,
    
    // Identifier
    Ident(String),
    
    // Binary operations
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    
    // Unary operations
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    
    // Function call
    Call {
        callee: Box<Expr>,
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
    Block(Vec<Stmt>),
    
    // Assignment
    Assign {
        name: String,
        value: Box<Expr>,
    },
}

/// Binary operators
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    And,
    Or,
}

/// Unary operators
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Not,
    Neg,
}

/// Statement nodes
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    // Expression statement
    Expr(Expr),
    
    // Variable declaration/assignment
    Let {
        name: String,
        value: Expr,
    },
    
    // Function declaration
    Function {
        name: String,
        params: Vec<String>,
        body: Box<Expr>,
    },
    
    // Return statement
    Return(Option<Expr>),
    
    // While loop
    While {
        condition: Expr,
        body: Box<Expr>,
    },
    
    // For loop
    For {
        iterator: String,
        iterable: Expr,
        body: Box<Expr>,
    },
    
    // Import statement
    Import {
        path: String,
        alias: Option<String>,
    },
}
