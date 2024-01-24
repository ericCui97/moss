use crate::environment::Environment;
use crate::stmt::Stmt;
use crate::{expr::Expr, scanner::LiteralValue};
use std::rc::Rc;

pub struct Interpreter {
    env: Rc<Environment>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Rc::new(Environment::new()),
        }
    }

    pub fn interpret(&mut self, stmts: &Vec<Stmt>) -> Result<(), String> {
        for stmt in stmts {
            self.interpret_stmt(stmt.clone())?;
        }
        Ok(())
    }

    pub fn get_mut_env(&mut self) -> &mut Environment {
        Rc::get_mut(&mut self.env)
            .expect("cant get mut from Rc<Environment> in Interpreter::get_mut_env")
    }

    fn interpret_stmt(&mut self, stmt: Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Expression { expression } => {
                expression.evaluate(self.get_mut_env())?;
            }
            Stmt::Print { expression } => {
                let value = expression.evaluate(self.get_mut_env())?;
                println!("print op: {}", value.to_string());
            }
            Stmt::Var { name, initializer } => {
                if initializer != Expr::Literal(LiteralValue::NIL) {
                    let value = initializer.evaluate(self.get_mut_env())?;
                    self.get_mut_env().define(name.lexeme, value);
                } else {
                    self.get_mut_env().define(name.lexeme, LiteralValue::NIL);
                }
            }
            Stmt::Block { statements } => {
                let mut new_env = Environment::new();
                new_env.enclosing = Some(self.env.clone());
                let old_env = self.env.clone();
                self.env = Rc::new(new_env);
                for stmt in statements {
                    self.interpret_stmt(stmt)?;
                }
                self.env = old_env;
            }
            Stmt::IfStmt {
                condition,
                then_branch,
                else_branch,
            } => {
               let condition = condition.evaluate(self.get_mut_env())?;
                if condition.is_truthy() {
                     self.interpret_stmt(*then_branch)?;
                } else if let Some(else_branch) = else_branch {
                     self.interpret_stmt(*else_branch)?;
                }
            }
        }
        Ok(())
    }
}
