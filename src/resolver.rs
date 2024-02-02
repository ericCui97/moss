use crate::interpreter::Interpreter;
use crate::stmt::Stmt;

pub struct Resolver {
    interpreter:Interpreter,
}

impl Resolver{
    pub fn resolve(&mut self,stmt:Stmt){
        todo!()
    }

    fn resolve_block(&mut self,stmt:Stmt){
        todo!()
    }

    fn resolve_begin(){
        todo!()
    }

    fn resolve_end(){
        todo!()
    }
}