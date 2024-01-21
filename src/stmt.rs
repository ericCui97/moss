use crate::{
    expr::Expr,
    scanner::{LiteralValue, Token},
};
#[derive(Debug, Clone)]
pub enum Stmt {
    Expression { expression: Expr },
    Print { expression: Expr },
    Var { name: Token, initializer: Expr },
}
// for test
impl Stmt {
    pub fn evaluate(&self) -> Result<LiteralValue, String> {
        match self {
            Stmt::Expression { expression } => expression.evaluate(),

            _ => Err(String::from("not implemented")),
        }
    }
}
