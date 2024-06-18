use crate::expr::{Expr, Expr::*, LiteralValue};
use crate::scanner::{Token, TokenType, TokenType::*};
use crate::stmt::Stmt;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    next_id: usize,
}

#[derive(Debug)]
enum FunctionKind {
    Function,
    Method,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            next_id: 0,
        }
    }

    fn get_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        id
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts = vec![];
        let mut errs = vec![];

        while !self.is_at_end() {
            let stmt = self.declaration();
            match stmt {
                Ok(s) => stmts.push(s),
                Err(msg) => {
                    errs.push(msg);
                    self.synchronize();
                }
            }
        }

        if errs.len() == 0 {
            Ok(stmts)
        } else {
            Err(errs.join("\n"))
        }
    }

    fn declaration(&mut self) -> Result<Stmt, String> {
        if self.match_token(Var) {
            self.var_declaration()
        } else if self.match_token(Fun) {
            self.function(FunctionKind::Function)
        } else if self.match_token(Class) {
            self.class_declaration()
        } else {
            self.statement()
        }
    }

    fn class_declaration(&mut self) -> Result<Stmt, String> {
        let name = self.consume(Identifier, "Expected name after 'class' keyword.")?;
        let superclass = if self.match_token(TokenType::Less) {
            self.consume(Identifier, "Expected superclass name after '<'.")?;
            Some(Expr::Variable {
                id: self.get_id(),
                name: self.previous(),
            })
        } else {
            None
        };

        self.consume(LeftBrace, "Expected '{' before class body.")?;

        let mut methods = vec![];
        while !self.check(RightBrace) && !self.is_at_end() {
            let method = self.function(FunctionKind::Method)?;
            methods.push(Box::new(method));
        }

        self.consume(RightBrace, "Expected '}' after class body.")?;

        Ok(Stmt::Class {
            name,
            methods,
            superclass,
        })
    }

    fn function(&mut self, kind: FunctionKind) -> Result<Stmt, String> {
        let name = self.consume(Identifier, &format!("Expected {kind:?} name"))?;

        if self.match_token(Gets) {
            let cmd_body = self.consume(StringLit, "Expected command body")?; 
            self.consume(Semicolon, "Expected ';' after command body")?;

            return Ok(Stmt::CmdFunction {
                name,
                cmd: cmd_body.lexeme,
            });
        }

        self.consume(LeftParen, &format!("Expected '(' after {kind:?} name"))?;

        let mut parameters = vec![];
        if !self.check(RightParen) {
            loop {
                if parameters.len() >= 255 {
                    let location = self.peek().line_number;
                    return Err(format!(
                        "Line {location}: Cant have more than 255 arguments"
                    ));
                }

                let param = self.consume(Identifier, "Expected parameter name")?;
                parameters.push(param);

                if !self.match_token(Comma) {
                    break;
                }
            }
        }
        self.consume(RightParen, "Expected ')' after parameters.")?;

        self.consume(LeftBrace, &format!("Expected '{{' before {kind:?} body."))?;
        let body = match self.block_statement()? {
            Stmt::Block { statements } => statements,
            _ => panic!("Block statement parsed something that was not a block"),
        };

        Ok(Stmt::Function {
            name,
            params: parameters,
            body,
        })
    }

    fn var_declaration(&mut self) -> Result<Stmt, String> {
        let token = self.consume(Identifier, "Expected variable name")?;

        let initializer;
        if self.match_token(Equal) {
            initializer = self.expression()?;
        } else {
            initializer = Literal {
                id: self.get_id(),
                value: LiteralValue::Nil,
            };
        }

        self.consume(Semicolon, "Expected ';' after variable declaration")?;

        Ok(Stmt::Var {
            name: token,
            initializer,
        })
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        if self.match_token(Print) {
            self.print_statement()
        } else if self.match_token(LeftBrace) {
            self.block_statement()
        } else if self.match_token(If) {
            self.if_statement()
        } else if self.match_token(While) {
            self.while_statement()
        } else if self.match_token(For) {
            self.for_statement()
        } else if self.match_token(Return) {
            self.return_statement()
        } else {
            self.expression_statement()
        }
    }

    fn return_statement(&mut self) -> Result<Stmt, String> {
        let keyword = self.previous();
        let value;
        if !self.check(Semicolon) {
            // NOT return;
            value = Some(self.expression()?);
        } else {
            value = None;
        }
        self.consume(Semicolon, "Expected ';' after return value;")?;

        Ok(Stmt::ReturnStmt { keyword, value })
    }

    fn for_statement(&mut self) -> Result<Stmt, String> {
        // for v
        //       ( SMTH ; SMTH ; SMTH )
        self.consume(LeftParen, "Expected '(' after 'for'.")?;

        // Consumes "SMTH ;"
        let initializer;
        if self.match_token(Semicolon) {
            initializer = None;
        } else if self.match_token(Var) {
            let var_decl = self.var_declaration()?;
            initializer = Some(var_decl);
        } else {
            let expr = self.expression_statement()?;
            initializer = Some(expr);
        }

        // Consumes "SMTH? ;"
        let condition;
        if !self.check(Semicolon) {
            let expr = self.expression()?;
            condition = Some(expr);
        } else {
            condition = None;
        }
        self.consume(Semicolon, "Expected ';' after loop condition.")?;

        let increment;
        if !self.check(RightParen) {
            let expr = self.expression()?;
            increment = Some(expr);
        } else {
            increment = None;
        }
        self.consume(RightParen, "Expected ')' after for clauses.")?;

        let mut body = self.statement()?;

        if let Some(incr) = increment {
            body = Stmt::Block {
                statements: vec![
                    Box::new(body),
                    Box::new(Stmt::Expression { expression: incr }),
                ],
            };
        }

        let cond;
        match condition {
            None => {
                cond = Expr::Literal {
                    id: self.get_id(),
                    value: LiteralValue::True,
                }
            }
            Some(c) => cond = c,
        }
        body = Stmt::WhileStmt {
            condition: cond,
            body: Box::new(body),
        };

        if let Some(init) = initializer {
            body = Stmt::Block {
                statements: vec![Box::new(init), Box::new(body)],
            };
        }

        Ok(body)
    }

    fn while_statement(&mut self) -> Result<Stmt, String> {
        self.consume(LeftParen, "Expected '(' after 'while'")?;
        let condition = self.expression()?;
        self.consume(RightParen, "Expected ')' after condition.")?;
        let body = self.statement()?;

        Ok(Stmt::WhileStmt {
            condition,
            body: Box::new(body),
        })
    }

    fn if_statement(&mut self) -> Result<Stmt, String> {
        self.consume(LeftParen, "Expected '(' after 'if'")?;
        let predicate = self.expression()?;
        self.consume(RightParen, "Expected ')' after if-predicate")?;

        let then = Box::new(self.statement()?);
        let els = if self.match_token(Else) {
            let stm = self.statement()?;
            Some(Box::new(stm))
        } else {
            None
        };

        Ok(Stmt::IfStmt {
            predicate,
            then,
            els,
        })
    }

    fn block_statement(&mut self) -> Result<Stmt, String> {
        let mut statements = vec![];

        while !self.check(RightBrace) && !self.is_at_end() {
            let decl = self.declaration()?;
            statements.push(Box::new(decl));
        }

        self.consume(RightBrace, "Expected '}' after a block")?;
        Ok(Stmt::Block { statements })
    }

    fn print_statement(&mut self) -> Result<Stmt, String> {
        let value = self.expression()?;
        self.consume(Semicolon, "Expected ';' after value.")?;
        Ok(Stmt::Print { expression: value })
    }

    fn expression_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;
        self.consume(Semicolon, "Expected ';' after expression.")?;
        Ok(Stmt::Expression { expression: expr })
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.assignment()
    }

    fn function_expression(&mut self) -> Result<Expr, String> {
        let paren = self.consume(LeftParen, "Expected '(' after anonymous function")?;
        let mut parameters = vec![];
        if !self.check(RightParen) {
            loop {
                if parameters.len() >= 255 {
                    let location = self.peek().line_number;
                    return Err(format!(
                        "Line {location}: Cant have more than 255 arguments"
                    ));
                }

                let param = self.consume(Identifier, "Expected parameter name")?;
                parameters.push(param);

                if !self.match_token(Comma) {
                    break;
                }
            }
        }
        self.consume(
            RightParen,
            "Expected ')' after anonymous function parameters",
        )?;

        self.consume(
            LeftBrace,
            "Expected '{' after anonymous function declaration",
        )?;

        let body = match self.block_statement()? {
            Stmt::Block { statements } => statements,
            _ => panic!("Block statement parsed something that was not a block"),
        };

        Ok(Expr::AnonFunction {
            id: self.get_id(),
            paren,
            arguments: parameters,
            body,
        })
    }

    fn assignment(&mut self) -> Result<Expr, String> {
        // a = 2; NOT var a = 2;
        let expr = self.pipe()?; // a |> f = 2;

        if self.match_token(Equal) {
            let value = self.expression()?;

            match expr {
                Variable { id: _, name } => Ok(Assign {
                    id: self.get_id(),
                    name,
                    value: Box::from(value),
                }),
                Get {
                    id: _,
                    object,
                    name,
                } => Ok(Set {
                    id: self.get_id(),
                    object,
                    name,
                    value: Box::new(value),
                }),
                _ => Err("Invalid assignment target.".to_string()),
            }
        } else {
            Ok(expr)
        }
    }

    fn pipe(&mut self) -> Result<Expr, String> {
        // expr |> f
        // expr |> f1 |> f2
        // expr |> (f1 |> f2)
        // expr |> (f1 |> (f2 |> f3))
        // (expr |> f1) |> f2

        // expr |> fun (a) { return a + 1; }
        // expr |> a -> a + 1
        let mut expr = self.or()?;
        while self.match_token(Pipe) {
            let pipe = self.previous();
            let function = self.or()?;

            expr = Call {
                id: self.get_id(),
                callee: Box::new(function),
                paren: pipe,
                arguments: vec![expr],
            };
        }
        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, String> {
        let mut expr = self.and()?;

        while self.match_token(Or) {
            let operator = self.previous();
            let right = self.and()?;

            expr = Logical {
                id: self.get_id(),
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, String> {
        let mut expr = self.equality()?;

        while self.match_token(And) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Logical {
                id: self.get_id(),
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;
        while self.match_tokens(&[BangEqual, EqualEqual]) {
            let operator = self.previous();
            let rhs = self.comparison()?;
            expr = Binary {
                id: self.get_id(),
                left: Box::from(expr),
                operator,
                right: Box::from(rhs),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;

        while self.match_tokens(&[Greater, GreaterEqual, Less, LessEqual]) {
            let op = self.previous();
            let rhs = self.term()?;
            expr = Binary {
                id: self.get_id(),
                left: Box::from(expr),
                operator: op,
                right: Box::from(rhs),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;

        while self.match_tokens(&[Minus, Plus]) {
            let op = self.previous();
            let rhs = self.factor()?;
            expr = Binary {
                id: self.get_id(),
                left: Box::from(expr),
                operator: op,
                right: Box::from(rhs),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;
        while self.match_tokens(&[Slash, Star]) {
            let op = self.previous();
            let rhs = self.unary()?;
            expr = Binary {
                id: self.get_id(),
                left: Box::from(expr),
                operator: op,
                right: Box::from(rhs),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_tokens(&[Bang, Minus]) {
            let op = self.previous();
            let rhs = self.unary()?;
            Ok(Unary {
                id: self.get_id(),
                operator: op,
                right: Box::from(rhs),
            })
        } else {
            self.call()
        }
    }

    fn call(&mut self) -> Result<Expr, String> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token(LeftParen) {
                expr = self.finish_call(expr)?;
            } else if self.match_token(Dot) {
                let name = self.consume(Identifier, "Expected token after dot-accessor")?;
                expr = Get {
                    id: self.get_id(),
                    object: Box::new(expr),
                    name,
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, String> {
        let mut arguments = vec![];

        if !self.check(RightParen) {
            loop {
                let arg = self.expression()?;
                arguments.push(arg);
                if arguments.len() >= 255 {
                    let location = self.peek().line_number;
                    return Err(format!(
                        "Line {location}: Cant have more than 255 arguments"
                    ));
                }

                if !self.match_token(Comma) {
                    break;
                }
            }
        }
        let paren = self.consume(RightParen, "Expected ')' after arguments.")?;

        Ok(Call {
            id: self.get_id(),
            callee: Box::new(callee),
            paren,
            arguments,
        })
    }

    fn primary(&mut self) -> Result<Expr, String> {
        let token = self.peek();
        let result;
        match token.token_type {
            LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(RightParen, "Expected ')'")?;
                result = Grouping {
                    id: self.get_id(),
                    expression: Box::from(expr),
                };
            }
            False | True | Nil | Number | StringLit => {
                self.advance();
                result = Literal {
                    id: self.get_id(),
                    value: LiteralValue::from_token(token),
                }
            }
            Identifier => {
                self.advance();
                result = Variable {
                    id: self.get_id(),
                    name: self.previous(),
                };
            }
            TokenType::This => {
                self.advance();
                result = Expr::This {
                    id: self.get_id(),
                    keyword: token,
                };
            }
            TokenType::Super => {
                // Should always occur with a method call
                self.advance();
                self.consume(TokenType::Dot, "Expected '.' after 'super'.")?;
                let method =
                    self.consume(TokenType::Identifier, "Expected superclass method name.")?;
                result = Expr::Super {
                    id: self.get_id(),
                    keyword: token,
                    method,
                };
            }
            Fun => {
                self.advance();
                result = self.function_expression()?;
            }
            _ => return Err("Expected expression".to_string()),
        }

        Ok(result)
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<Token, String> {
        let token = self.peek();
        if token.token_type == token_type {
            self.advance();
            let token = self.previous();
            Ok(token)
        } else {
            Err(format!("Line {}: {}", token.line_number, msg))
        }
    }

    fn check(&mut self, typ: TokenType) -> bool {
        self.peek().token_type == typ
    }

    fn match_token(&mut self, typ: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            if self.peek().token_type == typ {
                self.advance();
                true
            } else {
                false
            }
        }
    }

    fn match_tokens(&mut self, typs: &[TokenType]) -> bool {
        for typ in typs {
            if self.match_token(*typ) {
                return true;
            }
        }

        false
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn peek(&mut self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&mut self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().token_type == Eof
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == Semicolon {
                return;
            }

            match self.peek().token_type {
                Class | Fun | Var | For | If | While | Print | Return => return,
                _ => (),
            }

            self.advance();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::{LiteralValue::*, Scanner};

    #[test]
    fn test_addition() {
        let one = Token {
            token_type: Number,
            lexeme: "1".to_string(),
            literal: Some(FValue(1.0)),
            line_number: 0,
        };
        let plus = Token {
            token_type: Plus,
            lexeme: "+".to_string(),
            literal: None,
            line_number: 0,
        };
        let two = Token {
            token_type: Number,
            lexeme: "2".to_string(),
            literal: Some(FValue(2.0)),
            line_number: 0,
        };
        let semicol = Token {
            token_type: Semicolon,
            lexeme: ";".to_string(),
            literal: None,
            line_number: 0,
        };
        let eof = Token {
            token_type: Eof,
            lexeme: "".to_string(),
            literal: None,
            line_number: 0,
        };

        let tokens = vec![one, plus, two, semicol, eof];
        let mut parser = Parser::new(tokens);

        let parsed_expr = parser.parse().unwrap();
        let string_expr = parsed_expr[0].to_string();

        assert_eq!(string_expr, "(+ 1 2)");
    }

    #[test]
    fn test_comparison() {
        let source = "1 + 2 == 5 + 7;";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let parsed_expr = parser.parse().unwrap();
        let string_expr = parsed_expr[0].to_string();

        assert_eq!(string_expr, "(== (+ 1 2) (+ 5 7))");
    }

    #[test]
    fn test_eq_with_paren() {
        let source = "1 == (2 + 2);";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let parsed_expr = parser.parse().unwrap();
        let string_expr = parsed_expr[0].to_string();

        assert_eq!(string_expr, "(== 1 (group (+ 2 2)))");
    }
}
