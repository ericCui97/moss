use std::{fmt, string};
use std::ops::Deref;
use crate::parser::Expr::Literal;

use crate::scanner::{Token, LiteralValue, TokenType};
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

pub struct Parser {
	tokens: Vec<Token>,
	current: usize,
}

impl Parser {
	pub fn new(tokens: Vec<Token>) -> Self {
		Parser {
			tokens,
			current: 0,
		}
	}
	pub fn parse(&mut self) -> Result<Expr, String> {
		self.expression()
	}
	fn expression(&mut self) -> Result<Expr, String> {
		self.equality()
//		let lhs = self.comparsion()?;
	}


	fn match_token(&mut self, types: &[TokenType]) -> bool {
		if self.is_at_end() {
			return false;
		}
		for t in types {
			if self.check(t) {
				self.advance();
				return true;
			}
		}
		false
	}
	fn equality(&mut self) -> Result<Expr, String> {
	let mut lhs = self.comparison()?;
		let match_case = self.match_token(&[TokenType::BANG_EQUAL,TokenType::EQUAL_EQUAL]);
		while match_case{
			let op = self.previous();
			let rhs = self.comparison()?;
			lhs = Expr::Binary(Box::from(lhs),op.clone(),Box::from(rhs));
		}
		return Ok(lhs);

	}

	fn comparison(&mut self)->Result<Expr,String>{
		let mut expr = self.term()?;
		while self.match_token(&[TokenType::GREATER,TokenType::GREATER_EQUAL,TokenType::LESS,TokenType::LESS_EQUAL]){
			let op = self.previous();
			let rhs = self.term()?;
			expr = Expr::Binary(Box::from(expr),op.clone(),Box::from(rhs));
		}
		Ok(expr)
	}

	fn term(&mut self)->Result<Expr,String>{
		let mut expr = self.factor()?;
		while self.match_token(&[TokenType::MINUS,TokenType::PLUS]){
			let op = self.previous();
			let rhs = self.factor()?;
			expr = Expr::Binary(Box::from(expr),op.clone(),Box::from(rhs));
		}
		Ok(expr)
	}

	fn factor(&mut self)->Result<Expr,String>{
		let mut expr = self.unary();
		while self.match_token(&[TokenType::SLASH,TokenType::STAR]){
			let op = self.previous();
			let rhs = self.unary();
			expr = Expr::Binary(Box::from(expr),op.clone(),Box::from(rhs));
		}
		Ok(expr)
	}

	fn unary(&mut self)->Expr{
		if self.match_token(&[TokenType::BANG,TokenType::MINUS]){
			let op = self.previous();
			let rhs = self.unary();
			return Expr::Unary(op.clone(),Box::from(rhs));
		}
		self.primary().unwrap()
	}

	fn primary(&mut self)->Result<Expr,String>{
		if self.match_token(&[TokenType::LEFT_PAREN]){
			let expr = self.expression()?;
			self.consume(TokenType::RIGHT_PAREN,"expect ')' after expression")?;
			return Ok(Expr::Grouping(Box::from(expr)));
		}else{
			let token = self.peek();
			self.advance();
			Ok(Literal(
				LiteralValue::from_token(token.token_type).unwrap()
			))
		}
	}
	fn consume(&mut self, token_type: TokenType, message: &str) -> Result<(), String> {
		if self.check(&token_type) {
			Ok(())
		} else {
			Err(format!("{}", message))
		}
	}

	fn peek(&self) -> &Token {
		self.tokens.get(self.current).as_ref().unwrap()
	}
	fn is_at_end(&self) -> bool {
		self.peek().token_type == TokenType::EOF
	}
	fn previous(&mut self) -> &Token {
		match self.tokens.get(self.current - 1) {
			Some(t) => t,
			None => panic!("no previous token"),
		}
	}

	fn check(&self, token_type: &TokenType) -> bool {
		if self.is_at_end() {
			return false;
		}
		self.peek().token_type == *token_type
	}

	fn advance(&mut self)->&Token{
		if !self.is_at_end() {
			self.current += 1;
		}
		self.previous()
	}
}


