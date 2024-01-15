use crate::expr::Expr;
use crate::scanner::{LiteralValue, Token, TokenType};
use std::cell::{Cell, RefCell};

#[derive(Debug, Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    current: RefCell<usize>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: RefCell::from(0),
        }
    }
    pub fn parse(&mut self) -> Result<Expr, String> {
        self.expression()
    }
    fn expression(&self) -> Result<Expr, String> {
        self.equality()
    }

    fn match_token(&self, types: &[TokenType]) -> bool {
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
    fn equality(&self) -> Result<Expr, String> {
        let mut lhs = self.comparison()?;
        let match_case = self.match_token(&[TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]);
        while match_case {
            let op = self.previous();
            let rhs = self.comparison()?;
            lhs = Expr::Binary(Box::from(lhs), op.clone(), Box::from(rhs));
        }
        return Ok(lhs);
    }

    fn comparison(&self) -> Result<Expr, String> {
        let mut expr = self.term()?;
        while self.match_token(&[
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL,
        ]) {
            let op = self.previous();
            let rhs = self.term()?;
            expr = Expr::Binary(Box::from(expr), op.clone(), Box::from(rhs));
        }
        Ok(expr)
    }

    fn term(&self) -> Result<Expr, String> {
        let mut expr = self.factor()?;
        while self.match_token(&[TokenType::MINUS, TokenType::PLUS]) {
            let op = self.previous();
            let rhs = self.factor()?;
            expr = Expr::Binary(Box::from(expr), op.clone(), Box::from(rhs));
        }
        Ok(expr)
    }

    fn factor(&self) -> Result<Expr, String> {
        let mut expr = self.unary();
        while self.match_token(&[TokenType::SLASH, TokenType::STAR]) {
            let op = self.previous();
            let rhs = self.unary();
            expr = Expr::Binary(Box::from(expr), op.clone(), Box::from(rhs));
        }
        Ok(expr)
    }

    fn unary(&self) -> Expr {
        if self.match_token(&[TokenType::BANG, TokenType::MINUS]) {
            let op = self.previous();
            let rhs = self.unary();
            return Expr::Unary(op.clone(), Box::from(rhs));
        }
        self.primary().unwrap()
    }

    fn primary(&self) -> Result<Expr, String> {
        if self.match_token(&[TokenType::LEFT_PAREN]) {
            let expr = self.expression()?;
            self.consume(TokenType::RIGHT_PAREN, "expect ')' after expression")?;
            return Ok(Expr::Grouping(Box::from(expr)));
        } else {
            let token = self.peek();
            self.advance();
            Ok(Expr::Literal(
                LiteralValue::from_token(token.token_type).unwrap(),
            ))
        }
    }
    fn consume(&self, token_type: TokenType, message: &str) -> Result<(), String> {
        if self.check(&token_type) {
            Ok(())
        } else {
            Err(format!("{}", message))
        }
    }

    fn peek(&self) -> &Token {
        self.tokens
            .get(self.current.clone().into_inner())
            .as_ref()
            .unwrap()
    }
    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }
    fn previous(&self) -> &Token {
        match self.tokens.get(self.current.clone().into_inner() - 1) {
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

    // current 前移 返回previous
    fn advance(&self) -> &Token {
        if !self.is_at_end() {
            *self.current.borrow_mut() += 1;
        }
        self.previous()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_parser() {
        use crate::parser::Parser;
        use crate::scanner::Scanner;
        let source = "1+2*3";
        let scan = &mut Scanner::new(source);
        let tokens = scan.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        println!("{:?}", expr);
    }
    #[test]
    fn test_parse() {
        use crate::parser::Parser;
        use crate::scanner::Scanner;
        let source = "(1+2)*3";
        let scan = &mut Scanner::new(source);
        let tokens = scan.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        println!("{:?}", expr);
    }
}
