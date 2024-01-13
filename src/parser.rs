use std::{fmt, string};

use crate::scanner::{Token,LiteralValue};


#[derive(Debug,Clone)]
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
// impl fmt::Display for Expr {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write_expr_with_indent(self, f, 0)
//     }
// }

// fn write_expr_with_indent(expr: &Expr, f: &mut fmt::Formatter<'_>, indent_level: usize) -> fmt::Result {
//     match expr {
//         Expr::Grouping(e) => {
//             write!(f, "(\n")?;
//             write_expr_with_indent(e, f, indent_level + 1)?;
//             write!(f, "{})\n", "  ".repeat(indent_level))
//         }
//         Expr::Binary(left, op, right) => {
//             write!(f, "BINARY ({} {} {})\n", op.lexeme, left.to_string(), right.to_string())
//         }
//         Expr::Literal(lit) => write!(f, "LITERAL {}\n", lit.to_string()),
//         Expr::Unary(op, right) => {
//             write!(f, "UNARY ({} {})\n", op.lexeme, right.to_string())
//         }
//     }
// }

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

pub struct  Parser {
	tokens: Vec<Token>,
	current: usize,
}

impl Parser {
	pub fn new(tokens: Vec<Token>) -> Self {
		Parser {
			tokens,
			current:0,
		}
	}
	pub fn parse(&mut self) -> Result<Expr, String> {
		self.expression()
	}
	fn expression(&mut self) -> Result<Expr,String>{
		// self.equality();
//		let lhs = self.comparsion()?;
		todo!()
	}

	fn comparsion() -> Result<Expr,String>{
		todo!()
	}
	fn equality(&self) {
		todo!()
	}
}




