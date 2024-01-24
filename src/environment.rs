use crate::scanner::LiteralValue;
use std::collections::HashMap;

#[derive(Default)]
pub struct Environment {
    map: HashMap<String, LiteralValue>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_enclosing(enclosing: Environment) -> Self {
        Self {
            map: HashMap::new(),
            enclosing: Some(Box::new(enclosing)),
        }
    }

    pub fn define(&mut self, name: String, value: LiteralValue) {
        self.map.insert(name, value);
    }

    pub fn get(&self, name: String) -> Option<&LiteralValue> {
        match self.map.get(&name) {
            Some(value) => Some(value),
            None => match &self.enclosing {
                Some(enclosing) => enclosing.get(name),
                None => None,
            },
        }
    }

    pub fn assign(&mut self, name: String, value: LiteralValue) -> Result<(), String> {
        match self.map.get_mut(&name) {
            Some(v) => {
                *v = value;
                Ok(())
            }
            None => match &mut self.enclosing {
                Some(enclosing) => enclosing.assign(name, value),
                None => Err(format!("Undefined variable '{}'.", name)),
            },
        }
    }
}
