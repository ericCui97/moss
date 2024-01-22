use crate::environment::get;
use crate::scanner::{LiteralValue, Token, TokenType};
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    // grouping expression like (1+2)
    Grouping(Box<Expr>),
    // binary expression like 1+2
    Binary(Box<Expr>, Token, Box<Expr>),
    // literal expression like 1,2,3 or "hello world" true false nil
    Literal(LiteralValue),
    // unary expression like !true
    Unary(Token, Box<Expr>),

    // variable expression like var a = 1
    Variable(Token),
}
impl Expr {
    #[allow(clippy::inherent_to_string)]
    #[allow(dead_code)]
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
            Expr::Variable(name) => {
                format!("V({})", name.lexeme)
            }
        }
    }

    // 执行表达式
    pub fn evaluate(&self) -> Result<LiteralValue, String> {
        match self {
            Expr::Variable(name) => match get(name.lexeme.clone()) {
                Some(v) => Ok(v),
                None => Err(format!("variable {} not found", name.lexeme)),
            },
            Expr::Literal(lit) => Ok(lit.clone()),
            Expr::Grouping(e) => e.evaluate(),
            Expr::Unary(op, right) => {
                let right = (*right).evaluate();
                match op.token_type {
                    TokenType::MINUS => match right {
                        Ok(LiteralValue::NUMBER(n)) => Ok(LiteralValue::NUMBER(-n)),
                        _ => Err("unary minus can only apply to number".to_string()),
                    },
                    TokenType::BANG => match right.unwrap().unwrap_as_boolean() {
                        LiteralValue::BOOLEAN(b) => Ok(LiteralValue::BOOLEAN(!b)),
                        _ => Err("unary ! can only apply to boolean".to_string()),
                    },
                    _ => Err(format!("unary operator {:?} not supported", op.token_type)),
                }
            }
            Expr::Binary(left, op, right) => {
                let left = left.evaluate()?;
                let right = right.evaluate()?;
                match (&left, op.token_type, &right) {
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
                        Ok(LiteralValue::STRING(String::from(x) + y))
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
                    _ => {
                        let left = left.clone().to_string();
                        let right = right.clone().to_string();
                        Err(format!(
                            "binary operator {:?} between {:?} ans {:?} not supported ",
                            op.token_type, left, right
                        ))
                    }
                }
            }
        }
    }
}
//---------------------------------test---------------------------------//
// #region
#[cfg(test)]
mod tests {
    use crate::parser::Parser;
    use crate::scanner::LiteralValue;
    use crate::scanner::Scanner;
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Debug, Serialize, Deserialize)]
    struct Test {
        name: String,
        expr: String,
        expected_result: f64,
    }

    // fn read_tests_from_file(file_path: &str) -> Vec<Test> {
    //     let mut file = File::open(file_path).expect("Failed to open file");
    //     let mut contents = String::new();
    //     file.read_to_string(&mut contents)
    //         .expect("Failed to read file");
    //     serde_json::from_str(&contents).expect("Failed to parse JSON")
    // }
    #[test]
    fn test_unary() {
        // let expr = "!0";
        // let mut scanner = Scanner::new(expr);
        // let tokens = scanner.scan_tokens().unwrap();
        // let mut parser = Parser::new(tokens);
        // let expr = parser.parse().unwrap();
        // assert_eq!(expr.evaluate().unwrap(),LiteralValue::BOOLEAN(true));
        let exprs = ["!0", "!1", "!true", "!false", "!nil"];
        let res = [true, false, false, true, true];
        for (i, expr) in exprs.iter().enumerate() {
            let mut scanner = Scanner::new(expr);
            let tokens = scanner.scan_tokens().unwrap();
            let parser = Parser::new(tokens);
            let expr = parser.parse_expression().unwrap();
            assert_eq!(expr.evaluate().unwrap(), LiteralValue::BOOLEAN(res[i]));
        }
    }
    #[test]
    fn test_unary2() {
        let expr = "!\"hello world\"";
        let mut scanner = Scanner::new(expr);
        let tokens = scanner.scan_tokens().unwrap();
        let parser = Parser::new(tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr.evaluate().unwrap(), LiteralValue::BOOLEAN(false));
    }
    #[test]
    fn test_unary3() {
        let exprs = ["1", "-1", "-0", "-1.1", "-0.1"];
        let res = [1.0, -1.0, 0.0, -1.1, -0.1];
        for (i, expr) in exprs.iter().enumerate() {
            let mut scanner = Scanner::new(expr);
            let tokens = scanner.scan_tokens().unwrap();
            let parser = Parser::new(tokens);
            let expr = parser.parse_expression().unwrap();
            assert_eq!(expr.evaluate().unwrap(), LiteralValue::NUMBER(res[i]));
        }
    }
}
// #endregion
