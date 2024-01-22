use crate::environment::Environment;
use crate::stmt::Stmt;
use crate::{expr::Expr, scanner::LiteralValue};

pub struct Interpreter {
    env: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
        }
    }

    pub fn interpret(&mut self, stmts: &Vec<Stmt>) -> Result<(), String> {
        for stmt in stmts {
            self.interpret_stmt(stmt.clone())?;
        }
        Ok(())
    }

    fn interpret_stmt(&mut self, stmt: Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Expression { expression } => {
                expression.evaluate(&self.env)?;
            }
            Stmt::Print { expression } => {
                let value = expression.evaluate(&self.env)?;
                println!("print op: {}", value.to_string());
            }
            Stmt::Var { name, initializer } => {
                if initializer != Expr::Literal(LiteralValue::NIL) {
                    let value = initializer.evaluate(&self.env)?;
                    self.env.define(name.lexeme, value);
                } else {
                    self.env.define(name.lexeme, LiteralValue::NIL);
                }
            }
        }
        Ok(())
    }
}
