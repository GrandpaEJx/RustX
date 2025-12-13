use crate::ast::{BinaryOp, Expr, Stmt, UnaryOp};
use crate::token::Token;

/// Parser for RustX language
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    /// Creates a new parser from a token stream
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    /// Returns the current token
    fn current_token(&self) -> &Token {
        self.tokens.get(self.current).unwrap_or(&Token::Eof)
    }

    /// Advances to the next token
    fn advance(&mut self) -> &Token {
        if self.current < self.tokens.len() {
            self.current += 1;
        }
        self.current_token()
    }

    /// Checks if current token matches expected type
    fn check(&self, token: &Token) -> bool {
        std::mem::discriminant(self.current_token()) == std::mem::discriminant(token)
    }

    /// Consumes token if it matches, otherwise returns error
    fn expect(&mut self, token: Token) -> Result<(), String> {
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
    fn skip_newlines(&mut self) {
        while matches!(self.current_token(), Token::Newline) {
            self.advance();
        }
    }

    /// Parses a program (list of statements)
    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();
        self.skip_newlines();

        while !matches!(self.current_token(), Token::Eof) {
            statements.push(self.parse_statement()?);
            self.skip_newlines();
        }

        Ok(statements)
    }

    /// Parses a statement
    fn parse_statement(&mut self) -> Result<Stmt, String> {
        self.skip_newlines();

        match self.current_token() {
            Token::Fn => self.parse_function(),
            Token::Return => self.parse_return(),
            Token::While => self.parse_while(),
            Token::For => self.parse_for(),
            Token::Import => self.parse_import(),
            _ => {
                // Try to parse as assignment or expression
                let expr = self.parse_expression()?;
                
                // Check for assignment
                if matches!(self.current_token(), Token::Eq) {
                    if let Expr::Ident(name) = expr {
                        self.advance(); // consume '='
                        let value = self.parse_expression()?;
                        return Ok(Stmt::Let { name, value });
                    }
                }
                
                Ok(Stmt::Expr(expr))
            }
        }
    }

    /// Parses a function declaration
    fn parse_function(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'fn'
        
        let name = match self.current_token() {
            Token::Ident(n) => n.clone(),
            _ => return Err("Expected function name".to_string()),
        };
        self.advance();

        self.expect(Token::LParen)?;
        let mut params = Vec::new();

        while !matches!(self.current_token(), Token::RParen) {
            if let Token::Ident(param) = self.current_token() {
                params.push(param.clone());
                self.advance();
                
                if matches!(self.current_token(), Token::Comma) {
                    self.advance();
                }
            } else {
                return Err("Expected parameter name".to_string());
            }
        }

        self.expect(Token::RParen)?;

        // Check for arrow function
        let body = if matches!(self.current_token(), Token::Arrow) {
            self.advance();
            Box::new(self.parse_expression()?)
        } else {
            Box::new(self.parse_block_expr()?)
        };

        Ok(Stmt::Function { name, params, body })
    }

    /// Parses a return statement
    fn parse_return(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'return'
        
        if matches!(self.current_token(), Token::Newline | Token::Eof) {
            Ok(Stmt::Return(None))
        } else {
            Ok(Stmt::Return(Some(self.parse_expression()?)))
        }
    }

    /// Parses a while loop
    fn parse_while(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'while'
        let condition = self.parse_expression()?;
        let body = Box::new(self.parse_block_expr()?);
        Ok(Stmt::While { condition, body })
    }

    /// Parses a for loop
    fn parse_for(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'for'
        
        let iterator = match self.current_token() {
            Token::Ident(name) => name.clone(),
            _ => return Err("Expected iterator variable".to_string()),
        };
        self.advance();

        self.expect(Token::In)?;
        let iterable = self.parse_expression()?;
        let body = Box::new(self.parse_block_expr()?);

        Ok(Stmt::For {
            iterator,
            iterable,
            body,
        })
    }

    /// Parses an import statement
    fn parse_import(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'import'
        
        let path = match self.current_token() {
            Token::String(s) => s.clone(),
            _ => return Err("Expected import path string".to_string()),
        };
        self.advance();

        let alias = if matches!(self.current_token(), Token::As) {
            self.advance();
            match self.current_token() {
                Token::Ident(name) => {
                    let alias_name = name.clone();
                    self.advance();
                    Some(alias_name)
                }
                _ => return Err("Expected alias name".to_string()),
            }
        } else {
            None
        };

        Ok(Stmt::Import { path, alias })
    }

    /// Parses an expression
    fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_logical_or()
    }

    /// Parses logical OR
    fn parse_logical_or(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_logical_and()?;

        while matches!(self.current_token(), Token::Or) {
            self.advance();
            let right = self.parse_logical_and()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::Or,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parses logical AND
    fn parse_logical_and(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_equality()?;

        while matches!(self.current_token(), Token::And) {
            self.advance();
            let right = self.parse_equality()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::And,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parses equality operators
    fn parse_equality(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_comparison()?;

        while let Token::EqEq | Token::NotEq = self.current_token() {
            let op = match self.current_token() {
                Token::EqEq => BinaryOp::Eq,
                Token::NotEq => BinaryOp::NotEq,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_comparison()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parses comparison operators
    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_term()?;

        while let Token::Lt | Token::Gt | Token::LtEq | Token::GtEq = self.current_token() {
            let op = match self.current_token() {
                Token::Lt => BinaryOp::Lt,
                Token::Gt => BinaryOp::Gt,
                Token::LtEq => BinaryOp::LtEq,
                Token::GtEq => BinaryOp::GtEq,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_term()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parses addition and subtraction
    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_factor()?;

        while let Token::Plus | Token::Minus = self.current_token() {
            let op = match self.current_token() {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Sub,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_factor()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parses multiplication, division, and modulo
    fn parse_factor(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;

        while let Token::Star | Token::Slash | Token::Percent = self.current_token() {
            let op = match self.current_token() {
                Token::Star => BinaryOp::Mul,
                Token::Slash => BinaryOp::Div,
                Token::Percent => BinaryOp::Mod,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_unary()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parses unary operators
    fn parse_unary(&mut self) -> Result<Expr, String> {
        match self.current_token() {
            Token::Not => {
                self.advance();
                Ok(Expr::Unary {
                    op: UnaryOp::Not,
                    expr: Box::new(self.parse_unary()?),
                })
            }
            Token::Minus => {
                self.advance();
                Ok(Expr::Unary {
                    op: UnaryOp::Neg,
                    expr: Box::new(self.parse_unary()?),
                })
            }
            _ => self.parse_postfix(),
        }
    }

    /// Parses postfix expressions (calls, indexing)
    fn parse_postfix(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;

        loop {
            match self.current_token() {
                Token::LParen => {
                    self.advance();
                    let mut args = Vec::new();

                    while !matches!(self.current_token(), Token::RParen) {
                        args.push(self.parse_expression()?);
                        if matches!(self.current_token(), Token::Comma) {
                            self.advance();
                        }
                    }

                    self.expect(Token::RParen)?;
                    expr = Expr::Call {
                        callee: Box::new(expr),
                        args,
                    };
                }
                Token::LBracket => {
                    self.advance();
                    let index = self.parse_expression()?;
                    self.expect(Token::RBracket)?;
                    expr = Expr::Index {
                        object: Box::new(expr),
                        index: Box::new(index),
                    };
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    /// Parses primary expressions
    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.current_token().clone() {
            Token::Int(n) => {
                self.advance();
                Ok(Expr::Int(n))
            }
            Token::Float(f) => {
                self.advance();
                Ok(Expr::Float(f))
            }
            Token::String(s) => {
                self.advance();
                Ok(Expr::String(s))
            }
            Token::TemplateString(s) => {
                self.advance();
                // Parser just passes the template string as-is
                // Interpolation happens at runtime in the interpreter
                Ok(Expr::TemplateString(s))
            }
            Token::Bool(b) => {
                self.advance();
                Ok(Expr::Bool(b))
            }
            Token::Ident(name) => {
                self.advance();
                Ok(Expr::Ident(name))
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            Token::LBracket => self.parse_array(),
            Token::LBrace => self.parse_block_or_map(),
            Token::If => self.parse_if(),
            _ => Err(format!("Unexpected token: {:?}", self.current_token())),
        }
    }

    /// Parses an array literal
    fn parse_array(&mut self) -> Result<Expr, String> {
        self.advance(); // consume '['
        let mut elements = Vec::new();

        while !matches!(self.current_token(), Token::RBracket) {
            elements.push(self.parse_expression()?);
            if matches!(self.current_token(), Token::Comma) {
                self.advance();
            }
        }

        self.expect(Token::RBracket)?;
        Ok(Expr::Array(elements))
    }

    /// Parses a block expression or map literal
    fn parse_block_or_map(&mut self) -> Result<Expr, String> {
        self.advance(); // consume '{'
        self.skip_newlines();

        // Empty block
        if matches!(self.current_token(), Token::RBrace) {
            self.advance();
            return Ok(Expr::Block(vec![]));
        }

        // Try to determine if it's a map or block
        // If we see "string": expr, it's a map
        if let Token::String(_) = self.current_token() {
            return self.parse_map_contents();
        }

        // Otherwise, parse as block
        self.parse_block_contents()
    }

    /// Parses map literal contents
    fn parse_map_contents(&mut self) -> Result<Expr, String> {
        let mut pairs = Vec::new();

        while !matches!(self.current_token(), Token::RBrace) {
            let key = match self.current_token() {
                Token::String(s) => s.clone(),
                _ => return Err("Expected string key in map".to_string()),
            };
            self.advance();

            self.expect(Token::Colon)?;
            let value = self.parse_expression()?;
            pairs.push((key, value));

            if matches!(self.current_token(), Token::Comma) {
                self.advance();
                self.skip_newlines();
            }
        }

        self.expect(Token::RBrace)?;
        Ok(Expr::Map(pairs))
    }

    /// Parses block expression contents
    fn parse_block_contents(&mut self) -> Result<Expr, String> {
        let mut statements = Vec::new();

        while !matches!(self.current_token(), Token::RBrace) {
            statements.push(self.parse_statement()?);
            self.skip_newlines();
        }

        self.expect(Token::RBrace)?;
        Ok(Expr::Block(statements))
    }

    /// Parses a block expression (used for function bodies, loops, etc.)
    fn parse_block_expr(&mut self) -> Result<Expr, String> {
        self.expect(Token::LBrace)?;
        self.parse_block_contents()
    }

    /// Parses an if expression
    fn parse_if(&mut self) -> Result<Expr, String> {
        self.advance(); // consume 'if'
        let condition = Box::new(self.parse_expression()?);
        let then_branch = Box::new(self.parse_block_expr()?);

        let else_branch = if matches!(self.current_token(), Token::Else) {
            self.advance();
            Some(Box::new(if matches!(self.current_token(), Token::If) {
                self.parse_if()?
            } else {
                self.parse_block_expr()?
            }))
        } else {
            None
        };

        Ok(Expr::If {
            condition,
            then_branch,
            else_branch,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_parse_simple_assignment() {
        let input = "x = 10";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        assert_eq!(ast.len(), 1);
        match &ast[0] {
            Stmt::Let { name, value } => {
                assert_eq!(name, "x");
                assert_eq!(*value, Expr::Int(10));
            }
            _ => panic!("Expected Let statement"),
        }
    }

    #[test]
    fn test_parse_binary_expr() {
        let input = "x = 10 + 20 * 2";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let _ast = parser.parse().unwrap();
        // Just verify it parses without error
    }

    #[test]
    fn test_parse_function() {
        let input = "fn add(a, b) => a + b";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        assert_eq!(ast.len(), 1);
        match &ast[0] {
            Stmt::Function { name, params, .. } => {
                assert_eq!(name, "add");
                assert_eq!(params.len(), 2);
            }
            _ => panic!("Expected Function statement"),
        }
    }
}
