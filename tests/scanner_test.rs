// use moss::scanner::Scanner;

#[cfg(test)]
mod tests {
    use moss::scanner::*;

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
    // #[test]
    // fn test_all_tokens(){
    //     let mut scanner = Scanner::new("{} !//,.*");
    //     let tokens = scanner.scan_tokens().unwrap();
    //     assert_eq!(tokens.len(),9);
    //     assert_eq!(tokens[0].token_type, TokenType::LEFT_BRACE);
    //     assert_eq!(tokens[1].token_type, TokenType::RIGHT_BRACE);
    //     assert_eq!(tokens[2].token_type, TokenType::BANG);
    //     assert_eq!(tokens[3].token_type, TokenType::SLASH);
    //     assert_eq!(tokens[4].token_type, TokenType::SLASH);
    //     assert_eq!(tokens[5].token_type, TokenType::COMMA);
    //     assert_eq!(tokens[6].token_type, TokenType::DOT);
    //     assert_eq!(tokens[7].token_type, TokenType::STAR);
    //     assert_eq!(tokens[8].token_type, TokenType::EOF);

    // }
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
        assert_eq!(tokens.is_err(), true);
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
