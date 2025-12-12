/// Token types for the RustX language
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    
    // Identifiers and Keywords
    Ident(String),
    Fn,
    If,
    Else,
    While,
    For,
    In,
    Return,
    Import,
    As,
    
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Eq,
    EqEq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    And,
    Or,
    Not,
    Arrow, // =>
    
    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Colon,
    Semicolon,
    Newline,
    
    // Special
    Eof,
}

impl Token {
    /// Returns true if this token is a keyword
    pub fn is_keyword(s: &str) -> Option<Token> {
        match s {
            "fn" => Some(Token::Fn),
            "if" => Some(Token::If),
            "else" => Some(Token::Else),
            "while" => Some(Token::While),
            "for" => Some(Token::For),
            "in" => Some(Token::In),
            "return" => Some(Token::Return),
            "import" => Some(Token::Import),
            "as" => Some(Token::As),
            "true" => Some(Token::Bool(true)),
            "false" => Some(Token::Bool(false)),
            _ => None,
        }
    }
}
