#[allow(dead_code)]
#[warn(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: u64,
    current: u64,
    line: usize,
}
impl<'a> Scanner<'a> {
    pub fn new(_source: &'a str) -> Self {
        Self {
            source: _source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, String> {
        let mut errs: Vec<String> = vec![];
        while !self.is_at_end() {
            self.start = self.current;
            // self.clone().scan_token().unwrap();
            match self.scan_token() {
                Ok(_) => (),
                Err(e) => errs.push(e),
            }
        }
        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: String::from(""),
            literal: None,
            line_number: self.line,
        });
        if errs.len() > 0 {
            return Err(errs.join("\n"));
        }
        Ok(self.tokens.clone())
    }
    pub fn scan_token(&mut self) -> Result<(), String> {
        let c = self.advance();
        dbg!("scan_token", c);
        match c {
            '(' => Ok(self.add_token(TokenType::LEFT_PAREN)),
            ')' => Ok(self.add_token(TokenType::RIGHT_PAREN)),
            '{' => Ok(self.add_token(TokenType::LEFT_BRACE)),
            '}' => Ok(self.add_token(TokenType::RIGHT_BRACE)),
            ',' => Ok(self.add_token(TokenType::COMMA)),
            '.' => Ok(self.add_token(TokenType::DOT)),
            '-' => Ok(self.add_token(TokenType::MINUS)),
            '+' => Ok(self.add_token(TokenType::PLUS)),
            ';' => Ok(self.add_token(TokenType::SEMICOLON)),
            '*' => Ok(self.add_token(TokenType::STAR)),
            // '/' => {
            //     loop {
            //         if self.is_at_end() || self.peek() == '\n' {
            //             self.line += 1;
            //             break;
            //         }
            //         self.advance();
            //     }
            //     Ok(())
            // }
            ' ' | '\r' | '\t' => Ok(()),
            '!' => {
                if self.match_char('=') {
                    Ok(self.add_token(TokenType::BANG_EQUAL))
                } else {
                    Ok(self.add_token(TokenType::BANG))
                }
            }
            '\n' => {
                self.line += 1;
                Ok(())
            }
            '=' => {
                if self.match_char('=') {
                    Ok(self.add_token(TokenType::EqualEqual))
                } else {
                    Ok(self.add_token(TokenType::EQUAL))
                }
            }
            '<' => {
                if self.match_char('=') {
                    Ok(self.add_token(TokenType::LESS_EQUAL))
                } else {
                    Ok(self.add_token(TokenType::LESS))
                }
            }
            '>' => {
                if self.match_char('=') {
                    Ok(self.add_token(TokenType::GREATER_EQUAL))
                } else {
                    Ok(self.add_token(TokenType::GREATER))
                }
            }
            _ => Err(format!("Unexpected character: {}", c)),
        }
    }

    fn peek(self: &mut Self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current as usize).unwrap()
    }
    fn is_at_end(&mut self) -> bool {
        self.current >= self.source.len() as u64
    }
    fn match_char(&mut self, c: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current as usize).unwrap() != c {
            return false;
        }
        self.current += 1;
        true
    }
    fn advance(self: &mut Self) -> char {
        let c = self.source.chars().nth(self.current as usize).unwrap();
        self.current += 1;
        c as char
    }
    fn add_token(self: &mut Self, token_type: TokenType) {
        self.add_token_lit(token_type, None);
    }
    fn add_token_lit(self: &mut Self, token_type: TokenType, literal: Option<LiteralValue>) {
        let text: String = String::from_utf8_lossy(
            &self.source.as_bytes()[self.start as usize..self.current as usize],
        )
        .to_string();
        self.tokens.push(Token {
            token_type,
            lexeme: text,
            literal,
            line_number: self.line,
        });
    }
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,  // (
    RIGHT_PAREN, // )
    LEFT_BRACE,  // {
    RIGHT_BRACE, // }
    COMMA,       // ,
    DOT,         // .
    MINUS,       // -
    PLUS,        // +
    SEMICOLON,   // ;
    SLASH,       // /
    STAR,        // *
    // one or two character tokens,
    BANG,          // !
    BANG_EQUAL,    // !=
    EQUAL,         // =
    EqualEqual,    // ==
    GREATER,       // >
    GREATER_EQUAL, // >=
    LESS,          // <
    LESS_EQUAL,    // <=
    // Literals.
    IDENTIFIER, // identifier
    STRING,     // string
    NUMBER,     // number

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

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    STRING(String),
    NUMBER(f64),
    BOOLEAN(bool),
    NIL,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    lexeme: String,
    literal: Option<LiteralValue>,
    line_number: usize,
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
    pub fn to_string(&self) -> String {
        format!("{:?} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
