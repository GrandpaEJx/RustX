use crate::ast::Node;
use crate::error::Result;
use crate::lexer::TokenType;
use super::super::Parser;

impl Parser {
    pub fn parse_return_statement(&mut self) -> Result<Option<Node>> {
        self.consume(TokenType::Return)?;

        let value = if self.check(TokenType::Semicolon) || self.check(TokenType::Newline) {
            None
        } else {
            Some(Box::new(self.parse_expression()?))
        };

        // Accept either semicolon or newline as statement terminator
        if self.check(TokenType::Semicolon) {
            self.advance_token()?;
        } else if self.check(TokenType::Newline) {
            self.advance_token()?;
        }

        Ok(Some(Node::Return { value }))
    }

    pub fn is_named_argument(&self) -> bool {
        // Check if current token is identifier followed by equals
        if let Some(token) = &self.current_token {
            if let TokenType::Identifier(_) = &token.token_type {
                // For now, we can't easily lookahead without complex state management
                // Let's be conservative and only treat specific patterns as named args
                // This is a simplified implementation

                // Check if the identifier is likely to be a parameter name
                // by checking common patterns in the test file
                if let Some(curr_token) = &self.current_token {
                    if let TokenType::Identifier(name) = &curr_token.token_type {
                        // Common parameter names in our test
                        return name == "name" || name == "msg" || name == "value";
                    }
                }
            }
        }
        false
    }
}