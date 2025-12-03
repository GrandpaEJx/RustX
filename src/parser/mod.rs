use crate::ast::Program;
use crate::error::Result;
use crate::lexer::{Lexer, Token};

pub mod expressions;
pub mod statements;
pub mod utils;

pub struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
    previous_token: Option<Token>,
}

impl Parser {
    pub fn new(input: String) -> Result<Self> {
        let mut parser = Parser {
            lexer: Lexer::new(input),
            current_token: None,
            previous_token: None,
        };
        parser.advance_token()?;
        Ok(parser)
    }

    pub fn parse(&mut self) -> Result<Program> {
        let mut program = Program::new();

        while !self.is_at_end() {
            if let Some(stmt) = self.parse_statement()? {
                program.add_statement(stmt);
            }
        }

        Ok(program)
    }
}
