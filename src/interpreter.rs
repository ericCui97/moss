use crate::environment::define;
use crate::stmt::Stmt;
use crate::{expr::Expr, scanner::LiteralValue};
pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&self, stmts: &Vec<Stmt>) -> Result<(), String> {
        for stmt in stmts {
            self.interpret_stmt(stmt.clone())?;
        }
        Ok(())
    }

    fn interpret_stmt(&self, stmt: Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Expression { expression } => {
                expression.evaluate()?;
            }
            Stmt::Print { expression } => {
                let value = expression.evaluate()?;
                println!("print op: {}", value.to_string());
            }
            Stmt::Var { name, initializer } => {
                if initializer != Expr::Literal(LiteralValue::NIL) {
                    let value = initializer.evaluate()?;
                    define(name.lexeme, value);
                } else {
                    define(name.lexeme, LiteralValue::NIL);
                }
            }
        }
        Ok(())
    }
}
