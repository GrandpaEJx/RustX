use crate::ast::{Node, VarType};
use crate::error::Result;
use crate::lexer::TokenType;
use super::super::Parser;

impl Parser {
    pub fn parse_variable_declaration(&mut self) -> Result<Option<Node>> {
        // Check if it's a 'let' declaration or explicit type declaration
        let var_type = if self.check(TokenType::Let) {
            self.advance_token(); // consume 'let'
            VarType::Auto
        } else {
            self.consume_type()?
        };

        let name = self.consume_identifier()?;
        self.consume(TokenType::Equals)?;
        let value = self.parse_expression()?;

        // Accept either semicolon or newline as statement terminator
        if self.check(TokenType::Semicolon) {
            self.advance_token();
        } else if self.check(TokenType::Newline) {
            self.advance_token();
        }

        Ok(Some(Node::VariableDecl {
            var_type,
            name,
            value: Box::new(value),
        }))
    }

    pub fn consume_type(&mut self) -> Result<VarType> {
        match &self.current_token.as_ref().unwrap().token_type {
            TokenType::TypeString => {
                self.advance_token();
                Ok(VarType::Str)
            }
            TokenType::TypeInt => {
                self.advance_token();
                Ok(VarType::Int)
            }
            TokenType::TypeBool => {
                self.advance_token();
                Ok(VarType::Bool)
            }
            TokenType::TypeFloat => {
                self.advance_token();
                Ok(VarType::Float)
            }
            _ => Err(crate::error::Error::ParseError("Expected type".to_string())),
        }
    }

    pub fn consume_param_type(&mut self) -> Result<VarType> {
        // Handle reference types like &str
        if self.check(TokenType::Ampersand) {
            self.advance_token();
            let inner_type = self.consume_type()?;
            return Ok(VarType::Ref(Box::new(inner_type)));
        }

        // Handle regular types
        self.consume_type()
    }
}