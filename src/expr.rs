use lazy_static::lazy_static;
use crate::scanner::{LiteralValue, Token};
use std::collections::HashMap;
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

//lazy_static! {
//    static ref EXPR_MAP:HashMap<Expr, &'static str> = {
//        [
//	        (Expr::Grouping,"G")
//        ]
//        .iter()
//		.cloned()
//        .collect()
//    };
//}

impl Expr {
//	pub fn to_string(&self) -> String {
//		match self {
//			Expr::Grouping(e) => format!("(GROUP {})", e.to_string()),
//			Expr::Binary(left, op, right) => format!("BINARY ({} {} {})", op.lexeme, left.to_string(), right.to_string()),
//			Expr::Literal(lit) => format!("LITERAL {}", lit.to_string()),
//			Expr::Unary(op, right) => format!("UNARY ({} {})", op.lexeme, right.to_string()),
//		}
//	}
	pub fn to_string(&self) ->String {
		match self {
			Expr::Grouping(e)=>{
//				String::from('G') + e.to_token_sequence().as_ref()
				format!("G({})",e.to_string())
			},
			Expr::Binary(left, op, right) => {
				format!("B({} {} {})",left.to_string(),op.lexeme,right.to_string())
			},
			Expr::Literal(lit) => {
//				lit.to_string()
				format!("L({})",lit.to_string())
			},

			Expr::Unary(op, right) => {
//				format!("{} {}",op.lexeme,right.to_token_sequence())
				format!("U({} {})",op.lexeme,right.to_string())
			},
		}
	}
}
