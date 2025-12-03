use crate::ast::Node;
use crate::error::Result;
use crate::lexer::TokenType;
use super::super::Parser;

impl Parser {
    pub fn parse_function_call(&mut self) -> Result<Option<Node>> {
        let name = match &self.current_token.as_ref().unwrap().token_type {
            TokenType::Print => {
                self.advance_token()?;
                "print".to_string()
            }
            TokenType::Println => {
                self.advance_token()?;
                "println".to_string()
            }
            TokenType::Printf => {
                self.advance_token()?;
                "printf".to_string()
            }
            TokenType::Identifier(name) => {
                let name = name.clone();
                self.advance_token()?;
                name
            }
            _ => {
                let (line, column) = self.current_position();
                return Err(crate::error::Error::ParseError {
                    message: "Expected function call".to_string(),
                    line,
                    column,
                });
            }
        };

        self.consume(TokenType::LeftParen)?;
        let mut arguments = Vec::new();

        if !self.check(TokenType::RightParen) {
            // For built-in functions, detect named arguments
            let has_named_args = self.is_named_argument();

            if has_named_args {
                // Named argument: name = value
                let param_name = self.consume_identifier()?;
                self.consume(TokenType::Equals)?;
                let value = self.parse_expression()?;
                arguments.push((param_name, value));

                while self.check(TokenType::Comma) {
                    self.advance_token()?;
                    let param_name = self.consume_identifier()?;
                    self.consume(TokenType::Equals)?;
                    let value = self.parse_expression()?;
                    arguments.push((param_name, value));
                }
            } else {
                // Positional arguments (for built-in functions)
                arguments.push(("".to_string(), self.parse_expression()?));

                while self.check(TokenType::Comma) {
                    self.advance_token()?;
                    arguments.push(("".to_string(), self.parse_expression()?));
                }
            }
        }

        self.consume(TokenType::RightParen)?;

        // Accept either semicolon or newline as statement terminator
        if self.check(TokenType::Semicolon) {
            self.advance_token()?;
        } else if self.check(TokenType::Newline) {
            self.advance_token()?;
        }

        Ok(Some(Node::FunctionCall { name, arguments }))
    }

    pub fn parse_function_declaration(&mut self) -> Result<Option<Node>> {
        self.consume(TokenType::Fn)?;
        let name = self.consume_identifier()?;

        // Parse parameters
        self.consume(TokenType::LeftParen)?;
        let mut parameters = Vec::new();

        if !self.check(TokenType::RightParen) {
            loop {
                let param_name = self.consume_identifier()?;
                self.consume(TokenType::Colon)?;
                let param_type = self.consume_param_type()?;
                parameters.push((param_name, param_type));

                if !self.check(TokenType::Comma) {
                    break;
                }
                self.advance_token()?; // consume comma
            }
        }

        self.consume(TokenType::RightParen)?;

        // Parse return type
        let return_type = if self.check(TokenType::Arrow) {
            self.advance_token()?; // consume ->
            self.consume_param_type()?
        } else {
            crate::ast::VarType::Void
        };

        // Parse function body
        self.consume(TokenType::LeftBrace)?;
        let mut body = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            if let Some(stmt) = self.parse_statement()? {
                body.push(stmt);
            }
        }

        self.consume(TokenType::RightBrace)?;

        Ok(Some(Node::FunctionDecl {
            name,
            parameters,
            return_type,
            body,
        }))
    }
}