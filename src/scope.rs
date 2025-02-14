use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Scope {
    variables: HashMap<String, Value>,
    parent: Option<Box<Scope>>,
}

impl Scope {
    pub fn new(parent: Option<Scope>) -> Self {
        Self {
            variables: HashMap::new(),
            parent: parent.map(Box::new),
        }
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        self.variables
            .get(name)
            .cloned()
            .or_else(|| self.parent.as_ref()?.get(name))
    }

    pub fn set(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }
}
