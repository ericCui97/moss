use moss::parser::Expr;
use moss::scanner::{LiteralValue, Token, TokenType};

#[cfg(test)]
mod tests{
	use super::*;
	#[test]
	fn test_expr(){
		let expr = Expr::Binary(
			Box::new(Expr::Unary(
				Token::new(TokenType::MINUS, "-".to_string(), None, 1),
				Box::new(Expr::Literal(LiteralValue::NUMBER(123.0))),
			)),
			Token::new(TokenType::STAR, "*".to_string(), None, 1),
			Box::new(Expr::Grouping(Box::new(Expr::Literal(LiteralValue::NUMBER(45.67))))),
		);
		// println!("{}", expr);
		assert_eq!(expr.to_string(), "BINARY (* UNARY (- LITERAL 123) (GROUP LITERAL 45.67))");
	}
}