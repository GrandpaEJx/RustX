use crate::ast::Node;
use crate::error::Result;
use crate::lexer::TokenType;
use super::Parser;

pub mod variables;
pub mod functions;
pub mod control;

impl Parser {
    pub fn parse_statement(&mut self) -> Result<Option<Node>> {
        // Skip newlines
        while self.check(TokenType::Newline) {
            self.advance_token();
        }

        if self.is_at_end() {
            return Ok(None);
        }

        match &self.current_token.as_ref().unwrap().token_type {
            TokenType::Fn => self.parse_function_declaration(),
            TokenType::Return => self.parse_return_statement(),
            TokenType::Let => self.parse_variable_declaration(),
            TokenType::TypeString
            | TokenType::TypeInt
            | TokenType::TypeBool
            | TokenType::TypeFloat => self.parse_variable_declaration(),
            TokenType::Print | TokenType::Println | TokenType::Printf => self.parse_function_call(),
            _ => {
                if self.current_token.as_ref().unwrap().token_type != TokenType::EOF {
                    let expr = self.parse_expression()?;

                    // If this is a function call as a statement, make sure it has proper handling
                    Ok(Some(Node::ExpressionStmt(Box::new(expr))))
                } else {
                    Ok(None)
                }
            }
        }
    }
}