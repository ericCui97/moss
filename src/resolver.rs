use std::collections::HashMap;

use crate::expr::Expr;
use crate::interpreter::Interpreter;
use crate::stmt::Stmt;
use crate::token::Token;

pub struct Resolver {
    interpreter: Interpreter,
    scopes: Vec<HashMap<String, bool>>,
}
#[allow(dead_code)]
impl Resolver {
    pub fn new(interpreter: Interpreter) -> Self {
        Self {
            interpreter,
            scopes: Vec::new(),
        }
    }
    pub fn resolve(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Block { statements: _ } => Ok(self.resolve_block(stmt.clone())?),
            Stmt::Function { name:_, params:_, body:_ } => Ok(self.resolve_function(stmt)?),
            Stmt::Var { name:_, initializer:_ } => Ok(self.resolve_var(stmt)?),
            Stmt::Expression { expression } => Ok(self.resolve_expr(expression)?),
            Stmt::IfStmt { condition:_, then_branch:_, else_branch:_ }=> Ok(self.resolve_if(stmt)?),
            Stmt::Print { expression }=> Ok(self.resolve_expr(expression)?),
            Stmt::Return { keyword:_, value }=> {
                if let Some(value) = value {
                    self.resolve_expr(value)?;
                }
                Ok(())
            }
            Stmt::WhileStmt { condition, body }=>{
                self.resolve_expr(condition)?;
                self.resolve(body.as_ref())?;
                Ok(())
            }
            _ => panic!("Expected block statement in resolve"),
        }
    }

    fn resolve_function(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Function { name, params, body } => {
                self.declare(name.lexeme.as_str());
                self.define(name.lexeme.as_str());
                self.resolve_function_body(params, body);
                Ok(())
            }
            _ => panic!("Expected function statement in resolve_function"),
        }
    }

    fn resolve_function_body(&mut self, params: &Vec<Token>, body: &Vec<Box<Stmt>>) {
        self.resolve_begin();
        for param in params {
            self.declare(param.lexeme.as_str());
            self.define(param.lexeme.as_str());
        }
        for stmt in body {
            self.resolve(stmt).unwrap();
        }
        self.resolve_end();
    }

    fn resolve_if(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::IfStmt { condition, then_branch, else_branch } => {
                self.resolve_expr(condition)?;
                self.resolve(then_branch.as_ref())?;
                if let Some(else_branch) = else_branch {
                    self.resolve(else_branch.as_ref())?;
                }
                Ok(())
            }
            _ => panic!("Expected if statement in resolve_if"),
        }
    }
    fn resolve_var(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Var { name, initializer } => {
                self.declare(name.lexeme.as_str());
                self.resolve_expr(initializer)?;
                self.define(name.lexeme.as_str());
                Ok(())
            }
            _ => panic!("Expected var statement in resolve_var"),
        }
    }

    fn declare(&mut self, name: &str) {
        if self.scopes.is_empty() {
            return;
        }
        self.scopes
            .last_mut()
            .expect("Expected scope to last_mut")
            .insert(name.to_string(), false);
    }

    fn define(&mut self, name: &str) {
        if self.scopes.is_empty() {
            return;
        }
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), true);
        }
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<(), String> {
        match expr {
            Expr::Variable(_name) => self.resolve_var_expr(expr),
            Expr::Binary(left,_ ,right )=> {
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
                Ok(())
            }
            Expr::Call { callee, paren:_, arguments }=>{
                self.resolve_expr(callee.as_ref())?;
                for arg in arguments {
                    self.resolve_expr(arg)?;
                }
                Ok(())
            }
            Expr::Grouping(expr) => self.resolve_expr(expr),
            Expr::Literal(_value) => Ok(()),
            Expr::Logical(left, _,right)=>{
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
                Ok(())
            }
            Expr::Unary(_, right)=>{
                self.resolve_expr(right)?;
                Ok(())
            }
            Expr::AnonymousFn {body,params }=>{
                self.resolve_begin();
                for param in params {
                    self.declare(param.lexeme.as_str());
                    self.define(param.lexeme.as_str());
                }
                for stmt in body {
                    self.resolve(stmt).unwrap();
                }
                self.resolve_end();
                Ok(())
            }

            _ => panic!("Expected variable expression in resolve_expr"),
        }
    }

    fn resolve_var_expr(&mut self, expr: &Expr) -> Result<(), String> {
        if let Expr::Variable(name) = expr {
            if let Some(scope) = self.scopes.last() {
                if let Some(declared) = scope.get(name.lexeme.as_str()) {
                    if !declared {
                        return Err("Cannot read local variable in its own initializer".to_string());
                    }
                    self.resolve_local(expr, name)?;
                }
            } else {
                panic!("wrong type in resolve_var_expr");
            }
        }

        Ok(())
    }

    fn resolve_local(&mut self, expr: &Expr, name: &Token) -> Result<(), String> {
        for (i, scope) in self.scopes.iter().enumerate().rev() {
            if scope.contains_key(name.lexeme.as_str()) {
                self.interpreter.resolve(name, i);
                return Ok(());
            }
        }
        Ok(())
    }

    fn resolve_expr_assignment(&mut self, expr: &Expr) -> Result<(), String> {
        match expr {
            Expr::Assign { name, value } => {
                self.resolve_expr(value)?;
                self.resolve_local(expr, name)?;
                Ok(())
            }
            _ => panic!("Expected assign expression in resolve_expr_assignment"),
        }
    }

    fn resolve_block(&mut self, stmt: Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Block { statements } => {
                self.resolve_begin();
                for stmt in statements {
                    self.resolve(&stmt);
                }
                self.resolve_end();
            }
            _ => panic!("Expected block statement in resolve"),
        }
        todo!()
    }

    fn resolve_begin(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn resolve_end(&mut self) {
        self.scopes.pop().expect("Expected scope to pop");
    }
}
