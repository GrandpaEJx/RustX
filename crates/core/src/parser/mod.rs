use crate::ast::{Stmt, Expr};
use crate::Token;

pub mod expr;
pub mod stmt;

/// Parser for RustX language
pub struct Parser {
    pub(crate) tokens: Vec<Token>,
    pub(crate) current: usize,
}

impl Parser {
    /// Creates a new parser from a token stream
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    /// Returns the current token
    pub(crate) fn current_token(&self) -> &Token {
        self.tokens.get(self.current).unwrap_or(&Token::Eof)
    }

    /// Advances to the next token
    pub(crate) fn advance(&mut self) -> &Token {
        if self.current < self.tokens.len() {
            self.current += 1;
        }
        self.current_token()
    }

    /// Checks if current token matches expected type
    pub(crate) fn check(&self, token: &Token) -> bool {
        std::mem::discriminant(self.current_token()) == std::mem::discriminant(token)
    }

    /// Consumes token if it matches, otherwise returns error
    pub(crate) fn expect(&mut self, token: Token) -> Result<(), String> {
        if self.check(&token) {
            self.advance();
            Ok(())
        } else {
            Err(format!(
                "Expected {:?}, found {:?}",
                token,
                self.current_token()
            ))
        }
    }

    /// Skips newline tokens
    pub(crate) fn skip_newlines(&mut self) {
        while matches!(self.current_token(), Token::Newline) {
            self.advance();
        }
    }

    /// Parses a program (list of statements)
    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();
        self.skip_newlines();

        while !matches!(self.current_token(), Token::Eof) {
            statements.push(self.parse_statement()?);
            self.skip_newlines();
        }

        Ok(statements)
    }

    // Delegate methods to submodules

    pub fn parse_statement(&mut self) -> Result<Stmt, String> {
        stmt::parse_statement(self)
    }

    pub fn parse_expression(&mut self) -> Result<Expr, String> {
        expr::parse_expression(self)
    }

    pub(crate) fn parse_block_expr(&mut self) -> Result<Expr, String> {
        expr::parse_block_expr(self)
    }
}
