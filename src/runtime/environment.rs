use super::value::Value;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Environment {
    variables: HashMap<String, Value>,
    parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            variables: HashMap::new(),
            parent: None,
        }
    }

    pub fn with_parent(parent: Box<Environment>) -> Self {
        Environment {
            variables: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(value) = self.variables.get(name) {
            Some(value.clone())
        } else if let Some(parent) = &self.parent {
            parent.get(name)
        } else {
            None
        }
    }

    pub fn assign(&mut self, name: String, value: Value) {
        if self.variables.contains_key(&name) {
            self.variables.insert(name, value);
        } else if let Some(parent) = &mut self.parent {
            parent.assign(name, value);
        } else {
            self.variables.insert(name, value);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.variables.is_empty() && self.parent.is_none()
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
