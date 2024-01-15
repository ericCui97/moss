use crate::scanner::{LiteralValue, Token};
#[derive(Debug, Clone)]
pub enum Expr {
    // grouping expression like (1+2)
    Grouping(Box<Expr>),
    // binary expression like 1+2
    Binary(Box<Expr>, Token, Box<Expr>),
    // literal expression like 1,2,3 or "hello world" true false nil
    Literal(LiteralValue),
    // unary expression like !true
    Unary(Token, Box<Expr>),
}

impl Expr {
    pub fn to_string(&self) -> String {
        match self {
            Expr::Grouping(e) => format!("(GROUP {})", e.to_string()),
            Expr::Binary(left, op, right) => format!("BINARY ({} {} {})", op.lexeme, left.to_string(), right.to_string()),
            Expr::Literal(lit) => format!("LITERAL {}", lit.to_string()),
            Expr::Unary(op, right) => format!("UNARY ({} {})", op.lexeme, right.to_string()),
        }
    }
}
