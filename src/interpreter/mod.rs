use crate::ast::{BinaryOperator, Node, Program, VarType};
use crate::error::{Error, Result};
use crate::runtime::{Environment, Value};
use std::collections::HashMap;

pub mod evaluator;
pub mod functions;
pub mod operations;
pub mod strings;

pub struct Interpreter {
    environment: Environment,
    external_functions: HashMap<String, Box<dyn Fn(Vec<Value>) -> Result<Value> + 'static>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new(),
            external_functions: HashMap::new(),
        }
    }

    pub fn register_function<F>(&mut self, name: &str, func: F)
    where
        F: Fn(Vec<Value>) -> Result<Value> + 'static,
    {
        self.external_functions.insert(name.to_string(), Box::new(func));
    }

    pub fn interpret(&mut self, program: Program) -> Result<Value> {
        let mut result = Value::Null;

        for statement in program.statements {
            result = self.interpret_node(statement)?;
        }

        Ok(result)
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}