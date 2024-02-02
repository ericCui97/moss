use std::fmt::Debug;

use crate::expr::Expr;
use crate::token::Token;

#[allow(clippy::enum_variant_names)]
#[allow(clippy::vec_box)]
#[derive(Clone)]
pub enum Stmt {
    Expression {
        expression: Expr,
    },
    Print {
        expression: Expr,
    },
    Var {
        name: Token,
        initializer: Expr,
    },
    IfStmt {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    Block {
        statements: Vec<Box<Stmt>>,
    },
    WhileStmt {
        condition: Expr,
        body: Box<Stmt>,
    },
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Box<Stmt>>,
    },
    Return {
        keyword: Token,
        value: Option<Expr>,
    },
}

impl Debug for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.to_string())
    }
}
impl Stmt {
    #[allow(clippy::inherent_to_string)]
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        match self {
            Stmt::Return { keyword, value } => match value {
                Some(expr) => format!("Return ({},{})", keyword.lexeme, expr.to_string()),
                None => format!("Return ({},void)", keyword.lexeme),
            },
            Stmt::WhileStmt { condition, body } => {
                format!("While ({} {})", condition.to_string(), body.to_string())
            }
            Stmt::Expression { expression } => {
                format!("Expression ({})", expression.to_string())
            }
            Stmt::Print { expression } => {
                format!("Print ({})", expression.to_string())
            }
            Stmt::Var { name, initializer } => {
                format!("Var ({},{})", name.lexeme, initializer.to_string())
            }
            Stmt::Block { statements } => {
                let mut s = String::from("Block (");
                for stmt in statements {
                    s.push_str(&stmt.to_string());
                }
                s.push(')');
                s
            }
            Stmt::IfStmt {
                condition,
                then_branch,
                else_branch,
            } => {
                let mut s = String::from("If(");
                s.push_str(&condition.to_string());
                s.push_str(&then_branch.to_string());
                if let Some(else_branch) = else_branch {
                    s.push_str(&else_branch.to_string());
                }
                s.push(')');
                s
            }
            Stmt::Function { name, params, body } => {
                let mut s = String::from("Function (");
                s.push_str(&name.lexeme);
                s.push_str(&params.len().to_string());
                for stmt in body {
                    s.push_str(&stmt.to_string());
                }
                s.push(')');
                s
            }
        }
    }
}
