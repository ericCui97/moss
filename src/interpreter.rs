use crate::{expr::{Expr, Accept}, scanner::{Token, LiteralValue}};
struct Interpreter {}
enum Primitive {
    Number(f64),
    String(String),
    Boolean(bool),
}
pub trait Vistor {
    fn visit_binary_expr(&mut self, lhs: Box<Expr>, op: &Token, rhs: Box<Expr>) -> Result<(), String>;
    fn visit_grouping_expr(&mut self, expr: Box<Expr>) -> Result<(), String>;
    fn visit_literal_expr(&mut self, expr: &LiteralValue) -> Result<Option<Primitive>,String>;
    fn visit_unary_expr(&mut self,op:&Token, expr: Box<Expr>) -> Result<(), String>;
}
impl Vistor for Interpreter {
    fn visit_binary_expr(&mut self, lhs: Box<Expr>, op: &Token, rhs: Box<Expr>) -> Result<(), String> {
        todo!()
    }
    fn visit_grouping_expr(&mut self, expr: Box<Expr>) -> Result<(), String> {
        todo!()
    }
    fn visit_literal_expr(&mut self, expr: &LiteralValue) -> Option<Primitive> {
        expr.accept(self)
    }
    fn visit_unary_expr(&mut self, op:&Token,expr: Box<Expr>) -> Result<(), String> {
        expr.accept(self)

    }
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }
    pub fn interpret(&mut self, expr: Expr) -> Result<(), String> {
        
    }
}
