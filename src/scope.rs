// scope.rs

use std::collections::HashMap;
use crate::ast::Value;

#[derive(Debug, Clone)]
pub struct Scope {
    parent: Option<Box<Scope>>,
    variables: HashMap<String, Value>,
}

impl Scope {
    pub fn new(parent: Option<Scope>) -> Self {
        Self {
            parent: parent.map(Box::new),
            variables: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        self.variables.get(name).cloned().or_else(|| {
            self.parent.as_ref().and_then(|parent_scope| parent_scope.get(name))
        })
    }

    pub fn set(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }
}
