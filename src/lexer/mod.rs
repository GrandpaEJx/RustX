use crate::error::{Error, Result};

pub mod token;
pub mod reader;
pub mod utils;

pub use token::{Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input,
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace();

        let start_line = self.line;
        let start_column = self.column;

        if self.is_at_end() {
            return Ok(Token {
                token_type: TokenType::EOF,
                line: start_line,
                column: start_column,
            });
        }

        let c = self.peek();

        // Identifiers and keywords
        if self.is_alpha(c) {
            return self.read_identifier(start_line, start_column);
        }

        // Numbers
        if self.is_digit(c) {
            return self.read_number(start_line, start_column);
        }

        // Strings
        if c == '"' {
            return self.read_string(start_line, start_column);
        }

        // Single character tokens
        self.advance();

        match c {
            '+' => Ok(self.make_token(TokenType::Plus, start_line, start_column)),
            '-' => {
                if self.peek() == '>' {
                    self.advance();
                    Ok(self.make_token(TokenType::Arrow, start_line, start_column))
                } else {
                    Ok(self.make_token(TokenType::Minus, start_line, start_column))
                }
            }
            '*' => Ok(self.make_token(TokenType::Multiply, start_line, start_column)),
            '/' => {
                if self.peek() == '/' {
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                    self.next_token()
                } else {
                    Ok(self.make_token(TokenType::Divide, start_line, start_column))
                }
            }
            '=' => {
                if self.peek() == '=' {
                    self.advance();
                    Ok(self.make_token(TokenType::DoubleEquals, start_line, start_column))
                } else {
                    Ok(self.make_token(TokenType::Equals, start_line, start_column))
                }
            }
            '(' => Ok(self.make_token(TokenType::LeftParen, start_line, start_column)),
            ')' => Ok(self.make_token(TokenType::RightParen, start_line, start_column)),
            '{' => Ok(self.make_token(TokenType::LeftBrace, start_line, start_column)),
            '}' => Ok(self.make_token(TokenType::RightBrace, start_line, start_column)),
            ';' => Ok(self.make_token(TokenType::Semicolon, start_line, start_column)),
            ',' => Ok(self.make_token(TokenType::Comma, start_line, start_column)),
            ':' => Ok(self.make_token(TokenType::Colon, start_line, start_column)),
            '&' => Ok(self.make_token(TokenType::Ampersand, start_line, start_column)),
            '\n' => {
                self.line += 1;
                self.column = 1;
                Ok(self.make_token(TokenType::Newline, start_line, start_column))
            }
            _ => Err(Error::LexerError { message: format!("Unexpected character: {}", c), line: start_line, column: start_column }),
        }
    }
}
