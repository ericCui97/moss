use crate::expr::Expr;
use crate::scanner::Token;

#[derive(Debug, Clone)]
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
    Block {
        statements: Vec<Box<Stmt>>,
    },
    Class {
        name: Token,
        methods: Vec<Box<Stmt>>,
        superclass: Option<Expr>,
    },
    IfStmt {
        predicate: Expr,
        then: Box<Stmt>,
        els: Option<Box<Stmt>>,
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
    CmdFunction {
        name: Token,
        cmd: String,
    },
    ReturnStmt {
        keyword: Token,
        value: Option<Expr>,
    },
}
// 改了文件批处理测试方法， to_string out !
impl Stmt {
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        use Stmt::*;
        match self {
            Expression { expression } => expression.to_string(),
            Print { expression } => format!("(print {})", expression.to_string()),
            Var {
                name,
                initializer: _,
            } => format!("(var {})", name.lexeme),
            Block { statements } => format!(
                "(block {})",
                statements
                    .into_iter()
                    .map(|stmt| stmt.to_string())
                    .collect::<String>()
            ),
            IfStmt {
                predicate: _,
                then: _,
                els: _,
            } => todo!(),
            WhileStmt {
                condition: _,
                body: _,
            } => todo!(),
            Function {
                name: _,
                params: _,
                body: _,
            } => todo!(),
            CmdFunction { name: _, cmd: _ } => todo!(),
            ReturnStmt { keyword: _, value: _ } => todo!(),
            _ => todo!(),
        }
    }
}
