use crate::ast::{BinaryOperator, Node};
use crate::error::Result;
use crate::lexer::TokenType;
use super::Parser;

impl Parser {
    pub fn parse_expression(&mut self) -> Result<Node> {
        self.parse_comparison()
    }

    pub fn parse_comparison(&mut self) -> Result<Node> {
        let mut left = self.parse_term()?;

        while self.check(TokenType::DoubleEquals) || self.check(TokenType::Equals) {
            let _operator = self.advance_token().clone();
            let right = self.parse_term()?;

            left = Node::BinaryOp {
                left: Box::new(left),
                operator: BinaryOperator::Equals,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    pub fn parse_term(&mut self) -> Result<Node> {
        let mut left = self.parse_factor()?;

        while self.check(TokenType::Plus) || self.check(TokenType::Minus) {
            let operator = self.advance_token().clone();
            let right = self.parse_factor()?;

            let operator = match operator.token_type {
                TokenType::Plus => BinaryOperator::Add,
                TokenType::Minus => BinaryOperator::Subtract,
                _ => return Err(crate::error::Error::ParseError("Invalid operator".to_string())),
            };

            left = Node::BinaryOp {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    pub fn parse_factor(&mut self) -> Result<Node> {
        let mut left = self.parse_unary()?;

        while self.check(TokenType::Multiply) || self.check(TokenType::Divide) {
            let operator = self.advance_token().clone();
            let right = self.parse_unary()?;

            let operator = match operator.token_type {
                TokenType::Multiply => BinaryOperator::Multiply,
                TokenType::Divide => BinaryOperator::Divide,
                _ => return Err(crate::error::Error::ParseError("Invalid operator".to_string())),
            };

            left = Node::BinaryOp {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    pub fn parse_unary(&mut self) -> Result<Node> {
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

    pub fn parse_primary(&mut self) -> Result<Node> {
        match &self.current_token.as_ref().unwrap().token_type {
            TokenType::Null => {
                self.advance_token();
                Ok(Node::Null)
            }
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

                // Check if this is a function call (identifier followed by left paren)
                if self.check(TokenType::LeftParen) {
                    self.advance_token(); // consume LeftParen

                    // Parse function call arguments
                    let mut arguments = Vec::new();

                    if !self.check(TokenType::RightParen) {
                        // Check if we have named arguments (identifier = value syntax)
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

                    Ok(Node::FunctionCall { name, arguments })
                } else {
                    Ok(Node::Identifier(name))
                }
            }
            TokenType::LeftParen => {
                self.advance_token();
                let expr = self.parse_expression()?;
                self.consume(TokenType::RightParen)?;
                Ok(expr)
            }
            _ => Err(crate::error::Error::ParseError(format!(
                "Unexpected token: {:?}",
                self.current_token.as_ref().unwrap().token_type
            ))),
        }
    }
}