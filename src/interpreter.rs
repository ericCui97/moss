use crate::environment::Environment;
use crate::stmt::Stmt;
use crate::{expr::Expr, scanner::LiteralValue};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Interpreter {
    env: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Rc::new(RefCell::new(Environment::new())),
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
                expression.evaluate(self.env.clone())?;
            }
            Stmt::Print { expression } => {
                let value = expression.evaluate(self.env.clone())?;
                println!("{}", value.to_string());
            }
            Stmt::Var { name, initializer } => {
                if initializer != Expr::Literal(LiteralValue::NIL) {
                    let value = initializer.evaluate(self.env.clone())?;
                    self.env.borrow_mut().define(name.lexeme, value);
                } else {
                    self.env.borrow_mut().define(name.lexeme, LiteralValue::NIL);
                }
            }
            Stmt::Block { statements } => {
                let mut new_env = Environment::new();
                new_env.enclosing = Some(self.env.clone());
                let old_env = self.env.clone();
                self.env = Rc::new(RefCell::new(new_env));
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
                let condition = condition.evaluate(self.env.clone())?;
                if condition.is_truthy() {
                    self.interpret_stmt(*then_branch)?;
                } else if let Some(else_branch) = else_branch {
                    self.interpret_stmt(*else_branch)?;
                }
            }
            Stmt::WhileStmt { condition, body } => {
                while condition.evaluate(self.env.clone())?.is_truthy() {
                    self.interpret_stmt(*body.clone())?;
                }
            }
        }
        Ok(())
    }
}

//  var a=1;while(a<4){a=a+1;print a;}
