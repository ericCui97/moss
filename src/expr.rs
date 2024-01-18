use std::ops::Deref;

use crate::scanner::{LiteralValue, Token, TokenType};
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
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        use crate::scanner::LiteralValue::*;
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

    pub fn evaluate(&self) -> Result<LiteralValue, String> {
        match self {
            Expr::Literal(lit) => Ok(lit.clone()),
            Expr::Grouping(e) => e.evaluate(),
            Expr::Unary(op, right) => {
                let right = (*right).evaluate();
                match op.token_type {
                    TokenType::MINUS => match right {
                        Ok(LiteralValue::NUMBER(n)) => Ok(LiteralValue::NUMBER(-n)),
                        _ => Err(format!("unary minus can only apply to number")),
                    },
                    TokenType::BANG => match right.unwrap().unwrap_as_boolean() {
                        LiteralValue::BOOLEAN(b) => Ok(LiteralValue::BOOLEAN(!b.clone())),
                        _ => Err(format!("unary ! can only apply to boolean")),
                    },
                    _ => Err(format!("unary operator {:?} not supported", op.token_type)),
                }
            }
            Expr::Binary(left, op, right) => {
                let left = left.evaluate()?;
                let right = right.evaluate()?;
                match (left, op.token_type, right) {
                    (LiteralValue::NUMBER(x), TokenType::PLUS, LiteralValue::NUMBER(y)) => {
                        Ok(LiteralValue::NUMBER(x + y))
                    }
                    (LiteralValue::NUMBER(x), TokenType::MINUS, LiteralValue::NUMBER(y)) => {
                        Ok(LiteralValue::NUMBER(x - y))
                    }
                    (LiteralValue::NUMBER(x), TokenType::STAR, LiteralValue::NUMBER(y)) => {
                        Ok(LiteralValue::NUMBER(x * y))
                    }
                    (LiteralValue::NUMBER(x), TokenType::SLASH, LiteralValue::NUMBER(y)) => {
                        Ok(LiteralValue::NUMBER(x / y))
                    }
                    (LiteralValue::NUMBER(x), TokenType::GREATER, LiteralValue::NUMBER(y)) => {
                        Ok(LiteralValue::BOOLEAN(x > y))
                    }
                    (
                        LiteralValue::NUMBER(x),
                        TokenType::GREATER_EQUAL,
                        LiteralValue::NUMBER(y),
                    ) => Ok(LiteralValue::BOOLEAN(x >= y)),
                    (LiteralValue::NUMBER(x), TokenType::LESS, LiteralValue::NUMBER(y)) => {
                        Ok(LiteralValue::BOOLEAN(x < y))
                    }
                    (LiteralValue::NUMBER(x), TokenType::LESS_EQUAL, LiteralValue::NUMBER(y)) => {
                        Ok(LiteralValue::BOOLEAN(x <= y))
                    }
                    (LiteralValue::NUMBER(x), TokenType::BANG_EQUAL, LiteralValue::NUMBER(y)) => {
                        Ok(LiteralValue::BOOLEAN(x != y))
                    }
                    (LiteralValue::NUMBER(x), TokenType::EQUAL_EQUAL, LiteralValue::NUMBER(y)) => {
                        Ok(LiteralValue::BOOLEAN(x == y))
                    }
                    (LiteralValue::STRING(x), TokenType::PLUS, LiteralValue::STRING(y)) => {
                        Ok(LiteralValue::STRING(x + y.as_str()))
                    }
                    (LiteralValue::STRING(x), TokenType::EQUAL_EQUAL, LiteralValue::STRING(y)) => {
                        Ok(LiteralValue::BOOLEAN(x == y))
                    }
                    (LiteralValue::STRING(x), TokenType::BANG_EQUAL, LiteralValue::STRING(y)) => {
                        Ok(LiteralValue::BOOLEAN(x != y))
                    }
                    (
                        LiteralValue::BOOLEAN(x),
                        TokenType::EQUAL_EQUAL,
                        LiteralValue::BOOLEAN(y),
                    ) => Ok(LiteralValue::BOOLEAN(x == y)),
                    (LiteralValue::BOOLEAN(x), TokenType::BANG_EQUAL, LiteralValue::BOOLEAN(y)) => {
                        Ok(LiteralValue::BOOLEAN(x != y))
                    }
                    _ => Err(format!(
                        "binary operator {:?}  not supported",
                        op.token_type
                    )),
                }
            }
        }
    }
}
//---------------------------------test---------------------------------//
// #region
#[cfg(test)]
mod tests {

    use std::fs::File;
    use std::io::Read;

    use serde::Deserialize;
    use serde::Serialize;

    use crate::expr::Expr;
    use crate::parser::Parser;
    use crate::scanner::LiteralValue;
    use crate::scanner::Scanner;

    #[derive(Debug, Serialize, Deserialize)]
    struct Test {
        name: String,
        expr: String,
        expected_result: f64,
    }

    fn read_tests_from_file(file_path: &str) -> Vec<Test> {
        let mut file = File::open(file_path).expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");
        serde_json::from_str(&contents).expect("Failed to parse JSON")
    }
    #[test]
    fn test_unary() {
        // let expr = "!0";
        // let mut scanner = Scanner::new(expr);
        // let tokens = scanner.scan_tokens().unwrap();
        // let mut parser = Parser::new(tokens);
        // let expr = parser.parse().unwrap();
        // assert_eq!(expr.evaluate().unwrap(),LiteralValue::BOOLEAN(true));
        let exprs = vec!["!0", "!1", "!true", "!false", "!nil"];
        let res = vec![true, false, false, true, true];
        for (i, expr) in exprs.iter().enumerate() {
            let mut scanner = Scanner::new(expr);
            let tokens = scanner.scan_tokens().unwrap();
            let mut parser = Parser::new(tokens);
            let expr = parser.parse().unwrap();
            dbg!(i, expr.to_string(), res[i]);
            assert_eq!(expr.evaluate().unwrap(), LiteralValue::BOOLEAN(res[i]));
        }
    }
    #[test]
    fn test_unary2() {
        let expr = "!\"hello world\"";
        let mut scanner = Scanner::new(expr);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        dbg!(expr.to_string(), expr.evaluate().unwrap());
        assert_eq!(expr.evaluate().unwrap(), LiteralValue::BOOLEAN(false));
    }
    #[test]
    fn test_unary3() {
        let exprs = vec!["1", "-1", "-0", "-1.1", "-0.1"];
        let res = vec![1.0, -1.0, 0.0, -1.1, -0.1];
        for (i, expr) in exprs.iter().enumerate() {
            let mut scanner = Scanner::new(expr);
            let tokens = scanner.scan_tokens().unwrap();
            let mut parser = Parser::new(tokens);
            let expr = parser.parse().unwrap();
            assert_eq!(expr.evaluate().unwrap(), LiteralValue::NUMBER(res[i]));
        }
    }
    #[test]
    fn test_all() {
        let tests = read_tests_from_file("test_file/expr.json");
        print!("{:?}", tests);
        for test in tests {
            let mut scanner = Scanner::new(&test.expr);
            let tokens = scanner.scan_tokens().unwrap();
            let mut parser = Parser::new(tokens);
            let expr = parser.parse().unwrap();
            dbg!(&test.name, &test.expr, test.expected_result);
            assert_eq!(
                expr.evaluate().unwrap(),
                LiteralValue::NUMBER(test.expected_result),
                "Test '{}' failed",
                &test.name
            );
        }
    }
}
// #endregion
