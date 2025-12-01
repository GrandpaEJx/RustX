use crate::lexer::{Lexer, Token, TokenType};
use crate::error::{Error, Result};
use crate::ast::{Node, Program, VarType, BinaryOperator};

pub struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
    previous_token: Option<Token>,
}

impl Parser {
    pub fn new(input: String) -> Self {
        let mut parser = Parser {
            lexer: Lexer::new(input),
            current_token: None,
            previous_token: None,
        };
        parser.advance_token();
        parser
    }
    
    pub fn parse(&mut self) -> Result<Program> {
        let mut program = Program::new();
        
        while !self.is_at_end() {
            if let Some(stmt) = self.parse_statement()? {
                program.add_statement(stmt);
            }
        }
        
        Ok(program)
    }
    
    fn parse_statement(&mut self) -> Result<Option<Node>> {
        // Skip newlines
        while self.check(TokenType::Newline) {
            self.advance_token();
        }
        
        if self.is_at_end() {
            return Ok(None);
        }
        
        match &self.current_token.as_ref().unwrap().token_type {
            TokenType::Let => self.parse_variable_declaration(),
            TokenType::TypeStr | TokenType::TypeInt | TokenType::TypeBool | TokenType::TypeFloat => {
                self.parse_variable_declaration()
            }
            TokenType::Print | TokenType::Println | TokenType::Printf => {
                self.parse_function_call()
            }
            _ => {
                if self.current_token.as_ref().unwrap().token_type != TokenType::EOF {
                    let expr = self.parse_expression()?;
                    Ok(Some(Node::ExpressionStmt(Box::new(expr))))
                } else {
                    Ok(None)
                }
            }
        }
    }
    
    fn parse_variable_declaration(&mut self) -> Result<Option<Node>> {
        let var_type = self.consume_type()?;
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
    
    fn consume_type(&mut self) -> Result<VarType> {
        match &self.current_token.as_ref().unwrap().token_type {
            TokenType::TypeStr => {
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
            _ => Err(Error::ParseError("Expected type".to_string())),
        }
    }
    
    fn parse_function_call(&mut self) -> Result<Option<Node>> {
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
            _ => return Err(Error::ParseError("Expected function call".to_string())),
        };
        
        self.consume(TokenType::LeftParen)?;
        let mut arguments = Vec::new();
        
        if !self.check(TokenType::RightParen) {
            arguments.push(self.parse_expression()?);
            
            while self.check(TokenType::Comma) {
                self.advance_token();
                arguments.push(self.parse_expression()?);
            }
        }
        
        self.consume(TokenType::RightParen)?;
        
        // Accept either semicolon or newline as statement terminator
        if self.check(TokenType::Semicolon) {
            self.advance_token();
        } else if self.check(TokenType::Newline) {
            self.advance_token();
        }
        
        Ok(Some(Node::FunctionCall {
            name,
            arguments,
        }))
    }
    
    fn parse_expression(&mut self) -> Result<Node> {
        self.parse_comparison()
    }
    
    fn parse_comparison(&mut self) -> Result<Node> {
        let mut left = self.parse_term()?;
        
        while self.check(TokenType::DoubleEquals) || self.check(TokenType::Equals) {
            let operator = self.advance_token().clone();
            let right = self.parse_term()?;
            
            left = Node::BinaryOp {
                left: Box::new(left),
                operator: BinaryOperator::Equals,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    fn parse_term(&mut self) -> Result<Node> {
        let mut left = self.parse_factor()?;
        
        while self.check(TokenType::Plus) || self.check(TokenType::Minus) {
            let operator = self.advance_token().clone();
            let right = self.parse_factor()?;
            
            let operator = match operator.token_type {
                TokenType::Plus => BinaryOperator::Add,
                TokenType::Minus => BinaryOperator::Subtract,
                _ => return Err(Error::ParseError("Invalid operator".to_string())),
            };
            
            left = Node::BinaryOp {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    fn parse_factor(&mut self) -> Result<Node> {
        let mut left = self.parse_unary()?;
        
        while self.check(TokenType::Multiply) || self.check(TokenType::Divide) {
            let operator = self.advance_token().clone();
            let right = self.parse_unary()?;
            
            let operator = match operator.token_type {
                TokenType::Multiply => BinaryOperator::Multiply,
                TokenType::Divide => BinaryOperator::Divide,
                _ => return Err(Error::ParseError("Invalid operator".to_string())),
            };
            
            left = Node::BinaryOp {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    fn parse_unary(&mut self) -> Result<Node> {
        if self.check(TokenType::Minus) {
            self.advance_token();
            let operand = self.parse_unary()?;
            return Ok(Node::BinaryOp {
                left: Box::new(Node::Integer(0)),
                operator: BinaryOperator::Subtract,
                right: Box::new(operand),
            });
        }
        
        self.parse_primary()
    }
    
    fn parse_primary(&mut self) -> Result<Node> {
        match &self.current_token.as_ref().unwrap().token_type {
            TokenType::String(s) => {
                let value = s.clone();
                self.advance_token();
                Ok(Node::String(value))
            }
            TokenType::Int(i) => {
                let value = *i;
                self.advance_token();
                Ok(Node::Integer(value))
            }
            TokenType::Float(f) => {
                let value = *f;
                self.advance_token();
                Ok(Node::Float(value))
            }
            TokenType::Bool(b) => {
                let value = *b;
                self.advance_token();
                Ok(Node::Boolean(value))
            }
            TokenType::Identifier(name) => {
                let name = name.clone();
                self.advance_token();
                Ok(Node::Identifier(name))
            }
            TokenType::LeftParen => {
                self.advance_token();
                let expr = self.parse_expression()?;
                self.consume(TokenType::RightParen)?;
                Ok(expr)
            }
            _ => Err(Error::ParseError(format!("Unexpected token: {:?}", self.current_token.as_ref().unwrap().token_type))),
        }
    }
    
    fn consume(&mut self, expected: TokenType) -> Result<()> {
        if self.check(expected.clone()) {
            self.advance_token();
            Ok(())
        } else {
            Err(Error::ParseError(format!("Expected {:?}, found {:?}", expected, self.current_token.as_ref().unwrap().token_type)))
        }
    }
    
    fn consume_identifier(&mut self) -> Result<String> {
        match &self.current_token.as_ref().unwrap().token_type {
            TokenType::Identifier(name) => {
                let name = name.clone();
                self.advance_token();
                Ok(name)
            }
            _ => Err(Error::ParseError("Expected identifier".to_string())),
        }
    }
    
    fn check(&self, token_type: TokenType) -> bool {
        self.current_token.as_ref().map_or(false, |token| 
            token.token_type == token_type)
    }
    
    fn advance_token(&mut self) -> &Token {
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
    
    fn is_at_end(&self) -> bool {
        self.current_token.as_ref().map_or(true, |token| 
            matches!(token.token_type, TokenType::EOF))
    }
}