use crate::expr::{CallableImpl, LiteralValue, NativeFunctionImpl};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct Environment {
    pub values: Rc<RefCell<HashMap<String, LiteralValue>>>,
    locals: Rc<RefCell<HashMap<usize, usize>>>,
    pub enclosing: Option<Box<Environment>>,
}

fn clock_impl(_args: &Vec<LiteralValue>) -> LiteralValue {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .expect("Could not get system time")
        .as_millis();

    LiteralValue::Number(now as f64 / 1000.0)
}

fn get_globals() -> Rc<RefCell<HashMap<String, LiteralValue>>> {
    let mut env = HashMap::new();
    let fun_impl = NativeFunctionImpl {
        name: "clock".to_string(),
        arity: 0,
        fun: Rc::new(clock_impl),
    };
    let callable_impl = CallableImpl::NativeFunction(fun_impl);
    env.insert("clock".to_string(), LiteralValue::Callable(callable_impl));

    Rc::new(RefCell::new(env))
}

impl Environment {
    pub fn new(locals: HashMap<usize, usize>) -> Self {
        Self {
            values: get_globals(),
            locals: Rc::new(RefCell::new(locals)),
            enclosing: None,
        }
    }

    pub fn resolve(&self, locals: HashMap<usize, usize>) {
        // self.locals = locals --! Bad because it wont update enclosing
        for (key, val) in locals.iter() {
            self.locals.borrow_mut().insert(*key, *val);
        }
    }

    pub fn enclose(&self) -> Environment {
        Self {
            values: Rc::new(RefCell::new(HashMap::new())),
            locals: self.locals.clone(),
            enclosing: Some(Box::new(self.clone())),
        }
    }

    pub fn define(&self, name: String, value: LiteralValue) {
        self.values.borrow_mut().insert(name, value);
    }

    pub fn get(&self, name: &str, expr_id: usize) -> Option<LiteralValue> {
        let distance = self.locals.borrow().get(&expr_id).cloned();
        self.get_internal(name, distance)
    }

    pub fn get_this_instance(&self, super_id: usize) -> Option<LiteralValue> {
        let distance = self
            .locals
            .borrow()
            .get(&super_id)
            .cloned()
            .expect("Could not find 'this' even though 'super' was defined");
        self.get_internal("this", Some(distance - 1))
    }

    pub fn get_distance(&self, expr_id: usize) -> Option<usize> {
        self.locals.borrow().get(&expr_id).cloned()
    }

    fn get_internal(&self, name: &str, distance: Option<usize>) -> Option<LiteralValue> {
        if let None = distance {
            match &self.enclosing {
                None => self.values.borrow().get(name).cloned(),
                Some(env) => env.get_internal(name, distance),
            }
        } else {
            let distance = distance.unwrap();
            if distance == 0 {
                self.values.borrow().get(name).cloned()
            } else {
                match &self.enclosing {
                    None => panic!("Tried to resolve a variable that was defined deeper than the current environment depth"),
                    Some(env) => {
                        assert!(distance > 0);
                        env.get_internal(name, Some(distance - 1))
                    }
                }
            }
        }
    }

    pub fn assign_global(&self, name: &str, value: LiteralValue) -> bool {
        self.assign_internal(name, value, None)
    }

    pub fn assign(&self, name: &str, value: LiteralValue, expr_id: usize) -> bool {
        // ! Important that this ID matches with the resolver
        let distance = self.locals.borrow().get(&expr_id).cloned();
        self.assign_internal(name, value, distance)
    }

    fn assign_internal(&self, name: &str, value: LiteralValue, distance: Option<usize>) -> bool {
        if let None = distance {
            match &self.enclosing {
                Some(env) => env.assign_internal(name, value, distance),
                None => match self.values.borrow_mut().insert(name.to_string(), value) {
                    Some(_) => true,
                    None => false,
                },
            }
        } else {
            let distance = distance.unwrap();
            if distance == 0 {
                self.values.borrow_mut().insert(name.to_string(), value);
                true
            } else {
                match &self.enclosing {
                    None => panic!("Tried to define a variable in a too deep level"),
                    Some(env) => env.assign_internal(name, value, Some(distance - 1)),
                };
                true
            }
        }
    }

    #[allow(dead_code)]
    pub fn dump(&self, indent: usize) -> String {
        let mut result = String::new();
        for (key, val) in self.values.borrow().iter() {
            for _ in 0..indent {
                result.push_str(" ");
            }
            result.push_str(&format!("{}: {:?}\n", key, val));
        }
        if let Some(env) = &self.enclosing {
            result.push_str(&env.dump(indent + 2));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_init() {
        let _environment = Environment::new(HashMap::new());
    }
}
