use crate::environment::Environment;
use crate::scanner::{LiteralValue, Token, TokenType};
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
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

    Assign {
        name: Token,
        value: Box<Expr>,
    },

    Logical(Box<Expr>, Token, Box<Expr>),

    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
}

impl std::fmt::Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.to_string())
    }
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
            Expr::Assign { name, value } => {
                format!("A({} {})", name.lexeme, value.to_string())
            }
            Expr::Logical(left, op, right) => {
                format!(
                    "Logical({} {} {})",
                    left.to_string(),
                    op.lexeme,
                    right.to_string()
                )
            }
            Expr::Call {
                callee,
                paren,
                arguments,
            } => {
                let mut s = String::from("Call(");
                s.push_str(&callee.to_string());
                s.push_str(&paren.lexeme);
                for arg in arguments {
                    s.push_str(&arg.to_string());
                }
                s.push(')');
                s
            }
        }
    }

    // 执行表达式
    pub fn evaluate(&self, env: Rc<RefCell<Environment>>) -> Result<LiteralValue, String> {
        match self {
            Expr::Logical(left, op, right) => {
                let left_value = left.evaluate(env.clone())?;
                let left_true = left_value.is_truthy();
                match op.token_type {
                    TokenType::OR => {
                        if left_true {
                            Ok(left_value)
                        } else {
                            right.evaluate(env.clone())
                        }
                    }
                    TokenType::AND => {
                        if !left_true {
                            Ok(LiteralValue::BOOLEAN(false))
                        } else {
                            right.evaluate(env.clone())
                        }
                    }
                    _ => Err(format!(
                        "logical operator {:?} not supported",
                        op.token_type
                    )),
                }
            }
            Expr::Assign { name, value } => {
                let value = (*value).evaluate(env.clone())?;
                match env.borrow_mut().assign(name.lexeme.clone(), value) {
                    true => Ok(LiteralValue::NIL),
                    false => Err(format!(
                        "assignment failed,variable {} not found",
                        name.lexeme
                    )),
                }
            }
            Expr::Variable(name) => match env.borrow_mut().get(name.lexeme.clone()) {
                Some(v) => Ok(v.clone()),
                //                None => Err(format!("variable {} not found", name.lexeme)),
                None => {
                    println!("variable {} not found", name.lexeme);
                    Ok(LiteralValue::NIL)
                }
            },
            Expr::Literal(lit) => Ok(lit.clone()),
            Expr::Grouping(e) => e.evaluate(env),
            Expr::Unary(op, right) => {
                let right = (*right).evaluate(env);
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
            Expr::Call {
                callee,
                paren:_,
                arguments,
            } => {
                let callable = (*callee).evaluate(env.clone())?;
                match callable {
                    LiteralValue::Callable { name, arity, func } => {
                        if arguments.len() != arity {
                            return Err(format!(
                                "function {} expect {} arguments, but got {}",
                                name,
                                arity,
                                arguments.len()
                            ));
                        }
                        let mut args = vec![];
                        for arg in arguments {
                            args.push(arg.evaluate(env.clone())?);
                        }
                        Ok(func(env.clone(), &args))
                    }
                    other => Err(format!("{} is not callable", other.to_string())),
                }
            }
            Expr::Binary(left, op, right) => {
                let left = left.evaluate(env.clone())?;
                let right = right.evaluate(env.clone())?;
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
