use crate::scanner::Token;
use crate::stmt::Stmt;
use crate::environment::Environment;
use crate::expr::LiteralValue;
use std::rc::Rc;
use std::cell::RefCell;

pub struct FunctionVal {
    paren: Token,
    params: Vec<Token>,
    body: Vec<Stmt>,
    parent_environment: Rc<RefCell<Environment>>,
}


impl FnOnce<(Vec<LiteralValue>,)> for FunctionVal {
    type Output = LiteralValue;
    extern "rust-call" fn call_once(self, _args: (Vec<LiteralValue>,)) -> Self::Output {
        todo!();
    }
}

impl FnMut<(Vec<LiteralValue>,)> for FunctionVal {
    // type Output = LiteralValue;
    extern "rust-call" fn call_mut(&mut self, _args: (Vec<LiteralValue>,)) -> Self::Output {
        todo!();
    }
}


impl Fn<(Vec<LiteralValue>,)> for FunctionVal {
    // type Output = LiteralValue;
    extern "rust-call" fn call(&self, _args: (Vec<LiteralValue>,)) -> Self::Output {
        todo!();
    }
}



