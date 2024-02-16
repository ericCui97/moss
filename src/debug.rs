use crate::{token::Token, expr::Expr};


trait DebugString{
    fn debug_string(&self) -> String;
}

impl DebugString for Token {
    fn debug_string(&self) -> String {
        format!("{:?} {}", self.token_type, self.lexeme)
    }
}

impl DebugString for Expr{
    
}