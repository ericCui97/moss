use crate::scanner::LiteralValue;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    pub static ref GLOBAL_MAP: Mutex<HashMap<String, LiteralValue>> = Mutex::new(HashMap::new());
}

pub fn define(name: String, value: LiteralValue) {
    GLOBAL_MAP.lock().unwrap().insert(name, value);
}

pub fn get(name: String) -> Option<LiteralValue> {
    GLOBAL_MAP.lock().unwrap().get(&name).cloned()
}
