use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
    Whitespace,
    EOF,
}

pub struct Lexer<'a> {
    chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            chars: input.chars(),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token() {
            if (token != Token::Whitespace) && (token != Token::EOF) {
                tokens.push(token);
            }
        }
        tokens
    }

    fn next_token(&mut self) -> Option<Token> {
        let next_char = self.chars.next()?;
        match next_char {
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Multiply),
            '/' => Some(Token::Divide),
            '0'..='9' => {
                let mut number = next_char.to_digit(10)? as i32;
                while let Some(next_char) = self.chars.clone().next() {
                    if let Some(digit) = next_char.to_digit(10) {
                        number = number * 10 + digit as i32;
                        self.chars.next();
                    } else {
                        break;
                    }
                }
                Some(Token::Number(number))
            }
            ' ' => Some(Token::Whitespace),
            _ => Some(Token::EOF),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tokenize_number() {
        let mut lexer = Lexer::new("123");
        assert_eq!(lexer.tokenize(), vec![Token::Number(123)]);
    }

    #[test]
    fn tokenize_number_with_multiple_digits() {
        let mut lexer = Lexer::new("12345");
        assert_eq!(lexer.tokenize(), vec![Token::Number(12345)]);
    }

    #[test]
    fn tokenize_number_with_multiple_digits_and_operator() {
        let mut lexer = Lexer::new("123+45");
        assert_eq!(
            lexer.tokenize(),
            vec![Token::Number(123), Token::Plus, Token::Number(45)]
        );
    }
    #[test]
    fn tokenize_expression() {
        let mut lexer = Lexer::new("1 + 2 * 3 - 4 / 5");
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Number(1),
                Token::Plus,
                Token::Number(2),
                Token::Multiply,
                Token::Number(3),
                Token::Minus,
                Token::Number(4),
                Token::Divide,
                Token::Number(5)
            ]
        );
    }
}
