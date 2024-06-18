use crate::expr::Expr;
use crate::scanner::Token;
use crate::stmt::Stmt;
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq)]
enum FunctionType {
    None,
    Function,
    Method,
}

#[allow(dead_code)]
pub struct Resolver {
    scopes: Vec<HashMap<String, bool>>,
    current_function: FunctionType,
    locals: HashMap<usize, usize>,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            scopes: vec![],
            current_function: FunctionType::None,
            locals: HashMap::new(),
        }
    }

    fn resolve_internal(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Block { statements: _ } => self.resolve_block(stmt)?,
            Stmt::Var {
                name: _,
                initializer: _,
            } => self.resolve_var(stmt)?,
            Stmt::Class {
                name,
                methods,
                superclass,
            } => {
                // Resolve superclass, if present
                if let Some(super_expr) = superclass {
                    if let Expr::Variable {
                        id: _,
                        name: super_name,
                    } = super_expr
                    {
                        if super_name.lexeme == name.lexeme {
                            return Err("A class cannot inherit from itself".to_string());
                        }
                    }

                    self.resolve_expr(super_expr)?;
                    self.begin_scope();
                    self.scopes
                        .last_mut()
                        .unwrap()
                        .insert("super".to_string(), true);
                }

                // Resolving class
                self.declare(name)?;
                self.define(name);

                // Resolving methods
                self.begin_scope();
                self.scopes
                    .last_mut()
                    .unwrap()
                    .insert("this".to_string(), true);
                for method in methods {
                    let declaration = FunctionType::Method;
                    self.resolve_function(method, declaration)?;
                }
                self.end_scope();

                if superclass.is_some() {
                    self.end_scope();
                }
            }
            Stmt::Function {
                name: _,
                params: _,
                body: _,
            } => self.resolve_function(stmt, FunctionType::Function)?,
            Stmt::CmdFunction { name: _, cmd: _ } => self.resolve_var(stmt)?,
            Stmt::Expression { expression } => self.resolve_expr(expression)?,
            Stmt::IfStmt {
                predicate: _,
                then: _,
                els: _,
            } => self.resolve_if_stmt(stmt)?,
            Stmt::Print { expression } => self.resolve_expr(expression)?,
            Stmt::ReturnStmt { keyword: _, value } => {
                if self.current_function == FunctionType::None {
                    return Err("Return statement is not allowed outside of a function".to_string());
                }

                if let Some(value) = value {
                    self.resolve_expr(value)?;
                }
            }
            Stmt::WhileStmt { condition, body } => {
                self.resolve_expr(condition)?;
                self.resolve_internal(body.as_ref())?;
            }
        }
        Ok(())
    }

    fn resolve_many(&mut self, stmts: &Vec<&Stmt>) -> Result<(), String> {
        for stmt in stmts {
            self.resolve_internal(stmt)?;
        }

        Ok(())
    }

    pub fn resolve(mut self, stmts: &Vec<&Stmt>) -> Result<HashMap<usize, usize>, String> {
        self.resolve_many(stmts)?;
        Ok(self.locals)
    }

    fn resolve_block(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Block { statements } => {
                self.begin_scope();
                self.resolve_many(&statements.iter().map(|b| b.as_ref()).collect())?;
                self.end_scope();
            }
            _ => panic!("Wrong type"),
        }

        Ok(())
    }

    fn resolve_var(&mut self, stmt: &Stmt) -> Result<(), String> {
        if let Stmt::Var { name, initializer } = stmt {
            self.declare(name)?;
            self.resolve_expr(initializer)?;
            self.define(name);
        } else if let Stmt::CmdFunction {name, cmd: _} = stmt {
            self.declare(name)?;
            self.define(name);
        } else {
            panic!("Wrong type in resolve var");
        }

        Ok(())
    }

    fn resolve_function(&mut self, stmt: &Stmt, fn_type: FunctionType) -> Result<(), String> {
        if let Stmt::Function { name, params, body } = stmt {
            self.declare(name)?;
            self.define(name);

            self.resolve_function_helper(
                params,
                &body.iter().map(|b| b.as_ref()).collect(),
                fn_type,
            )
        } else {
            panic!("Wrong type in resolve function");
        }
    }

    fn resolve_if_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        if let Stmt::IfStmt {
            predicate,
            then,
            els,
        } = stmt
        {
            self.resolve_expr(predicate)?;
            self.resolve_internal(then.as_ref())?;
            if let Some(els) = els {
                self.resolve_internal(els.as_ref())?;
            }

            Ok(())
        } else {
            panic!("Wrong type in resolve if stmt");
        }
    }

    fn resolve_function_helper(
        &mut self,
        params: &Vec<Token>,
        body: &Vec<&Stmt>,
        resolving_function: FunctionType,
    ) -> Result<(), String> {
        let enclosing_function = self.current_function;
        self.current_function = resolving_function;
        self.begin_scope();
        for param in params {
            self.declare(param)?;
            self.define(param);
        }
        self.resolve_many(body)?;
        self.end_scope();
        self.current_function = enclosing_function;
        Ok(())
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop().expect("Stack underflow");
    }

    fn declare(&mut self, name: &Token) -> Result<(), String> {
        let size = self.scopes.len();
        if self.scopes.is_empty() {
            return Ok(());
        }

        if self.scopes[size - 1].contains_key(&name.lexeme.clone()) {
            return Err("A variable with this name is already in scope".to_string());
        }

        self.scopes[size - 1].insert(name.lexeme.clone(), false);

        Ok(())
    }

    fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        let size = self.scopes.len();
        self.scopes[size - 1].insert(name.lexeme.clone(), true);
    }

    // (i > j) may require different resolution distances
    // { var a = 2; fun fn() { return a;} { var a = 1; var b = fn(); } }
    // (i > 3) -> take id -> store resolution distance
    // (i > 3) ->
    //         -> i -> try to resolve
    //         -> 3 -> try to resolve (trivial)
    fn resolve_expr(&mut self, expr: &Expr) -> Result<(), String> {
        match expr {
            Expr::Variable { id: _, name: _ } => self.resolve_expr_var(expr, expr.get_id()),
            Expr::Assign {
                id: _,
                name: _,
                value: _,
            } => self.resolve_expr_assign(expr, expr.get_id()),
            Expr::Binary {
                id: _,
                left,
                operator: _,
                right,
            } => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)
            }
            Expr::Call {
                id: _,
                callee,
                paren: _,
                arguments,
            } => {
                self.resolve_expr(callee.as_ref())?;
                for arg in arguments {
                    self.resolve_expr(arg)?;
                }

                Ok(())
            }
            Expr::Get {
                id: _,
                object,
                name: _,
            } => self.resolve_expr(object),
            Expr::Grouping { id: _, expression } => self.resolve_expr(expression),
            Expr::Literal { id: _, value: _ } => Ok(()),
            Expr::Logical {
                id: _,
                left,
                operator: _,
                right,
            } => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)
            }
            Expr::Set {
                id: _,
                object,
                name: _,
                value,
            } => {
                self.resolve_expr(value)?;
                self.resolve_expr(object)
            }
            Expr::This { id: _, keyword } => {
                if self.current_function != FunctionType::Method {
                    return Err("Cannot use 'this' keyword outside of a class".to_string());
                }
                self.resolve_local(keyword, expr.get_id())
            }
            Expr::Super {
                id: _,
                keyword,
                method: _,
            } => {
                if self.current_function != FunctionType::Method {
                    return Err("Cannot use 'super' keyword outside of a class".to_string());
                }
                if self.scopes.len() < 3 || !self.scopes[self.scopes.len() - 3].contains_key("super") {
                    return Err("Class has no superclass".to_string());
                }
                self.resolve_local(keyword, expr.get_id())
            }
            Expr::Unary {
                id: _,
                operator: _,
                right,
            } => self.resolve_expr(right),
            Expr::AnonFunction {
                id: _,
                paren: _,
                arguments,
                body,
            } => self.resolve_function_helper(
                arguments,
                &body.iter().map(|b| b.as_ref()).collect(),
                FunctionType::Function,
            ),
        }
    }

    fn resolve_expr_var(&mut self, expr: &Expr, resolve_id: usize) -> Result<(), String> {
        match expr {
            Expr::Variable { id: _, name } => {
                if !self.scopes.is_empty() {
                    if let Some(false) = self.scopes[self.scopes.len() - 1].get(&name.lexeme) {
                        return Err("Can't read local variable in its own initializer".to_string());
                    }
                }

                self.resolve_local(name, resolve_id)
            }
            Expr::Call {
                id: _,
                callee,
                paren: _,
                arguments: _,
            } => match callee.as_ref() {
                Expr::Variable { id: _, name } => self.resolve_local(&name, resolve_id),
                _ => panic!("Wrong type in resolve_expr_var"),
            },
            _ => panic!("Wrong type in resolve_expr_var"),
        }
    }

    fn resolve_local(&mut self, name: &Token, resolve_id: usize) -> Result<(), String> {
        let size = self.scopes.len();
        if size == 0 {
            return Ok(());
        }

        for i in (0..=(size - 1)).rev() {
            let scope = &self.scopes[i];
            if scope.contains_key(&name.lexeme) {
                self.locals.insert(resolve_id, size - 1 - i);
                return Ok(());
            }
        }

        // Assume it's global
        Ok(())
    }

    fn resolve_expr_assign(&mut self, expr: &Expr, resolve_id: usize) -> Result<(), String> {
        if let Expr::Assign { id: _, name, value } = expr {
            self.resolve_expr(value.as_ref())?;
            self.resolve_local(name, resolve_id)?;
        } else {
            panic!("Wrong type in resolve assign");
        }

        Ok(())
    }
}
