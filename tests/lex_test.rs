#[cfg(test)]
mod test_lex {
    use std::fs::read_to_string;
    use std::{fs, path};
    use tinylang_rs::lexer::lex::Lexer;
    use tinylang_rs::lexer::token::{Token, TokenType};
    #[test]
    fn test_lex() -> Result<(), std::io::Error> {
        let path = path::Path::new("test.txt");
        let file = read_to_string(path)?;
        let mut lexer = Lexer::new(&file);

        let list = lexer.lex();
        assert_eq!(list.len(), 17);

        let mut iter = list.into_iter();
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::NUM, "123421".to_string(), 1, 1))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::ID, "what".to_string(), 1, 8))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(
                TokenType::COMMENT,
                "/* \r\nis \r\n*/"
                .to_string(),
                1,
                13
            ))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::ID, "test".to_string(), 4, 1))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::ID, "test".to_string(), 5, 1))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::VOID, "void".to_string(), 6, 1))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::IF, "if".to_string(), 6, 6))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::NUM, "11222".to_string(), 6, 9))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::WHILE, "while".to_string(), 7, 1))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::ID, "a".to_string(), 8, 1))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::MULTIPLY, "*".to_string(), 8, 3))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::NUM, "3".to_string(), 8, 5))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::INT, "int".to_string(), 9, 1))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::ID, "a".to_string(), 9, 5))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::ASSIGN, "=".to_string(), 9, 7))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::NUM, "3".to_string(), 9, 9))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::SEMI, ";".to_string(), 9, 10))
        );
        Ok(())
    }
}
