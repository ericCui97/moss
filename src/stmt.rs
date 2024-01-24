use crate::{expr::Expr, scanner::Token};
#[derive(Debug, Clone)]
pub enum Stmt {
    Expression { expression: Expr },
    Print { expression: Expr },
    Var { name: Token, initializer: Expr },
    Block { statements: Vec<Stmt> },
    IfStmt {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },

}
impl Stmt {
    #[allow(clippy::inherent_to_string)]
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        match self {
            Stmt::Expression { expression } => {
                format!("E({})", expression.to_string())
            }
            Stmt::Print { expression } => {
                format!("P({})", expression.to_string())
            }
            Stmt::Var { name, initializer } => {
                format!("V({} {})", name.lexeme, initializer.to_string())
            }
            Stmt::Block { statements } => {
                let mut s = String::from("B(");
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
                let mut s = String::from("I(");
                s.push_str(&condition.to_string());
                s.push_str(&then_branch.to_string());
                if let Some(else_branch) = else_branch {
                    s.push_str(&else_branch.to_string());
                }
                s.push(')');
                s
            }
        }
    }
}