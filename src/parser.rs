use crate::stmt::Stmt;
use crate::{
    expr::Expr,
    scanner::{LiteralValue, Token, TokenType},
};
use core::panic;
use std::cell::RefCell;

#[derive(Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    current: RefCell<usize>,
}

enum FuncType {
    Funciton,
    // Method,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: RefCell::from(0),
        }
    }
    pub fn parse(&self) -> Result<Vec<Stmt>, String> {
        let mut statements = vec![];
        let mut errs = vec![];
        while !self.is_at_end() {
            let stmt = self.declaration();
            match stmt {
                Ok(s) => statements.push(s),
                Err(e) => errs.push(e),
            }
        }
        if !errs.is_empty() {
            Err(errs.join("\n"))
        } else {
            Ok(statements)
        }
    }
    fn declaration(&self) -> Result<Stmt, String> {
        if self.match_token(&[TokenType::VAR]) {
            self.var_declaration()
        } else if self.match_token(&[TokenType::FUN]) {
            self.function(&FuncType::Funciton)
        } else {
            self.statement()
        }
    }

    fn function(&self, funcType: &FuncType) -> Result<Stmt, String> {
        let name = self.consume(
            TokenType::IDENTIFIER,
            "fun declaration should follow before identifier/n",
        )?;
        self.consume(TokenType::LEFT_PAREN, "expect '(' after function name")?;
        let mut params = vec![];
        if !self.check(&TokenType::RIGHT_PAREN) {
            loop {
                if params.len() >= 255 {
                    return Err("can't have more than 255 parameters".to_string());
                }
                params.push(self.consume(TokenType::IDENTIFIER, "expect parameter name")?);
                if !self.match_token(&[TokenType::COMMA]) {
                    break;
                }
            }
        }
        self.consume(TokenType::RIGHT_PAREN, "expect ')' after parameters")?;
        self.consume(TokenType::LEFT_BRACE, "expect '{' before function body")?;
        let body = match self.block_statement().unwrap() {
            Stmt::Block { statements } => statements,
            _ => panic!("expect block statement"),
        };
        Ok(Stmt::Function { name, params, body })
    }

    pub fn synchronize(&self) {
        todo!()
    }

    fn var_declaration(&self) -> Result<Stmt, String> {
        // var name = expression;
        let name = self.consume(TokenType::IDENTIFIER, "expect variable name")?;
        let initializer = if self.match_token(&[TokenType::EQUAL]) {
            self.expression()?
        } else {
            // 如果没有初始化，那么就是nil
            Expr::Literal(LiteralValue::NIL)
        };
        self.consume(
            TokenType::SEMICOLON,
            "expect ';' after variable declaration",
        )?;
        Ok(Stmt::Var { name, initializer })
    }

    fn statement(&self) -> Result<Stmt, String> {
        if self.match_token(&[TokenType::PRINT]) {
            self.print_statement()
        } else if self.match_token(&[TokenType::LEFT_BRACE]) {
            self.block_statement()
        } else if self.match_token(&[TokenType::IF]) {
            self.if_statement()
        } else if self.match_token(&[TokenType::WHILE]) {
            self.while_statement()
        } else if self.match_token(&[TokenType::FOR]) {
            self.for_statement()
        } else if self.match_token(&[TokenType::RETURN]) {
            self.return_statement()
        } else {
            self.expression_statement()
        }
    }

    fn return_statement(&self) -> Result<Stmt, String> {
        let keyword = self.previous();
        let value = if !self.check(&TokenType::SEMICOLON) {
            self.expression()?
        } else {
            Expr::Literal(LiteralValue::NIL)
        };
        self.consume(TokenType::SEMICOLON, "expect ';' after return value")?;
        Ok(Stmt::Return {
            keyword: keyword.clone(),
            value: Some(value),
        })
    }

    fn for_statement(&self) -> Result<Stmt, String> {
        self.consume(TokenType::LEFT_PAREN, "expect '(' after for");
        let mut initializer;
        if self.match_token(&[TokenType::SEMICOLON]) {
            initializer = None;
        } else if self.match_token(&[TokenType::VAR]) {
            initializer = Some(self.var_declaration()?);
        } else {
            initializer = Some(self.expression_statement()?);
        }

        let mut condition;
        if !self.check(&TokenType::SEMICOLON) {
            // expression 不会consume 分号
            condition = Some(self.expression()?);
        } else {
            condition = None;
        }

        self.consume(TokenType::SEMICOLON, "expect ';' after loop condition");

        let mut increment;
        if !self.check(&TokenType::RIGHT_PAREN) {
            increment = Some(self.expression()?);
        } else {
            increment = None;
        }

        self.consume(TokenType::RIGHT_PAREN, "expect ')' after for clauses")?;

        let mut body = self.statement()?;
        if let Some(incr) = increment {
            body = Stmt::Block {
                statements: vec![
                    Box::new(body),
                    Box::new(Stmt::Expression { expression: incr }),
                ],
            };
        }

        match condition {
            Some(c) => {
                body = Stmt::WhileStmt {
                    condition: c,
                    body: Box::from(body),
                };
            }
            None => {
                body = Stmt::WhileStmt {
                    condition: Expr::Literal(LiteralValue::BOOLEAN(true)),
                    body: Box::from(body),
                };
            }
        }

        if let Some(init) = initializer {
            body = Stmt::Block {
                statements: vec![Box::new(init), Box::new(body)],
            };
        }

        Ok(body)
    }

    fn if_statement(&self) -> Result<Stmt, String> {
        self.consume(TokenType::LEFT_PAREN, "expect '(' after if")?;
        let condition = self.expression()?;
        self.consume(TokenType::RIGHT_PAREN, "expect ')' after if condition")?;
        let then_branch = Box::from(self.statement()?);
        let else_branch = if self.match_token(&[TokenType::ELSE]) {
            Some(Box::from(self.statement()?))
        } else {
            None
        };
        Ok(Stmt::IfStmt {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn block_statement(&self) -> Result<Stmt, String> {
        let mut statements = vec![];
        while !self.check(&TokenType::RIGHT_BRACE) && !self.is_at_end() {
            let stmt = self.declaration();
            match stmt {
                Ok(s) => statements.push(Box::new(s)),
                Err(e) => return Err(e),
            }
        }
        self.consume(TokenType::RIGHT_BRACE, "expect '}' after block")?;
        Ok(Stmt::Block { statements })
    }

    fn while_statement(&self) -> Result<Stmt, String> {
        self.consume(TokenType::LEFT_PAREN, "expect '(' after while")?;
        let condition = self.expression()?;
        self.consume(TokenType::RIGHT_PAREN, "expect ')' after while condition")?;
        let body = Box::from(self.statement()?);
        Ok(Stmt::WhileStmt { condition, body })
    }

    fn expression_statement(&self) -> Result<Stmt, String> {
        let expr = self.expression()?;
        self.consume(
            TokenType::SEMICOLON,
            "expect ';' after expression statement",
        )?;
        Ok(Stmt::Expression { expression: expr })
    }

    fn print_statement(&self) -> Result<Stmt, String> {
        let value = self.expression()?;
        self.consume(TokenType::SEMICOLON, "expect ';' after value")?;
        Ok(Stmt::Print { expression: value })
    }

    fn expression(&self) -> Result<Expr, String> {
        self.assignment()
    }

    fn assignment(&self) -> Result<Expr, String> {
        let expr = self.or()?;
        if self.match_token(&[TokenType::EQUAL]) {
            let _equals = self.previous();
            let value = self.assignment()?;
            match expr {
                Expr::Variable(name) => {
                    return Ok(Expr::Assign {
                        name,
                        value: Box::from(value),
                    })
                }
                _ => {
                    return Err(format!("invalid assignment target {:?}", expr));
                }
            }
        }
        Ok(expr)
    }

    // logic operator or->and->equality(including unary)
    fn or(&self) -> Result<Expr, String> {
        let mut expr = self.and()?;
        while self.match_token(&[TokenType::OR]) {
            let op = self.previous();
            let rhs = self.and()?;
            expr = Expr::Logical(Box::new(expr), op.clone(), Box::new(rhs))
        }
        Ok(expr)
    }

    fn and(&self) -> Result<Expr, String> {
        let mut expr = self.equality()?;
        while self.match_token(&[TokenType::AND]) {
            let op = self.previous();
            let rhs = self.equality()?;
            expr = Expr::Logical(Box::new(expr), op.clone(), Box::new(rhs))
        }
        Ok(expr)
    }
    #[cfg(test)]
    pub fn parse_expression(&self) -> Result<Expr, String> {
        self.expression()
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
    // 处理流程
    // equality(== !=)
    // -> comparison(< <= > >=)
    // -> term(+ -)
    // -> factor(* /)
    // -> unary(- !)
    // -> primary( () primitive)
    fn equality(&self) -> Result<Expr, String> {
        let mut lhs = self.comparison()?;
        while self.match_token(&[TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let op = self.previous();
            let rhs = self.comparison()?;
            lhs = Expr::Binary(Box::from(lhs), op.clone(), Box::from(rhs));
        }
        Ok(lhs)
    }
    // handle > >= < <=

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
    // handle + -
    fn term(&self) -> Result<Expr, String> {
        let mut expr = self.factor()?;
        while self.match_token(&[TokenType::MINUS, TokenType::PLUS]) {
            let op = self.previous();
            let rhs = self.factor()?;
            expr = Expr::Binary(Box::from(expr), op.clone(), Box::from(rhs));
        }
        Ok(expr)
    }
    // handle * /
    fn factor(&self) -> Result<Expr, String> {
        let mut expr = self.unary()?;
        while self.match_token(&[TokenType::SLASH, TokenType::STAR]) {
            let op = self.previous();
            let rhs = self.unary()?;
            expr = Expr::Binary(Box::from(expr), op.clone(), Box::from(rhs));
        }
        Ok(expr)
    }
    // handle ! - 单目运算符
    fn unary(&self) -> Result<Expr, String> {
        if self.match_token(&[TokenType::BANG, TokenType::MINUS]) {
            let op = self.previous();
            let rhs = self.unary()?;
            return Ok(Expr::Unary(op.clone(), Box::from(rhs)));
        }
        // self.primary().unwrap()

        self.call()
    }

    fn call(&self) -> Result<Expr, String> {
        let mut expr = self.primary().unwrap();
        loop {
            if self.match_token(&[TokenType::LEFT_PAREN]) {
                expr = self.finish_call(expr).unwrap();
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn finish_call(&self, callee: Expr) -> Result<Expr, String> {
        let mut arguments = vec![];
        if !self.check(&TokenType::RIGHT_PAREN) {
            loop {
                if arguments.len() >= 255 {
                    return Err("can't have more than 255 arguments".to_string());
                }
                arguments.push(self.expression()?);
                if !self.match_token(&[TokenType::COMMA]) {
                    break;
                }
            }
        }
        let paren = self.consume(TokenType::RIGHT_PAREN, "expect ')' after arguments")?;
        Ok(Expr::Call {
            callee: Box::from(callee),
            paren,
            arguments,
        })
    }

    fn primary(&self) -> Result<Expr, String> {
        let token = self.peek();
        match token.token_type {
            TokenType::LEFT_PAREN => {
                self.advance();
                let expr = self.expression()?;
                self.consume(TokenType::RIGHT_PAREN, "expect ')' after expression")?;
                Ok(Expr::Grouping(Box::from(expr)))
            }
            TokenType::NUMBER
            | TokenType::STRING
            | TokenType::TRUE
            | TokenType::FALSE
            | TokenType::NIL => {
                self.advance();
                Ok(Expr::Literal(LiteralValue::from_token(token).unwrap()))
            }
            // 使用变量的时候
            TokenType::IDENTIFIER => {
                self.advance();
                Ok(Expr::Variable(token.clone()))
            }
            _ => Err(format!("expect expression, got {:?}", token.token_type)),
        }
    }
    fn consume(&self, token_type: TokenType, message: &str) -> Result<Token, String> {
        if self.check(&token_type) {
            self.advance();
            let token = self.previous();
            Ok(token.clone())
        } else {
            Err(message.to_string())
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
    use super::*;
    use crate::scanner::Scanner;

    fn unwrap_stmts_as_single_expr(stmts: Vec<Stmt>) -> Expr {
        let mut expr = Expr::Literal(LiteralValue::NIL);
        for stmt in stmts {
            match stmt {
                Stmt::Expression { expression } => expr = expression,
                _ => todo!(),
            }
        }
        expr
    }
    #[test]
    fn test_parser() {
        let source = "1+2*3;";
        let scan = &mut Scanner::new(source);
        let tokens = scan.scan_tokens().unwrap();
        let parser = Parser::new(tokens);
        let expr = unwrap_stmts_as_single_expr(parser.parse().unwrap());
        //		println!("{:?}", expr.to_string());
        assert_eq!(expr.to_string(), "B(L(1) + B(L(2) * L(3)))");
    }

    #[test]
    fn test_comparison() {
        let source = "1+2*3>=4;";
        let scan = &mut Scanner::new(source);
        let tokens = scan.scan_tokens().unwrap();
        let parser = Parser::new(tokens);
        let expr = unwrap_stmts_as_single_expr(parser.parse().unwrap());
        println!("{:?}", expr.to_string());
    }

    #[test]
    fn test_addition() {
        let source = "1+2;";
        let scan = &mut Scanner::new(source);
        let tokens = scan.scan_tokens().unwrap();
        let parser = Parser::new(tokens);
        let expr = unwrap_stmts_as_single_expr(parser.parse().unwrap());
        //		println!("{:?}", expr);
        assert_eq!(expr.to_string(), "B(L(1) + L(2))");
    }

    #[test]
    fn test_simple_math_expr_parse_with_paren() {
        let source = "(1+21)*2432;";
        let scan = &mut Scanner::new(source);
        let tokens = scan.scan_tokens().unwrap();
        let parser = Parser::new(tokens);
        let expr = unwrap_stmts_as_single_expr(parser.parse().unwrap());
        //		println!("{:?}", expr.to_string());
        assert_eq!(expr.to_string(), "B(G(B(L(1) + L(21))) * L(2432))");
    }

    #[test]
    fn test_complex_expression() {
        let source = "(1.2+2)*3.4>=4;";
        let scan = &mut Scanner::new(source);
        let tokens = scan.scan_tokens().unwrap();
        let parser = Parser::new(tokens);
        let expr = unwrap_stmts_as_single_expr(parser.parse().unwrap());
        //		println!("{:?}", expr.to_string());
        assert_eq!(
            expr.to_string(),
            "B(B(G(B(L(1.2) + L(2))) * L(3.4)) >= L(4))"
        );
    }

    #[test]
    fn test_simple_math_expr_parse2() {
        let source = "1+2*3;";
        let scan = &mut Scanner::new(source);
        let tokens = scan.scan_tokens().unwrap();
        let parser = Parser::new(tokens);
        let expr = unwrap_stmts_as_single_expr(parser.parse().unwrap());
        //		println!("{:?}", expr);
        assert_eq!(expr.to_string(), "B(L(1) + B(L(2) * L(3)))");
    }

    #[test]
    fn test_unary() {
        let source = "-1;";
        let scan = &mut Scanner::new(source);
        let tokens = scan.scan_tokens().unwrap();
        let parser = Parser::new(tokens);
        let expr = unwrap_stmts_as_single_expr(parser.parse().unwrap());
        assert_eq!(expr.to_string(), "U(- L(1))");
    }
}
