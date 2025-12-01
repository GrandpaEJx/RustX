use crate::error::{Error, Result};
use super::token::{Token, TokenType};

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
            },
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
            },
            '=' => {
                if self.peek() == '=' {
                    self.advance();
                    Ok(self.make_token(TokenType::DoubleEquals, start_line, start_column))
                } else {
                    Ok(self.make_token(TokenType::Equals, start_line, start_column))
                }
            },
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
            },
            _ => Err(Error::LexerError(format!("Unexpected character: {}", c))),
        }
    }
    
    fn skip_whitespace(&mut self) {
        while !self.is_at_end() && self.peek().is_whitespace() && self.peek() != '\n' {
            self.advance();
        }
    }
    
    fn read_identifier(&mut self, line: usize, column: usize) -> Result<Token> {
        let start = self.position;
        
        while !self.is_at_end() && (self.is_alpha_numeric(self.peek()) || self.peek() == '_') {
            self.advance();
        }
        
        let identifier = &self.input[start..self.position];
        
        let token_type = match identifier {
            "let" => TokenType::Let,
            "fn" => TokenType::Fn,
            "return" => TokenType::Return,
            "null" => TokenType::Null,
            "String" => TokenType::TypeString,
            "Int" => TokenType::TypeInt,
            "Bool" => TokenType::TypeBool,
            "Float" => TokenType::TypeFloat,
            "print" => TokenType::Print,
            "println" => TokenType::Println,
            "printf" => TokenType::Printf,
            "true" => TokenType::Bool(true),
            "false" => TokenType::Bool(false),
            _ => TokenType::Identifier(identifier.to_string()),
        };
        
        Ok(Token {
            token_type,
            line,
            column,
        })
    }
    
    fn read_number(&mut self, line: usize, column: usize) -> Result<Token> {
        let start = self.position;
        let mut has_decimal = false;
        
        while !self.is_at_end() {
            let c = self.peek();
            
            if c.is_ascii_digit() {
                self.advance();
            } else if c == '.' && !has_decimal {
                has_decimal = true;
                self.advance();
            } else {
                break;
            }
        }
        
        let number_str = &self.input[start..self.position];
        
        let token_type = if has_decimal {
            TokenType::Float(number_str.parse::<f64>().map_err(|_| 
                Error::LexerError(format!("Invalid float: {}", number_str)))?)
        } else {
            TokenType::Int(number_str.parse::<i64>().map_err(|_| 
                Error::LexerError(format!("Invalid integer: {}", number_str)))?)
        };
        
        Ok(Token {
            token_type,
            line,
            column,
        })
    }
    
    fn read_string(&mut self, line: usize, column: usize) -> Result<Token> {
        self.advance(); // Skip opening quote
        
        let mut string_content = String::new();
        
        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\\' {
                self.advance();
                if !self.is_at_end() {
                    let escape_char = self.peek();
                    let escaped = match escape_char {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '\\' => '\\',
                        '"' => '"',
                        _ => escape_char,
                    };
                    string_content.push(escaped);
                    self.advance();
                }
            } else {
                string_content.push(self.peek());
                self.advance();
            }
        }
        
        if self.is_at_end() {
            return Err(Error::LexerError("Unterminated string".to_string()));
        }
        
        self.advance(); // Skip closing quote
        
        Ok(Token {
            token_type: TokenType::String(string_content),
            line,
            column,
        })
    }
    
    fn make_token(&self, token_type: TokenType, line: usize, column: usize) -> Token {
        Token {
            token_type,
            line,
            column,
        }
    }
    
    fn advance(&mut self) {
        if self.position < self.input.len() {
            self.position += 1;
            self.column += 1;
        }
    }
    
    fn peek(&self) -> char {
        if self.position < self.input.len() {
            self.input.chars().nth(self.position).unwrap_or('\0')
        } else {
            '\0'
        }
    }
    
    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }
    
    fn is_alpha(&self, c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }
    
    fn is_digit(&self, c: char) -> bool {
        c.is_ascii_digit()
    }
    
    fn is_alpha_numeric(&self, c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }
}
