use crate::error::{Error, Result};
use crate::lexer::{Token, TokenType};
use super::Parser;

impl Parser {
    pub fn consume(&mut self, expected: TokenType) -> Result<()> {
        if self.check(expected.clone()) {
            self.advance_token()?;
            Ok(())
        } else {
            let (line, column) = self.current_position();
            Err(Error::ParseError {
                message: format!(
                    "Expected {:?}, found {:?}",
                    expected,
                    self.current_token.as_ref().unwrap().token_type
                ),
                line,
                column,
            })
        }
    }

    pub fn consume_identifier(&mut self) -> Result<String> {
        match &self.current_token.as_ref().unwrap().token_type {
            TokenType::Identifier(name) => {
                let name = name.clone();
                self.advance_token()?;
                Ok(name)
            }
            _ => {
                let (line, column) = self.current_position();
                Err(Error::ParseError {
                    message: "Expected identifier".to_string(),
                    line,
                    column,
                })
            }
        }
    }

    pub fn check(&self, token_type: TokenType) -> bool {
        self.current_token
            .as_ref()
            .map_or(false, |token| token.token_type == token_type)
    }

    pub fn advance_token(&mut self) -> Result<&Token> {
        self.previous_token = self.current_token.clone();
        self.current_token = Some(self.lexer.next_token()?);

        // Return previous token if available, otherwise return current token
        if let Some(prev_token) = &self.previous_token {
            Ok(prev_token)
        } else if let Some(curr_token) = &self.current_token {
            Ok(curr_token)
        } else {
            // This should never happen, but provide a fallback
            Ok(&Token {
                token_type: TokenType::EOF,
                line: 0,
                column: 0,
            })
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.current_token
            .as_ref()
            .map_or(true, |token| matches!(token.token_type, TokenType::EOF))
    }

    pub fn current_position(&self) -> (usize, usize) {
        if let Some(token) = &self.current_token {
            (token.line, token.column)
        } else {
            (0, 0)
        }
    }
}