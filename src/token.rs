use std::rc::Rc;

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    // (
    RIGHT_PAREN,
    // )
    LEFT_BRACE,
    // {
    RIGHT_BRACE,
    // }
    COMMA,
    // ,
    DOT,
    // .
    MINUS,
    // -
    PLUS,
    // +
    SEMICOLON,
    // ;
    SLASH,
    // /
    STAR,
    // *
    // one or two character tokens,
    BANG,
    // /!
    // ++
    PLUS_PLUS,
    //--
    MINUS_MINUS,
    BANG_EQUAL,
    // !=
    EQUAL,
    // =
    EQUAL_EQUAL,
    // ==
    GREATER,
    // >
    GREATER_EQUAL,
    // >=
    LESS,

    // <
    LESS_EQUAL,
    // <=
    // Literals.
    IDENTIFIER,
    // identifier
    STRING,
    // string
    NUMBER, // number

    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF, // end of file
}


type FuncType = Rc<dyn Fn(&Vec<LiteralValue>) -> LiteralValue>;
#[allow(clippy::upper_case_acronyms)]
#[derive(Clone)]
pub enum LiteralValue {
    STRING(String),
    NUMBER(f64),
    //    BOOLEAN(bool),
    NIL,
    BOOLEAN(bool),
    Callable {
        name: String,
        arity: usize,
        func: FuncType,
    },
}

impl LiteralValue {
    pub fn from_token(token: &Token) -> Result<LiteralValue, String> {
        match token.token_type {
            TokenType::STRING => Ok(LiteralValue::STRING(token.lexeme.clone())),
            TokenType::NUMBER => Ok(LiteralValue::NUMBER(token.lexeme.parse::<f64>().unwrap())),
            TokenType::NIL => Ok(LiteralValue::NIL),
            TokenType::TRUE => Ok(LiteralValue::BOOLEAN(true)),
            TokenType::FALSE => Ok(LiteralValue::BOOLEAN(false)),
            _ => Err(String::from("not a literal value")),
        }
    }

    pub fn unwrap_as_boolean(&self) -> LiteralValue {
        use LiteralValue::*;
        match self {
            NIL => LiteralValue::BOOLEAN(false),
            BOOLEAN(b) => LiteralValue::BOOLEAN(*b),
            NUMBER(n) => LiteralValue::BOOLEAN(*n != 0.0f64),
            STRING(s) => LiteralValue::BOOLEAN(!s.is_empty()),
            _ => panic!("can not unwrap callable"),
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            LiteralValue::NIL => false,
            LiteralValue::BOOLEAN(b) => *b,
            LiteralValue::NUMBER(n) => *n != 0.0f64,
            LiteralValue::STRING(s) => !s.is_empty(),
            _ => {
                panic!("can not unwrap callable")
            }
        }
    }
}

impl PartialEq for LiteralValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LiteralValue::STRING(s1), LiteralValue::STRING(s2)) => s1 == s2,
            (LiteralValue::NUMBER(n1), LiteralValue::NUMBER(n2)) => n1 == n2,
            (LiteralValue::BOOLEAN(b1), LiteralValue::BOOLEAN(b2)) => b1 == b2,
            (LiteralValue::NIL, LiteralValue::NIL) => true,
            (
                LiteralValue::Callable {
                    name: n1,
                    arity: a1,
                    func: _,
                },
                LiteralValue::Callable {
                    name: n2,
                    arity: a2,
                    func: _,
                },
            ) => n1 == n2 && a1 == a2,
            _ => false,
        }
    }
}
#[allow(clippy::inherent_to_string)]
impl LiteralValue {
    pub fn to_string(&self) -> String {
        match self {
            LiteralValue::NUMBER(n) => n.to_string(),
            LiteralValue::STRING(s) => s.to_string(),
            LiteralValue::BOOLEAN(b) => b.to_string(),
            LiteralValue::NIL => "nil".to_string(),
            LiteralValue::Callable {
                name,
                arity,
                func: _,
            } => {
                format!("Callable({} {})", name, arity)
            }
        }
    }
}
#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<LiteralValue>,
    pub line_number: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<LiteralValue>,
        line_number: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line_number,
        }
    }
}
