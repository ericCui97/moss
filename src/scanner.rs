use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        [
            ("and", TokenType::AND),
            ("class", TokenType::CLASS),
            ("else", TokenType::ELSE),
            ("false", TokenType::FALSE),
            ("fun", TokenType::FUN),
            ("for", TokenType::FOR),
            ("if", TokenType::IF),
            ("nil", TokenType::NIL),
            ("or", TokenType::OR),
            ("print", TokenType::PRINT),
            ("return", TokenType::RETURN),
            ("super", TokenType::SUPER),
            ("this", TokenType::THIS),
            ("true", TokenType::TRUE),
            ("var", TokenType::VAR),
            ("while", TokenType::WHILE),
            // 添加其他映射关系...
        ]
        .iter()
        .cloned()
        .collect()
    };
}

#[warn(non_camel_case_types)]
#[warn(dead_code)]
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
        if !errs.is_empty() {
            return Err(errs.join("\n"));
        }
        Ok(self.tokens.clone())
    }
    pub fn scan_token(&mut self) -> Result<(), String> {
        let c = self.advance();
        // dbg!("scan_token", c);
        match c {
            '(' => {
                self.add_token(TokenType::LEFT_PAREN);
                Ok(())
            }
            ')' => {
                self.add_token(TokenType::RIGHT_PAREN);
                Ok(())
            }
            '{' => {
                self.add_token(TokenType::LEFT_BRACE);
                Ok(())
            }
            '}' => {
                self.add_token(TokenType::RIGHT_BRACE);
                Ok(())
            }
            ',' => {
                self.add_token(TokenType::COMMA);
                Ok(())
            }
            '.' => {
                self.add_token(TokenType::DOT);
                Ok(())
            }
            '-' => {
                self.add_token(TokenType::MINUS);
                Ok(())
            }
            '+' => {
                self.add_token(TokenType::PLUS);
                Ok(())
            }
            ';' => {
                self.add_token(TokenType::SEMICOLON);
                Ok(())
            }
            '*' => {
                self.add_token(TokenType::STAR);
                Ok(())
            }
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
                    self.add_token(TokenType::BANG_EQUAL);
                    Ok(())
                } else {
                    self.add_token(TokenType::BANG);
                    Ok(())
                }
            }
            // comments 注释
            '/' => {
                if self.match_char('/') {
                    loop {
                        if self.peek() == '\n' || self.is_at_end() {
                            break;
                        }
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH)
                }
                Ok(())
            }
            '\n' => {
                self.line += 1;
                Ok(())
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EQUAL_EQUAL);
                    Ok(())
                } else {
                    self.add_token(TokenType::EQUAL);
                    Ok(())
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LESS_EQUAL);
                    Ok(())
                } else {
                    self.add_token(TokenType::LESS);
                    Ok(())
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GREATER_EQUAL);
                    Ok(())
                } else {
                    self.add_token(TokenType::GREATER);
                    Ok(())
                }
            }
            '"' => self.string(),
            c => {
                if is_digit(c) {
                    self.number()
                } else if is_alpha(c) {
                    self.identifier()
                } else {
                    Err(format!("Unexpected character: {}", c))
                }
            }
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current as usize).unwrap()
    }

    fn is_at_end(&self) -> bool {
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
    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current as usize).unwrap();
        self.current += 1;
        c
    }
    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_lit(token_type, None);
    }
    fn add_token_lit(&mut self, token_type: TokenType, literal: Option<LiteralValue>) {
        let text: String = String::from_utf8_lossy(
            &self.source.as_bytes()[self.start as usize..self.current as usize],
        )
        .to_string();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }
    fn string(&mut self) -> Result<(), String> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        // empty string
        // if self.peek() == '"' {
        //     dbg!("empty string");
        //     self.advance();
        //     self.add_token_lit(
        //         TokenType::STRING,
        //         Some(LiteralValue::STRING("".to_string())),
        //     );
        //     return Ok(());
        // }
        if self.is_at_end() {
            return Err("undeterminded string".to_string());
        }
        self.advance();
        //		let value = self.source.as_bytes()[self.start+1..self.current];
        let value = String::from_utf8_lossy(
            &self.source.as_bytes()[self.start as usize + 1..self.current as usize - 1],
        );
        self.add_token_lit(
            TokenType::STRING,
            Some(LiteralValue::STRING(value.to_string())),
        );
        Ok(())
    }

    fn number(&mut self) -> Result<(), String> {
        while is_digit(self.peek()) {
            self.advance();
        }
        while self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();
            while is_digit(self.peek()) {
                self.advance();
            }
        }
        let sub_string = String::from_utf8_lossy(
            &self.source.as_bytes()[self.start as usize..self.current as usize],
        );
        match sub_string.parse::<f64>() {
            Ok(v) => {
                self.add_token_lit(TokenType::NUMBER, Some(LiteralValue::NUMBER(v)));
                Ok(())
            }
            Err(e) => Err(format!("parse number error: {}", e)),
        }
    }
    fn identifier(&mut self) -> Result<(), String> {
        while is_alpha_numeric(self.peek()) {
            self.advance();
        }
        let sub_string = String::from_utf8_lossy(
            &self.source.as_bytes()[self.start as usize..self.current as usize],
        );
        let mut token_type = TokenType::IDENTIFIER;
        // match KEYWORDS.get(sub_string.as_ref()) {
        //     Some(keyword_type) => token_type = *keyword_type,
        //     None => (),
        // }
        if let Some(keyword_type) = KEYWORDS.get(sub_string.as_ref()) {
            token_type = *keyword_type
        }
        self.add_token(token_type);

        Ok(())
    }
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() as u64 {
            return '\0';
        }
        self.source
            .chars()
            .nth((self.current + 1) as usize)
            .unwrap()
    }
}

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
    // !
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
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    STRING(String),
    NUMBER(f64),
    //    BOOLEAN(bool),
    NIL,
    BOOLEAN(bool),
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
        match self {
            LiteralValue::NIL => LiteralValue::BOOLEAN(false),
            LiteralValue::BOOLEAN(b) => LiteralValue::BOOLEAN(*b),
            LiteralValue::NUMBER(n) => LiteralValue::BOOLEAN(*n != 0.0f64),
            LiteralValue::STRING(s) => LiteralValue::BOOLEAN(!s.is_empty()),
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            LiteralValue::NIL => false,
            LiteralValue::BOOLEAN(b) => *b,
            LiteralValue::NUMBER(n) => *n != 0.0f64,
            LiteralValue::STRING(s) => !s.is_empty(),
        }
    }
}

