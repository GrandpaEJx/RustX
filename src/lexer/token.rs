#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Identifier(String),
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),

    // Keywords
    Let,
    Fn,
    Return,
    TypeString,
    TypeInt,
    TypeBool,
    TypeFloat,
    Print,
    Println,
    Printf,
    Null,

    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Equals,
    DoubleEquals,
    LessThan,
    LessThanEquals,
    GreaterThan,
    GreaterThanEquals,
    NotEquals,
    Modules,
    Arrow,     // ->
    Ampersand, // &
    Colon,     // :

    // Delimiters
    Semicolon,
    Comma,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    // Special
    EOF,
    Newline,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}
