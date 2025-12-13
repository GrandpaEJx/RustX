/// Environment for variable storage in the interpreter

use crate::value::Value;
use std::collections::HashMap;

/// Environment for variable storage
#[derive(Debug, Clone)]
pub struct Environment {
    scopes: Vec<HashMap<String, Value>>,
}

impl Environment {
    /// Creates a new environment
    pub fn new() -> Self {
        Environment {
            scopes: vec![HashMap::new()],
        }
    }

    /// Pushes a new scope
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// Pops the current scope
    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    /// Sets a variable in the current scope
    pub fn set(&mut self, name: String, value: Value) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, value);
        }
    }

    /// Updates an existing variable or creates it in the current scope
    pub fn update(&mut self, name: String, value: Value) {
        // Search for the variable in all scopes from innermost to outermost
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(&name) {
                scope.insert(name, value);
                return;
            }
        }
        // If not found, create it in the current scope
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, value);
        }
    }

    /// Gets a variable value
    pub fn get(&self, name: &str) -> Result<Value, String> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Ok(value.clone());
            }
        }
        Err(format!("Undefined variable: {}", name))
    }
    /// Gets all variables in the global scope (for module exports)
    pub fn get_exports(&self) -> HashMap<String, Value> {
        // We assume the bottom-most scope is the global/module scope
        if let Some(scope) = self.scopes.first() {
            scope.clone()
        } else {
            HashMap::new()
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
