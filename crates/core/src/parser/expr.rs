use crate::ast::{BinaryOp, Expr, UnaryOp};
use crate::Token;
use super::Parser;

/// Parses an expression
pub fn parse_expression(parser: &mut Parser) -> Result<Expr, String> {
    parse_logical_or(parser)
}

/// Parses logical OR
fn parse_logical_or(parser: &mut Parser) -> Result<Expr, String> {
    let mut left = parse_logical_and(parser)?;

    while matches!(parser.current_token(), Token::Or) {
        parser.advance();
        let right = parse_logical_and(parser)?;
        left = Expr::Binary {
            left: Box::new(left),
            op: BinaryOp::Or,
            right: Box::new(right),
        };
    }

    Ok(left)
}

/// Parses logical AND
fn parse_logical_and(parser: &mut Parser) -> Result<Expr, String> {
    let mut left = parse_equality(parser)?;

    while matches!(parser.current_token(), Token::And) {
        parser.advance();
        let right = parse_equality(parser)?;
        left = Expr::Binary {
            left: Box::new(left),
            op: BinaryOp::And,
            right: Box::new(right),
        };
    }

    Ok(left)
}

/// Parses equality operators
fn parse_equality(parser: &mut Parser) -> Result<Expr, String> {
    let mut left = parse_comparison(parser)?;

    while let Token::EqEq | Token::NotEq = parser.current_token() {
        let op = match parser.current_token() {
            Token::EqEq => BinaryOp::Eq,
            Token::NotEq => BinaryOp::NotEq,
            _ => unreachable!(),
        };
        parser.advance();
        let right = parse_comparison(parser)?;
        left = Expr::Binary {
            left: Box::new(left),
            op,
            right: Box::new(right),
        };
    }

    Ok(left)
}

/// Parses comparison operators
fn parse_comparison(parser: &mut Parser) -> Result<Expr, String> {
    let mut left = parse_term(parser)?;

    while let Token::Lt | Token::Gt | Token::LtEq | Token::GtEq = parser.current_token() {
        let op = match parser.current_token() {
            Token::Lt => BinaryOp::Lt,
            Token::Gt => BinaryOp::Gt,
            Token::LtEq => BinaryOp::LtEq,
            Token::GtEq => BinaryOp::GtEq,
            _ => unreachable!(),
        };
        parser.advance();
        let right = parse_term(parser)?;
        left = Expr::Binary {
            left: Box::new(left),
            op,
            right: Box::new(right),
        };
    }

    Ok(left)
}

/// Parses addition and subtraction
fn parse_term(parser: &mut Parser) -> Result<Expr, String> {
    let mut left = parse_factor(parser)?;

    while let Token::Plus | Token::Minus = parser.current_token() {
        let op = match parser.current_token() {
            Token::Plus => BinaryOp::Add,
            Token::Minus => BinaryOp::Sub,
            _ => unreachable!(),
        };
        parser.advance();
        let right = parse_factor(parser)?;
        left = Expr::Binary {
            left: Box::new(left),
            op,
            right: Box::new(right),
        };
    }

    Ok(left)
}

/// Parses multiplication, division, and modulo
fn parse_factor(parser: &mut Parser) -> Result<Expr, String> {
    let mut left = parse_unary(parser)?;

    while let Token::Star | Token::Slash | Token::Percent = parser.current_token() {
        let op = match parser.current_token() {
            Token::Star => BinaryOp::Mul,
            Token::Slash => BinaryOp::Div,
            Token::Percent => BinaryOp::Mod,
            _ => unreachable!(),
        };
        parser.advance();
        let right = parse_unary(parser)?;
        left = Expr::Binary {
            left: Box::new(left),
            op,
            right: Box::new(right),
        };
    }

    Ok(left)
}

/// Parses unary operators
fn parse_unary(parser: &mut Parser) -> Result<Expr, String> {
    match parser.current_token() {
        Token::Not => {
            parser.advance();
            Ok(Expr::Unary {
                op: UnaryOp::Not,
                expr: Box::new(parse_unary(parser)?),
            })
        }
        Token::Minus => {
            parser.advance();
            Ok(Expr::Unary {
                op: UnaryOp::Neg,
                expr: Box::new(parse_unary(parser)?),
            })
        }
        _ => parse_postfix(parser),
    }
}

