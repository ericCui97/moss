use crate::scanner::LiteralValue;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Default,Clone)]
pub struct Environment {
    map: HashMap<String, LiteralValue>,
    pub enclosing: Option<Rc<Environment>>,
}
impl Environment {
    pub fn new() -> Self {
        Self::default()
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

    pub fn assign(&mut self, name: String, value: LiteralValue) -> Result<LiteralValue, String> {
        match self.map.get_mut(&name) {
            Some(v) => {
                let old = v.clone();
                *v = value;
                Ok(old)
            }
            None => match &mut self.enclosing {
                Some(enclosing) => Rc::get_mut(&mut enclosing.clone()).expect(
                    "cant get mut from Rc<Environment> in Environment::assign"
                ).assign(name,value),
                None => Err(format!("Undefined variable '{}'.", name)),
            },
        }
    }
}
