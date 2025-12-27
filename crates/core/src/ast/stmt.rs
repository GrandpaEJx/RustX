//! Statement nodes for RustX AST

/// Statement enum representing all statement types
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    // Expression statement
    Expr(super::Expr),

    // Variable declaration/assignment
    Let {
        name: String,
        value: super::Expr,
    },

    // Function declaration
    Function {
        name: String,
        params: Vec<String>,
        body: Box<super::Expr>,
    },

    // Return statement
    Return(Option<super::Expr>),

    // While loop
    While {
        condition: super::Expr,
        body: Box<super::Expr>,
    },

    // For loop
    For {
        iterator: String,
        iterable: super::Expr,
        body: Box<super::Expr>,
    },

    // Use statement (stdlib imports: use json, use os)
    Use {
        module: String,
    },

    // Import statement (file imports: import 'file.rsx' as name)
    Import {
        path: String,
        alias: Option<String>,
    },
    RustImport {
        crate_name: String,
        version: String,
        alias: Option<String>,
    },
    RustBlock {
        code: String,
    },
}
