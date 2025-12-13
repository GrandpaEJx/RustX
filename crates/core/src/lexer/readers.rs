use super::Lexer;
use super::token::Token;

/// Skips whitespace (except newlines, which are significant)
pub(crate) fn skip_whitespace(lexer: &mut Lexer) {
    while let Some(ch) = lexer.current_char {
        if ch == ' ' || ch == '\t' || ch == '\r' {
            lexer.advance();
        } else {
            break;
        }
    }
}

/// Skips single-line comments starting with //
pub(crate) fn skip_line_comment(lexer: &mut Lexer) {
    while let Some(ch) = lexer.current_char {
        if ch == '\n' {
            break;
        }
        lexer.advance();
    }
}

/// Skips multi-line comments /* ... */
pub(crate) fn skip_block_comment(lexer: &mut Lexer) -> Result<(), String> {
    lexer.advance(); // skip '/'
    lexer.advance(); // skip '*'
    
    while let Some(ch) = lexer.current_char {
        if ch == '*' {
            lexer.advance();
            if lexer.current_char == Some('/') {
                lexer.advance();
                return Ok(());
            }
        } else {
            lexer.advance();
        }
    }
    
    Err("Unterminated block comment".to_string())
}

/// Reads a number (integer or float)
pub(crate) fn read_number(lexer: &mut Lexer) -> Token {
    let mut num_str = String::new();
    let mut is_float = false;

    while let Some(ch) = lexer.current_char {
        if ch.is_ascii_digit() {
            num_str.push(ch);
            lexer.advance();
        } else if ch == '.' && !is_float {
            // Only treat as decimal point if followed by a digit
            if let Some(&next_ch) = lexer.peek() {
                if next_ch.is_ascii_digit() {
                    is_float = true;
                    num_str.push(ch);
                    lexer.advance();
                } else {
                    // It's a method call, not a float
                    break;
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }

    if is_float {
        Token::Float(num_str.parse().unwrap_or(0.0))
    } else {
        Token::Int(num_str.parse().unwrap_or(0))
    }
}

/// Reads an identifier or keyword
pub(crate) fn read_identifier(lexer: &mut Lexer) -> Token {
    let mut ident = String::new();

    while let Some(ch) = lexer.current_char {
        if ch.is_alphanumeric() || ch == '_' {
            ident.push(ch);
            lexer.advance();
        } else {
            break;
        }
    }

    Token::is_keyword(&ident).unwrap_or(Token::Ident(ident))
}

/// Reads a string literal
pub(crate) fn read_string(lexer: &mut Lexer) -> Result<Token, String> {
    lexer.advance(); // skip opening quote
    let mut string = String::new();

    while let Some(ch) = lexer.current_char {
        if ch == '"' {
            lexer.advance();
            return Ok(Token::String(string));
        } else if ch == '\\' {
            lexer.advance();
            match lexer.current_char {
                Some('n') => string.push('\n'),
                Some('t') => string.push('\t'),
                Some('r') => string.push('\r'),
                Some('\\') => string.push('\\'),
                Some('"') => string.push('"'),
                _ => return Err("Invalid escape sequence".to_string()),
            }
            lexer.advance();
        } else {
            string.push(ch);
            lexer.advance();
        }
    }

    Err("Unterminated string".to_string())
}

/// Reads a template string literal (backtick strings)
pub(crate) fn read_template_string(lexer: &mut Lexer) -> Result<Token, String> {
    lexer.advance(); // skip opening backtick
    let mut string = String::new();

    while let Some(ch) = lexer.current_char {
        if ch == '`' {
            lexer.advance();
            return Ok(Token::TemplateString(string));
        } else if ch == '\\' {
            lexer.advance();
            match lexer.current_char {
                Some('n') => string.push('\n'),
                Some('t') => string.push('\t'),
                Some('r') => string.push('\r'),
                Some('\\') => string.push('\\'),
                Some('`') => string.push('`'),
                _ => return Err("Invalid escape sequence".to_string()),
            }
            lexer.advance();
        } else {
            string.push(ch);
            lexer.advance();
        }
    }

    Err("Unterminated template string".to_string())
}
