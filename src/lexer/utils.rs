use super::token::Token;
use super::token::TokenType;
use super::Lexer;

impl Lexer {
    pub fn skip_whitespace(&mut self) {
        while !self.is_at_end() && self.peek().is_whitespace() && self.peek() != '\n' {
            self.advance();
        }
    }

    pub fn advance(&mut self) {
        if self.position < self.input.len() {
            self.position += 1;
            self.column += 1;
        }
    }

    pub fn peek(&self) -> char {
        if self.position < self.input.len() {
            self.input.chars().nth(self.position).unwrap_or('\0')
        } else {
            '\0'
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }

    pub fn make_token(&self, token_type: TokenType, line: usize, column: usize) -> Token {
        Token {
            token_type,
            line,
            column,
        }
    }

    pub fn is_alpha(&self, c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    pub fn is_digit(&self, c: char) -> bool {
        c.is_ascii_digit()
    }

    pub fn is_alpha_numeric(&self, c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }
}