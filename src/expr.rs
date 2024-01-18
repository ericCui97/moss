use crate::{
    interpreter::Vistor,
    scanner::{LiteralValue, Token},
};
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

pub trait Accept<T> {
    fn accept(&self, visitor: &mut dyn Vistor) -> Result<T, String>;
}
impl Expr {
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        match self {
            Expr::Grouping(e) => {
                //				String::from('G') + e.to_token_sequence().as_ref()
                format!("G({})", e.to_string())
            }
            Expr::Binary(left, op, right) => {
                format!(
                    "B({} {} {})",
                    left.to_string(),
                    op.lexeme,
                    right.to_string()
                )
            }
            Expr::Literal(lit) => {
                //				lit.to_string()
                format!("L({})", lit.to_string())
            }

            Expr::Unary(op, right) => {
                //				format!("{} {}",op.lexeme,right.to_token_sequence())
                format!("U({} {})", op.lexeme, right.to_string())
            }
        }
    }
}

impl Accept for Expr {
    fn accept(&self, visitor: &mut dyn Vistor) -> Result<(), String> {
        match self {
            Expr::Grouping(e) => visitor.visit_grouping_expr(e.clone()),
            Expr::Binary(left, op, right) => {
                visitor.visit_binary_expr(left.clone(), op, right.clone())
            }
            Expr::Literal(lit) => {
                visitor.visit_literal_expr(lit);
                Ok(())
            }
            Expr::Unary(op, right) => visitor.visit_unary_expr(op, right.clone()),
        }
    }
}
