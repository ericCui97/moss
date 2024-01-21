use crate::{expr::Expr, scanner::Token};
#[derive(Debug, Clone)]
pub enum Stmt {
    Expression { expression: Expr },
    Print { expression: Expr },
    Var { name: Token, initializer: Expr },
}
