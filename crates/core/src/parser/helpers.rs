/// Helper methods for the parser

use crate::token::Token;
use super::Parser;

impl Parser {
    /// Returns the current token
    pub(super) fn current_token(&self) -> &Token {
        self.tokens.get(self.current).unwrap_or(&Token::Eof)
    }

    /// Advances to the next token
    pub(super) fn advance(&mut self) -> &Token {
        if self.current < self.tokens.len() {
            self.current += 1;
        }
        self.current_token()
    }

    /// Checks if current token matches expected type
    pub(super) fn check(&self, token: &Token) -> bool {
        std::mem::discriminant(self.current_token()) == std::mem::discriminant(token)
    }

    /// Consumes token if it matches, otherwise returns error
    pub(super) fn expect(&mut self, token: Token) -> Result<(), String> {
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
    pub(super) fn skip_newlines(&mut self) {
        while matches!(self.current_token(), Token::Newline) {
            self.advance();
        }
    }
}
