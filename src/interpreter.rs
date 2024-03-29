use crate::environment::Environment;
use crate::expr::Expr;
use crate::stmt::Stmt;
use crate::token::{LiteralValue, Token};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::SystemTime;
#[allow(dead_code)]
pub struct Interpreter {
    pub special: Rc<RefCell<Environment>>,
    pub env: Rc<RefCell<Environment>>,
    // for resolve
    locals: HashMap<Expr, usize>,
}
#[allow(clippy::ptr_arg)]
fn clock_impl(_args: &Vec<LiteralValue>) -> LiteralValue {
    let now = std::time::SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();
    LiteralValue::NUMBER((now as f64) / 1000.0)
}
// #[allow(clippy::new_without_default)]
impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}
impl Interpreter {
    pub fn new() -> Self {
        let mut env = Environment::new();
        env.define(
            String::from("clock"),
            LiteralValue::Callable {
                name: "clock".to_string(),
                arity: 0,
                func: Rc::new(clock_impl),
            },
        );

        Self {
            special: Rc::new(RefCell::new(Environment::new())),
            env: Rc::new(RefCell::new(env)),
            locals: HashMap::new(),
        }
    }
    // make a new interpreter for closure
    fn for_closure(parent: Rc<RefCell<Environment>>) -> Self {
        let environment = Rc::new(RefCell::new(Environment::new()));
        environment.borrow_mut().enclosing = Some(parent);
        Self {
            special: Rc::new(RefCell::new(Environment::new())),
            env: environment,
            locals: HashMap::new(),
        }
    }

    pub fn for_anonymous(parent: Environment) -> Self {
        let mut env = Environment::new();
        env.enclosing = Some(Rc::new(RefCell::new(parent)));
        Self {
            special: Rc::new(RefCell::new(Environment::new())),
            env: Rc::new(RefCell::new(env)),
            locals: HashMap::new(),
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
                match initializer {
                    Expr::Literal(LiteralValue::NIL) => {
                        self.env.borrow_mut().define(name.lexeme, LiteralValue::NIL);
                    }
                    _ => {
                        let value = initializer.evaluate(self.env.clone())?;
                        self.env.borrow_mut().define(name.lexeme, value);
                    }
                }
                // if initializer != Expr::Literal(LiteralValue::NIL) {
                //     let value = initializer.evaluate(self.env.clone())?;
                //     self.env.borrow_mut().define(name.lexeme, value);
                // } else {
                //     self.env.borrow_mut().define(name.lexeme, LiteralValue::NIL);
                // }
            }
            Stmt::Block { statements } => {
                let mut new_env = Environment::new();
                new_env.enclosing = Some(self.env.clone());
                let old_env = self.env.clone();
                self.env = Rc::new(RefCell::new(new_env));
                for stmt in statements {
                    self.interpret_stmt(*stmt)?;
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
            Stmt::Function { name, params, body } => {
                let arity = params.len();
                let params: Vec<Token> = params.iter().map(|t| (*t).clone()).collect();
                let body: Vec<Box<Stmt>> = body.iter().map(|s| (*s).clone()).collect();
                let parent_env = self.env.clone();
                let func_impl = move |args: &Vec<LiteralValue>| {
                    let mut closure_int = Interpreter::for_closure(parent_env.clone());

                    for (index, arg) in args.iter().enumerate() {
                        closure_int
                            .env
                            .borrow_mut()
                            .define(params[index].lexeme.clone(), (*arg).clone());
                    }

                    for st in body.iter() {
                        // println!("st: {:?}", st);

                        closure_int.interpret(&vec![*(*st).clone()]).unwrap();
                        // .unwrap_or_else(|_| panic!("function {} execute failed", name_cloned));
                        if let Some(value) =
                            closure_int.special.borrow_mut().get("return".to_string())
                        {
                            return value;
                        }
                    }

                    LiteralValue::NIL
                };

                let callable = LiteralValue::Callable {
                    name: name.lexeme.clone(),
                    arity,
                    func: Rc::new(func_impl),
                };

                self.env.borrow_mut().define(name.lexeme, callable);
            }
            Stmt::Return { keyword: _, value } => {
                let eval_val;
                if let Some(value) = value {
                    eval_val = value.evaluate(self.env.clone())?;
                } else {
                    eval_val = LiteralValue::NIL;
                }
                self.special
                    .borrow_mut()
                    .define("return".to_string(), eval_val);
            }
        }
        Ok(())
    }
    pub fn resolve(&self, token: &Token, depth: usize) {
        // self.locals.insert(token.clone(),depth);
        todo!()
    }
}
