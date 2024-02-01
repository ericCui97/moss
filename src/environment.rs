use crate::scanner::LiteralValue;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Default, Clone)]
pub struct Environment {
    map: HashMap<String, LiteralValue>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}
impl Environment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn define(&mut self, name: String, value: LiteralValue) {
        self.map.insert(name, value);
    }

    pub fn get(&self, name: String) -> Option<LiteralValue> {
        let value = self.map.get(&name);
        match (value, &self.enclosing) {
            (Some(val), _) => Some(val.clone()),
            (None, Some(enclosing)) => enclosing.borrow().get(name),
            (None, None) => None,
        }
    }
    pub fn assign(&mut self, name: String, value: LiteralValue) -> bool {
        let old_value = self.map.get_mut(&name);
        match (old_value, &self.enclosing) {
            (Some(_), _) => {
                self.map.insert(name, value.clone());
                true
            }
            (None, Some(enclosing)) => (enclosing.borrow_mut()).assign(name, value),
            (None, None) => false,
        }
    }
}
