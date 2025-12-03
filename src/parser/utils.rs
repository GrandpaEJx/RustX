use crate::error::{Error, Result};
use crate::lexer::{Token, TokenType};
use super::Parser;

impl Parser {
    pub fn consume(&mut self, expected: TokenType) -> Result<()> {
        if self.check(expected.clone()) {
            self.advance_token();
            Ok(())
        } else {
            Err(Error::ParseError(format!(
                "Expected {:?}, found {:?}",
                expected,
                self.current_token.as_ref().unwrap().token_type
            )))
        }
    }

    pub fn consume_identifier(&mut self) -> Result<String> {
        match &self.current_token.as_ref().unwrap().token_type {
            TokenType::Identifier(name) => {
                let name = name.clone();
                self.advance_token();
                Ok(name)
            }
            _ => Err(Error::ParseError("Expected identifier".to_string())),
        }
    }

    pub fn check(&self, token_type: TokenType) -> bool {
        self.current_token
            .as_ref()
            .map_or(false, |token| token.token_type == token_type)
    }

    pub fn advance_token(&mut self) -> &Token {
        self.previous_token = self.current_token.clone();
        self.current_token = match self.lexer.next_token() {
            Ok(token) => Some(token),
            Err(_e) => {
                // If lexer fails, create an EOF token to avoid panic
                Some(Token {
                    token_type: TokenType::EOF,
                    line: 0,
                    column: 0,
                })
            }
        };

        // Return previous token if available, otherwise return current token
        if let Some(prev_token) = &self.previous_token {
            prev_token
        } else if let Some(curr_token) = &self.current_token {
            curr_token
        } else {
            // This should never happen, but provide a fallback
            &Token {
                token_type: TokenType::EOF,
                line: 0,
                column: 0,
            }
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.current_token
            .as_ref()
            .map_or(true, |token| matches!(token.token_type, TokenType::EOF))
    }
}