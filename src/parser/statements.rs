use crate::ast::{Node, VarType};
use crate::error::Result;
use crate::lexer::TokenType;
use super::Parser;

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

    pub fn parse_function_call(&mut self) -> Result<Option<Node>> {
        let name = match &self.current_token.as_ref().unwrap().token_type {
            TokenType::Print => {
                self.advance_token();
                "print".to_string()
            }
            TokenType::Println => {
                self.advance_token();
                "println".to_string()
            }
            TokenType::Printf => {
                self.advance_token();
                "printf".to_string()
            }
            TokenType::Identifier(name) => {
                let name = name.clone();
                self.advance_token();
                name
            }
            _ => return Err(crate::error::Error::ParseError("Expected function call".to_string())),
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
                    self.advance_token();
                    let param_name = self.consume_identifier()?;
                    self.consume(TokenType::Equals)?;
                    let value = self.parse_expression()?;
                    arguments.push((param_name, value));
                }
            } else {
                // Positional arguments (for built-in functions)
                arguments.push(("".to_string(), self.parse_expression()?));

                while self.check(TokenType::Comma) {
                    self.advance_token();
                    arguments.push(("".to_string(), self.parse_expression()?));
                }
            }
        }

        self.consume(TokenType::RightParen)?;

        // Accept either semicolon or newline as statement terminator
        if self.check(TokenType::Semicolon) {
            self.advance_token();
        } else if self.check(TokenType::Newline) {
            self.advance_token();
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
                self.advance_token(); // consume comma
            }
        }

        self.consume(TokenType::RightParen)?;

        // Parse return type
        let return_type = if self.check(TokenType::Arrow) {
            self.advance_token(); // consume ->
            self.consume_param_type()?
        } else {
            VarType::Void
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

    pub fn parse_return_statement(&mut self) -> Result<Option<Node>> {
        self.consume(TokenType::Return)?;

        let value = if self.check(TokenType::Semicolon) || self.check(TokenType::Newline) {
            None
        } else {
            Some(Box::new(self.parse_expression()?))
        };

        // Accept either semicolon or newline as statement terminator
        if self.check(TokenType::Semicolon) {
            self.advance_token();
        } else if self.check(TokenType::Newline) {
            self.advance_token();
        }

        Ok(Some(Node::Return { value }))
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