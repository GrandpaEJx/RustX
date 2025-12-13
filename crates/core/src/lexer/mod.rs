use std::iter::Peekable;
use std::str::Chars;

pub mod token;
pub mod readers;

use token::Token;

/// Lexer for tokenizing RustX source code
pub struct Lexer<'a> {
    pub(crate) input: Peekable<Chars<'a>>,
    pub(crate) current_char: Option<char>,
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
    pub(crate) fn advance(&mut self) {
        self.current_char = self.input.next();
    }

    /// Peeks at the next character without consuming it
    pub(crate) fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    /// Gets the next token
    pub fn next_token(&mut self) -> Result<Token, String> {
        readers::skip_whitespace(self);

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
                            readers::skip_line_comment(self);
                            return self.next_token();
                        } else if next_ch == '*' {
                            readers::skip_block_comment(self)?;
                            return self.next_token();
                        }
                    }
                }

                // Numbers
                if ch.is_ascii_digit() {
                    return Ok(readers::read_number(self));
                }

                // Identifiers and keywords
                if ch.is_alphabetic() || ch == '_' {
                    return Ok(readers::read_identifier(self));
                }

                // String literals
                if ch == '"' {
                    return readers::read_string(self);
                }

                // Template string literals
                if ch == '`' {
                    return readers::read_template_string(self);
                }

                // Operators and delimiters
                let token = match ch {
                    '+' => Token::Plus,
                    '-' => {
                         if let Some(&'>') = self.peek() {
                             self.advance();
                             self.advance();
                             return Ok(Token::ThinArrow);
                         }
                         Token::Minus
                    }
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
                    ':' => {
                         if let Some(&':') = self.peek() {
                             self.advance();
                             self.advance();
                             return Ok(Token::DoubleColon);
                         }
                         Token::Colon
                    }
                    ';' => Token::Semicolon,
                    '.' => Token::Dot,
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
                    '#' => Token::Hash,
                    '?' => Token::Question,
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
