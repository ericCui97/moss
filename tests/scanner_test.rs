// use moss::scanner::Scanner;

#[cfg(test)]
mod tests {
    use moss::scanner::*;

    #[test]
    fn handle_one_char_tokens() {
        let mut scanner = Scanner::new("(){};+-*");
        let tokens = scanner.scan_tokens().unwrap();
        assert_eq!(tokens.len(),9);
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
    fn test1(){
        let mut scanner = Scanner::new("(( ))");
        let tokens = scanner.scan_tokens().unwrap();
        assert_eq!(tokens.len(),5);
        assert_eq!(tokens[0].token_type, TokenType::LEFT_PAREN);
        assert_eq!(tokens[1].token_type, TokenType::LEFT_PAREN);
        assert_eq!(tokens[2].token_type, TokenType::RIGHT_PAREN);
        assert_eq!(tokens[3].token_type, TokenType::RIGHT_PAREN);
        assert_eq!(tokens[4].token_type, TokenType::EOF);
    }
    #[test]
    fn test2char(){
        let mut scanner = Scanner::new("!= == <= >= ");
        let tokens = scanner.scan_tokens().unwrap();
        assert_eq!(tokens.len(),5);
        assert_eq!(tokens[0].token_type, TokenType::BANG_EQUAL);
        assert_eq!(tokens[1].token_type, TokenType::EqualEqual);
        assert_eq!(tokens[2].token_type, TokenType::LESS_EQUAL);
        assert_eq!(tokens[3].token_type, TokenType::GREATER_EQUAL);
        assert_eq!(tokens[4].token_type, TokenType::EOF);
    }
}
