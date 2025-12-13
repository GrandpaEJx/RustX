use crate::token::Token;
use std::iter::Peekable;
use std::str::Chars;

/// Lexer for tokenizing RustX source code
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    /// Creates a new Lexer from source code
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input: input.chars().peekable(),
            current_char: None,
        };
        lexer.advance();
        lexer
    }

    /// Advances to the next character
    fn advance(&mut self) {
        self.current_char = self.input.next();
    }

    /// Peeks at the next character without consuming it
    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    /// Skips whitespace (except newlines, which are significant)
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch == ' ' || ch == '\t' || ch == '\r' {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Skips single-line comments starting with //
    fn skip_line_comment(&mut self) {
        while let Some(ch) = self.current_char {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
    }

    /// Skips multi-line comments /* ... */
    fn skip_block_comment(&mut self) -> Result<(), String> {
        self.advance(); // skip '/'
        self.advance(); // skip '*'
        
        while let Some(ch) = self.current_char {
            if ch == '*' {
                self.advance();
                if self.current_char == Some('/') {
                    self.advance();
                    return Ok(());
                }
            } else {
                self.advance();
            }
        }
        
        Err("Unterminated block comment".to_string())
    }

    /// Reads a number (integer or float)
    fn read_number(&mut self) -> Token {
        let mut num_str = String::new();
        let mut is_float = false;

        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() {
                num_str.push(ch);
                self.advance();
            } else if ch == '.' && !is_float {
                if let Some(&next_ch) = self.peek() {
                    if next_ch.is_ascii_digit() {
                        is_float = true;
                        num_str.push(ch);
                        self.advance();
                    } else {
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
    fn read_identifier(&mut self) -> Token {
        let mut ident = String::new();

        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        Token::is_keyword(&ident).unwrap_or(Token::Ident(ident))
    }

    /// Reads a string literal
    fn read_string(&mut self) -> Result<Token, String> {
        self.advance(); // skip opening quote
        let mut string = String::new();

        while let Some(ch) = self.current_char {
            if ch == '"' {
                self.advance();
                return Ok(Token::String(string));
            } else if ch == '\\' {
                self.advance();
                match self.current_char {
                    Some('n') => string.push('\n'),
                    Some('t') => string.push('\t'),
                    Some('r') => string.push('\r'),
                    Some('\\') => string.push('\\'),
                    Some('"') => string.push('"'),
                    _ => return Err("Invalid escape sequence".to_string()),
                }
                self.advance();
            } else {
                string.push(ch);
                self.advance();
            }
        }

        Err("Unterminated string".to_string())
    }

    /// Reads a template string literal (backtick strings)
    fn read_template_string(&mut self) -> Result<Token, String> {
        self.advance(); // skip opening backtick
        let mut string = String::new();

        while let Some(ch) = self.current_char {
            if ch == '`' {
                self.advance();
                return Ok(Token::TemplateString(string));
            } else if ch == '\\' {
                self.advance();
                match self.current_char {
                    Some('n') => string.push('\n'),
                    Some('t') => string.push('\t'),
                    Some('r') => string.push('\r'),
                    Some('\\') => string.push('\\'),
                    Some('`') => string.push('`'),
                    _ => return Err("Invalid escape sequence".to_string()),
                }
                self.advance();
            } else {
                string.push(ch);
                self.advance();
            }
        }

        Err("Unterminated template string".to_string())
    }

    /// Gets the next token
    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();

        match self.current_char {
            None => Ok(Token::Eof),
            Some('\n') => {
                self.advance();
                Ok(Token::Newline)
            }
            Some(ch) => {
                // Comments
                if ch == '/' {
                    if let Some(&next_ch) = self.peek() {
                        if next_ch == '/' {
                            self.skip_line_comment();
                            return self.next_token();
                        } else if next_ch == '*' {
                            self.skip_block_comment()?;
                            return self.next_token();
                        }
                    }
                }

                // Numbers
                if ch.is_ascii_digit() {
                    return Ok(self.read_number());
                }

                // Identifiers and keywords
                if ch.is_alphabetic() || ch == '_' {
                    return Ok(self.read_identifier());
                }

                // String literals
                if ch == '"' {
                    return self.read_string();
                }

                // Template string literals
                if ch == '`' {
                    return self.read_template_string();
                }

                // Operators and delimiters
                let token = match ch {
                    '+' => Token::Plus,
                    '-' => Token::Minus,
                    '*' => Token::Star,
                    '/' => Token::Slash,
                    '%' => Token::Percent,
                    '(' => Token::LParen,
                    ')' => Token::RParen,
                    '{' => Token::LBrace,
                    '}' => Token::RBrace,
                    '[' => Token::LBracket,
                    ']' => Token::RBracket,
                    ',' => Token::Comma,
                    ':' => Token::Colon,
                    ';' => Token::Semicolon,
                    '!' => {
                        if let Some(&'=') = self.peek() {
                            self.advance();
                            self.advance();
                            return Ok(Token::NotEq);
                        }
                        Token::Not
                    }
                    '=' => {
                        if let Some(&next) = self.peek() {
                            if next == '=' {
                                self.advance();
                                self.advance();
                                return Ok(Token::EqEq);
                            } else if next == '>' {
                                self.advance();
                                self.advance();
                                return Ok(Token::Arrow);
                            }
                        }
                        Token::Eq
                    }
                    '<' => {
                        if let Some(&'=') = self.peek() {
                            self.advance();
                            self.advance();
                            return Ok(Token::LtEq);
                        }
                        Token::Lt
                    }
                    '>' => {
                        if let Some(&'=') = self.peek() {
                            self.advance();
                            self.advance();
                            return Ok(Token::GtEq);
                        }
                        Token::Gt
                    }
                    '&' => {
                        if let Some(&'&') = self.peek() {
                            self.advance();
                            self.advance();
                            return Ok(Token::And);
                        }
                        return Err(format!("Unexpected character: {}", ch));
                    }
                    '|' => {
                        if let Some(&'|') = self.peek() {
                            self.advance();
                            self.advance();
                            return Ok(Token::Or);
                        }
                        return Err(format!("Unexpected character: {}", ch));
                    }
                    _ => return Err(format!("Unexpected character: {}", ch)),
                };

                self.advance();
                Ok(token)
            }
        }
    }

    /// Tokenizes the entire input into a vector of tokens
    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        
        loop {
            let token = self.next_token()?;
            if token == Token::Eof {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }
        
        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let input = "x = 10 + 20";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0], Token::Ident("x".to_string()));
        assert_eq!(tokens[1], Token::Eq);
        assert_eq!(tokens[2], Token::Int(10));
        assert_eq!(tokens[3], Token::Plus);
        assert_eq!(tokens[4], Token::Int(20));
    }

    #[test]
    fn test_comments() {
        let input = "// comment\nx = 5";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0], Token::Newline);
        assert_eq!(tokens[1], Token::Ident("x".to_string()));
    }

    #[test]
    fn test_block_comment() {
        let input = "/* block\ncomment */ x = 5";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0], Token::Ident("x".to_string()));
    }
}
