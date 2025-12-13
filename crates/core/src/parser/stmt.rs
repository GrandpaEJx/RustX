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
        Token::Use => parse_use(parser),
        Token::Rust => parse_rust_block(parser),
        Token::Let => parse_declaration(parser),
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

/// Parses a variable declaration
fn parse_declaration(parser: &mut Parser) -> Result<Stmt, String> {
    parser.advance(); // consume 'let'
    
    let name = match parser.current_token() {
        Token::Ident(n) => n.clone(),
        _ => return Err("Expected variable name".to_string()),
    };
    parser.advance();

    parser.expect(Token::Eq)?;
    
    let value = parser.parse_expression()?;
    
    Ok(Stmt::Let { name, value })
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
    
    let (path, alias) = match parser.current_token() {
        // import "file.rsx" as module
        Token::String(s) => {
            let path = s.clone();
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
            (path, alias)
        }
        // import web (stdlib module)
        Token::Ident(name) => {
            let module_name = name.clone();
            parser.advance();
            (module_name.clone(), Some(module_name))
        }
        _ => return Err("Expected import path (string or identifier)".to_string()),
    };

    Ok(Stmt::Import { path, alias })
}

/// Parses a use statement (e.g., use crate "rand" = "0.8")
fn parse_use(parser: &mut Parser) -> Result<Stmt, String> {
    parser.advance(); // consume 'use'
    
    parser.expect(Token::Crate)?; // consume 'crate'
    
    let crate_name = match parser.current_token() {
        Token::String(s) => s.clone(),
        _ => return Err("Expected crate name string".to_string()),
    };
    parser.advance();
    
    parser.expect(Token::Eq)?;
    
    let version = match parser.current_token() {
        Token::String(s) => s.clone(),
        _ => return Err("Expected version string".to_string()),
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

    Ok(Stmt::RustImport { crate_name, version, alias })
}

/// Parses a rust block
fn parse_rust_block(parser: &mut Parser) -> Result<Stmt, String> {
    parser.advance(); // consume 'rust'
    
    parser.expect(Token::LBrace)?;
    
    // For now, we'll just consume tokens until RBrace and reconstruct the string.
    
    let mut code = String::new();
    let mut brace_count = 1;
    
    while brace_count > 0 {
        match parser.current_token() {
            Token::LBrace => {
                brace_count += 1;
                code.push_str(" { ");
                parser.advance();
            }
            Token::RBrace => {
                brace_count -= 1;
                if brace_count > 0 {
                    code.push_str(" } ");
                    parser.advance();
                }
            }
            Token::Eof => return Err("Unterminated rust block".to_string()),
            token => {
                // Approximate reconstruction
                let s = token_to_string(token);
                code.push_str(&s);
                code.push(' ');
                parser.advance();
            }
        }
    }
    
    parser.advance(); // consume final RBrace
    
    Ok(Stmt::RustBlock { code })
}

fn token_to_string(token: &Token) -> String {
    match token {
        Token::Int(n) => n.to_string(),
        Token::Float(f) => f.to_string(),
        Token::String(s) => format!("\"{}\"", s), // Quote strings
        Token::TemplateString(s) => format!("`{}`", s),
        Token::Bool(b) => b.to_string(),
        Token::Ident(s) => s.clone(),
        Token::Fn => "fn".to_string(),
        Token::If => "if".to_string(),
        Token::Else => "else".to_string(),
        Token::While => "while".to_string(),
        Token::For => "for".to_string(),
        Token::In => "in".to_string(),
        Token::Return => "return".to_string(),
        Token::Import => "import".to_string(),
        Token::Use => "use".to_string(),
        Token::Crate => "crate".to_string(),
        Token::Rust => "rust".to_string(),
        Token::Let => "let".to_string(), // Ensure Let is handled if we use token_to_string for debugging
        Token::As => "as".to_string(),
        Token::Plus => "+".to_string(),
        Token::Minus => "-".to_string(),
        Token::Star => "*".to_string(),
        Token::Slash => "/".to_string(),
        Token::Percent => "%".to_string(),
        Token::Eq => "=".to_string(),
        Token::EqEq => "==".to_string(),
        Token::NotEq => "!=".to_string(),
        Token::Lt => "<".to_string(),
        Token::Gt => ">".to_string(),
        Token::LtEq => "<=".to_string(),
        Token::GtEq => ">=".to_string(),
        Token::And => "&&".to_string(),
        Token::Or => "||".to_string(),
        Token::Not => "!".to_string(),
        Token::Arrow => "=>".to_string(),
        Token::ThinArrow => "->".to_string(),
        Token::DoubleColon => "::".to_string(),
        Token::Hash => "#".to_string(),
        Token::Question => "?".to_string(),
        Token::LParen => "(".to_string(),
        Token::RParen => ")".to_string(),
        Token::LBrace => "{".to_string(),
        Token::RBrace => "}".to_string(),
        Token::LBracket => "[".to_string(),
        Token::RBracket => "]".to_string(),
        Token::Comma => ",".to_string(),
        Token::Colon => ":".to_string(),
        Token::Semicolon => ";".to_string(),
        Token::Dot => ".".to_string(),
        Token::Newline => "\n".to_string(),
        Token::Eof => "".to_string(),
    }
}
