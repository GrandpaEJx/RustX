use crate::error::{Error, Result};
use super::token::{Token, TokenType};
use super::Lexer;

impl Lexer {
    pub fn read_identifier(&mut self, line: usize, column: usize) -> Result<Token> {
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

    pub fn read_number(&mut self, line: usize, column: usize) -> Result<Token> {
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
            TokenType::Float(
                number_str
                    .parse::<f64>()
                    .map_err(|_| Error::LexerError { message: format!("Invalid float: {}", number_str), line, column })?,
            )
        } else {
            TokenType::Int(
                number_str
                    .parse::<i64>()
                    .map_err(|_| Error::LexerError { message: format!("Invalid integer: {}", number_str), line, column })?,
            )
        };

        Ok(Token {
            token_type,
            line,
            column,
        })
    }

    pub fn read_string(&mut self, line: usize, column: usize) -> Result<Token> {
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
            return Err(Error::LexerError { message: "Unterminated string".to_string(), line, column });
        }

        self.advance(); // Skip closing quote

        Ok(Token {
            token_type: TokenType::String(string_content),
            line,
            column,
        })
    }
}