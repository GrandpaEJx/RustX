use crate::ast::{Stmt, Expr};
use crate::Token;
use super::Parser;

/// Parses a statement
pub fn parse_statement(parser: &mut Parser) -> Result<Stmt, String> {
    parser.skip_newlines();

    match parser.current_token() {
        Token::Fn => parse_function(parser),
        Token::Return => parse_return(parser),
        Token::While => parse_while(parser),
        Token::For => parse_for(parser),
        Token::Import => parse_import(parser),
        _ => {
            // Try to parse as assignment or expression
            let expr = parser.parse_expression()?;
            
            // Check for assignment
            if matches!(parser.current_token(), Token::Eq) {
                if let Expr::Ident(name) = expr {
                    parser.advance(); // consume '='
                    let value = parser.parse_expression()?;
                    return Ok(Stmt::Let { name, value });
                }
            }
            
            Ok(Stmt::Expr(expr))
        }
    }
}

/// Parses a function declaration
fn parse_function(parser: &mut Parser) -> Result<Stmt, String> {
    parser.advance(); // consume 'fn'
    
    let name = match parser.current_token() {
        Token::Ident(n) => n.clone(),
        _ => return Err("Expected function name".to_string()),
    };
    parser.advance();

    parser.expect(Token::LParen)?;
    let mut params = Vec::new();

    while !matches!(parser.current_token(), Token::RParen) {
        if let Token::Ident(param) = parser.current_token() {
            params.push(param.clone());
            parser.advance();
            
            if matches!(parser.current_token(), Token::Comma) {
                parser.advance();
            }
        } else {
            return Err("Expected parameter name".to_string());
        }
    }

    parser.expect(Token::RParen)?;

    // Check for arrow function
    let body = if matches!(parser.current_token(), Token::Arrow) {
        parser.advance();
        Box::new(parser.parse_expression()?)
    } else {
        Box::new(parser.parse_block_expr()?)
    };

    Ok(Stmt::Function { name, params, body })
}

/// Parses a return statement
fn parse_return(parser: &mut Parser) -> Result<Stmt, String> {
    parser.advance(); // consume 'return'
    
    if matches!(parser.current_token(), Token::Newline | Token::Eof) {
        Ok(Stmt::Return(None))
    } else {
        Ok(Stmt::Return(Some(parser.parse_expression()?)))
    }
}

/// Parses a while loop
fn parse_while(parser: &mut Parser) -> Result<Stmt, String> {
    parser.advance(); // consume 'while'
    let condition = parser.parse_expression()?;
    let body = Box::new(parser.parse_block_expr()?);
    Ok(Stmt::While { condition, body })
}

/// Parses a for loop
fn parse_for(parser: &mut Parser) -> Result<Stmt, String> {
    parser.advance(); // consume 'for'
    
    let iterator = match parser.current_token() {
        Token::Ident(name) => name.clone(),
        _ => return Err("Expected iterator variable".to_string()),
    };
    parser.advance();

    parser.expect(Token::In)?;
    let iterable = parser.parse_expression()?;
    let body = Box::new(parser.parse_block_expr()?);

    Ok(Stmt::For {
        iterator,
        iterable,
        body,
    })
}

/// Parses an import statement
fn parse_import(parser: &mut Parser) -> Result<Stmt, String> {
    parser.advance(); // consume 'import'
    
    let path = match parser.current_token() {
        Token::String(s) => s.clone(),
        _ => return Err("Expected import path string".to_string()),
    };
    parser.advance();

    let alias = if matches!(parser.current_token(), Token::As) {
        parser.advance();
        match parser.current_token() {
            Token::Ident(name) => {
                let alias_name = name.clone();
                parser.advance();
                Some(alias_name)
            }
            _ => return Err("Expected alias name".to_string()),
        }
    } else {
        None
    };

    Ok(Stmt::Import { path, alias })
}