// impl Copy for LiteralValue {
//     fn copy(&self) -> Self {
//         match self {
//             LiteralValue::NUMBER(n) => LiteralValue::NUMBER(*n),
//             LiteralValue::STRING(s) => LiteralValue::STRING(s.clone()),
//             LiteralValue::BOOLEAN(b) => LiteralValue::BOOLEAN(*b),
//             LiteralValue::NIL => LiteralValue::NIL,
//         }
//     }
// }

#[allow(clippy::inherent_to_string)]
impl LiteralValue {
    pub fn to_string(&self) -> String {
        match self {
            LiteralValue::NUMBER(n) => n.to_string(),
            LiteralValue::STRING(s) => s.to_string(),
            LiteralValue::BOOLEAN(b) => b.to_string(),
            LiteralValue::NIL => "nil".to_string(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
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

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn is_alpha(c: char) -> bool {
    c.is_ascii_lowercase() || c.is_ascii_uppercase() || c == '_'
}

fn is_alpha_numeric(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}

/*-----------------test-----------------*/
// use moss::scanner::Scanner;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_one_char_tokens() {
        let mut scanner = Scanner::new("(){};+-*");
        let tokens = scanner.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 9);
        assert_eq!(tokens[0].token_type, TokenType::LEFT_PAREN);
        assert_eq!(tokens[1].token_type, TokenType::RIGHT_PAREN);
        assert_eq!(tokens[2].token_type, TokenType::LEFT_BRACE);
        assert_eq!(tokens[3].token_type, TokenType::RIGHT_BRACE);
        assert_eq!(tokens[4].token_type, TokenType::SEMICOLON);
        assert_eq!(tokens[5].token_type, TokenType::PLUS);
        assert_eq!(tokens[6].token_type, TokenType::MINUS);
        assert_eq!(tokens[7].token_type, TokenType::STAR);
        assert_eq!(tokens[8].token_type, TokenType::EOF);
    }
    #[test]
    fn test1() {
        let mut scanner = Scanner::new("(( ))");
        let tokens = scanner.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].token_type, TokenType::LEFT_PAREN);
        assert_eq!(tokens[1].token_type, TokenType::LEFT_PAREN);
        assert_eq!(tokens[2].token_type, TokenType::RIGHT_PAREN);
        assert_eq!(tokens[3].token_type, TokenType::RIGHT_PAREN);
        assert_eq!(tokens[4].token_type, TokenType::EOF);
    }
    #[test]
    fn test2char() {
        let mut scanner = Scanner::new("!= == <= >= ");
        let tokens = scanner.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].token_type, TokenType::BANG_EQUAL);
        assert_eq!(tokens[1].token_type, TokenType::EQUAL_EQUAL);
        assert_eq!(tokens[2].token_type, TokenType::LESS_EQUAL);
        assert_eq!(tokens[3].token_type, TokenType::GREATER_EQUAL);
        assert_eq!(tokens[4].token_type, TokenType::EOF);
    }
    #[test]
    fn test_string() {
        let mut scanner = Scanner::new("\"hello world\"");
        let tokens = scanner.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::STRING);
        assert_eq!(tokens[0].lexeme, "\"hello world\"");
        assert_eq!(tokens[1].token_type, TokenType::EOF);
    }
    #[test]
    fn test_string_err() {
        let mut scanner = Scanner::new("\"hello world");
        let tokens = scanner.scan_tokens();
        assert!(tokens.is_err());
    }
    #[test]
    fn test_string2() {
        let mut scanner = Scanner::new(r#""hello world""#);
        let tokens = scanner.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::STRING);
        assert_eq!(tokens[0].lexeme, "\"hello world\"");
        assert_eq!(tokens[1].token_type, TokenType::EOF);
        match tokens[0].literal.as_ref().unwrap() {
            LiteralValue::STRING(s) => assert_eq!(s, "hello world"),
            _ => panic!("literal is not string"),
        }
    }
    #[test]
    fn test_number() {
        let mut scanner = Scanner::new("123 123.456 0.1");
        let tokens = scanner.scan_tokens().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::NUMBER);
        assert_eq!(tokens[0].lexeme, "123");
        assert_eq!(tokens[0].literal, Some(LiteralValue::NUMBER(123.0)));

        assert_eq!(tokens[1].token_type, TokenType::NUMBER);
        assert_eq!(tokens[1].lexeme, "123.456");
        assert_eq!(tokens[2].token_type, TokenType::NUMBER);
        assert_eq!(tokens[2].lexeme, "0.1");
        assert_eq!(tokens[3].token_type, TokenType::EOF);
        match tokens[0].literal.as_ref().unwrap() {
            LiteralValue::NUMBER(n) => assert_eq!(*n, 123.0),
            _ => panic!("literal is not number"),
        }
    }

    #[test]
    fn test_identifier() {
        let mut scanner = Scanner::new(r#"var_a = "hello world";"#);
        let tokens = scanner.scan_tokens().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::IDENTIFIER);
        assert_eq!(tokens[0].lexeme, "var_a");
        assert_eq!(tokens[1].token_type, TokenType::EQUAL);
        assert_eq!(tokens[2].token_type, TokenType::STRING);
        assert_eq!(tokens[2].lexeme, "\"hello world\"");
        assert_eq!(tokens[3].token_type, TokenType::SEMICOLON);
        assert_eq!(tokens[4].token_type, TokenType::EOF);
    }
    #[test]
    fn test_identifier_hold() {
        let mut scanner = Scanner::new(
            r#"
        var a = 1 if(a==1) or nil return
        "#,
        );
        let tokens = scanner.scan_tokens().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::VAR);
        assert_eq!(tokens[1].token_type, TokenType::IDENTIFIER);
        assert_eq!(tokens[2].token_type, TokenType::EQUAL);
        assert_eq!(tokens[3].token_type, TokenType::NUMBER);
        assert_eq!(tokens[4].token_type, TokenType::IF);
        assert_eq!(tokens[5].token_type, TokenType::LEFT_PAREN);
        assert_eq!(tokens[6].token_type, TokenType::IDENTIFIER);
        assert_eq!(tokens[7].token_type, TokenType::EQUAL_EQUAL);
        assert_eq!(tokens[8].token_type, TokenType::NUMBER);
        assert_eq!(tokens[9].token_type, TokenType::RIGHT_PAREN);
        assert_eq!(tokens[10].token_type, TokenType::OR);
        assert_eq!(tokens[11].token_type, TokenType::NIL);
        assert_eq!(tokens[12].token_type, TokenType::RETURN);
        assert_eq!(tokens[13].token_type, TokenType::EOF);
    }
    #[test]
    fn test_all_keywords() {
        let mut scanner = Scanner::new(
            r#"
        and class else false fun for if nil or print return super this true var while
        "#,
        );
        let tokens = scanner.scan_tokens().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::AND);
        assert_eq!(tokens[1].token_type, TokenType::CLASS);
        assert_eq!(tokens[2].token_type, TokenType::ELSE);
        assert_eq!(tokens[3].token_type, TokenType::FALSE);
        assert_eq!(tokens[4].token_type, TokenType::FUN);
        assert_eq!(tokens[5].token_type, TokenType::FOR);
        assert_eq!(tokens[6].token_type, TokenType::IF);
        assert_eq!(tokens[7].token_type, TokenType::NIL);
        assert_eq!(tokens[8].token_type, TokenType::OR);
        assert_eq!(tokens[9].token_type, TokenType::PRINT);
        assert_eq!(tokens[10].token_type, TokenType::RETURN);
        assert_eq!(tokens[11].token_type, TokenType::SUPER);
        assert_eq!(tokens[12].token_type, TokenType::THIS);
        assert_eq!(tokens[13].token_type, TokenType::TRUE);
        assert_eq!(tokens[14].token_type, TokenType::VAR);
        assert_eq!(tokens[15].token_type, TokenType::WHILE);
        assert_eq!(tokens[16].token_type, TokenType::EOF);
    }
}