/// Parses postfix expressions (calls, indexing, method calls)
fn parse_postfix(parser: &mut Parser) -> Result<Expr, String> {
    let mut expr = parse_primary(parser)?;

    loop {
        match parser.current_token() {
            Token::LParen => {
                parser.advance();
                let mut args = Vec::new();

                while !matches!(parser.current_token(), Token::RParen) {
                    args.push(parse_expression(parser)?);
                    if matches!(parser.current_token(), Token::Comma) {
                        parser.advance();
                    }
                }

                parser.expect(Token::RParen)?;
                expr = Expr::Call {
                    callee: Box::new(expr),
                    args,
                };
            }
            Token::LBracket => {
                parser.advance();
                let index = parse_expression(parser)?;
                parser.expect(Token::RBracket)?;
                expr = Expr::Index {
                    object: Box::new(expr),
                    index: Box::new(index),
                };
            }
            Token::Dot => {
                parser.advance(); // consume '.'
                
                // Get method name
                let method = match parser.current_token() {
                    Token::Ident(name) => name.clone(),
                    _ => return Err("Expected method name after '.'".to_string()),
                };
                parser.advance();
                
                // Check for method call with parentheses
                let args = if matches!(parser.current_token(), Token::LParen) {
                    parser.advance();
                    let mut args = Vec::new();
                    
                    while !matches!(parser.current_token(), Token::RParen) {
                        args.push(parse_expression(parser)?);
                        if matches!(parser.current_token(), Token::Comma) {
                            parser.advance();
                        }
                    }
                    
                    parser.expect(Token::RParen)?;
                    args
                } else {
                    // Method call without parentheses (property-like)
                    Vec::new()
                };
                
                expr = Expr::MethodCall {
                    object: Box::new(expr),
                    method,
                    args,
                };
            }
            _ => break,
        }
    }

    Ok(expr)
}

/// Parses primary expressions
fn parse_primary(parser: &mut Parser) -> Result<Expr, String> {
    match parser.current_token().clone() {
        Token::Int(n) => {
            parser.advance();
            Ok(Expr::Int(n))
        }
        Token::Float(f) => {
            parser.advance();
            Ok(Expr::Float(f))
        }
        Token::String(s) => {
            parser.advance();
            Ok(Expr::String(s))
        }
        Token::TemplateString(s) => {
            parser.advance();
            Ok(Expr::TemplateString(s))
        }
        Token::Bool(b) => {
            parser.advance();
            Ok(Expr::Bool(b))
        }
        Token::Ident(name) => {
            parser.advance();
            Ok(Expr::Ident(name))
        }
        Token::LParen => {
            parser.advance();
            let expr = parse_expression(parser)?;
            parser.expect(Token::RParen)?;
            Ok(expr)
        }
        Token::LBracket => parse_array(parser),
        Token::LBrace => parse_block_or_map(parser),
        Token::If => parse_if(parser),
        _ => Err(format!("Unexpected token: {:?}", parser.current_token())),
    }
}

/// Parses an array literal
fn parse_array(parser: &mut Parser) -> Result<Expr, String> {
    parser.advance(); // consume '['
    let mut elements = Vec::new();

    while !matches!(parser.current_token(), Token::RBracket) {
        elements.push(parse_expression(parser)?);
        if matches!(parser.current_token(), Token::Comma) {
            parser.advance();
        }
    }

    parser.expect(Token::RBracket)?;
    Ok(Expr::Array(elements))
}

/// Parses a block expression or map literal
fn parse_block_or_map(parser: &mut Parser) -> Result<Expr, String> {
    parser.advance(); // consume '{'
    parser.skip_newlines();

    // Empty block
    if matches!(parser.current_token(), Token::RBrace) {
        parser.advance();
        return Ok(Expr::Block(vec![]));
    }

    // Try to determine if it's a map or block
    // If we see "string": expr, it's a map
    if let Token::String(_) = parser.current_token() {
        return parse_map_contents(parser);
    }

    // Otherwise, parse as block
    parse_block_contents(parser)
}

/// Parses map literal contents
fn parse_map_contents(parser: &mut Parser) -> Result<Expr, String> {
    let mut pairs = Vec::new();

    while !matches!(parser.current_token(), Token::RBrace) {
        let key = match parser.current_token() {
            Token::String(s) => s.clone(),
            _ => return Err("Expected string key in map".to_string()),
        };
        parser.advance();

        parser.expect(Token::Colon)?;
        let value = parse_expression(parser)?;
        pairs.push((key, value));

        if matches!(parser.current_token(), Token::Comma) {
            parser.advance();
            parser.skip_newlines();
        }
    }

    parser.expect(Token::RBrace)?;
    Ok(Expr::Map(pairs))
}

/// Parses block expression contents
fn parse_block_contents(parser: &mut Parser) -> Result<Expr, String> {
    let mut statements = Vec::new();

    while !matches!(parser.current_token(), Token::RBrace) {
        statements.push(parser.parse_statement()?);
        parser.skip_newlines();
    }

    parser.expect(Token::RBrace)?;
    Ok(Expr::Block(statements))
}

/// Parses a block expression (used for function bodies, loops, etc.)
pub fn parse_block_expr(parser: &mut Parser) -> Result<Expr, String> {
    parser.expect(Token::LBrace)?;
    parse_block_contents(parser)
}

/// Parses an if expression
fn parse_if(parser: &mut Parser) -> Result<Expr, String> {
    parser.advance(); // consume 'if'
    let condition = Box::new(parse_expression(parser)?);
    let then_branch = Box::new(parse_block_expr(parser)?);

    let else_branch = if matches!(parser.current_token(), Token::Else) {
        parser.advance();
        Some(Box::new(if matches!(parser.current_token(), Token::If) {
            parse_if(parser)?
        } else {
            parse_block_expr(parser)?
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
