use std::{str::Chars, option};
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token {
    /// Token Type
    pub kind: Kind,

    /// Start offset in source
    pub start: usize,

    /// End offset in source
    pub end: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Kind {
    Eof, // end of file
    Plus,
    If,
    While,
    For,
    Else,
}

struct Lexer<'a> {
    // source code
    source: &'a str,
    // remaining source code
    chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars(),
        }
    }

    pub fn read_next_kind(&mut self) -> Kind {
        while let Some(c) = self.chars.next() {
            match c {
                '+' => return Kind::Plus,
                _ => {}
            }
        }
        Kind::Eof
    }

    fn read_next_token(&mut self) -> Token {
        let start = self.offset();
        let kind = self.read_next_kind();
        let end = self.offset();
        Token { kind, start, end }
    }

    fn offset(&self) -> usize {
        self.source.len() - self.chars.as_str().len()
    }

    fn peek(&self)->Option<char>{
        self.chars.clone().next()
    }
}
