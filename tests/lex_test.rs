#[cfg(test)]
mod test_lex {
    use std::fs::read_to_string;
    use std::io::Error;
    use std::{fs, path};
    use tinylang_rs::lexer::lex::Lexer;
    use tinylang_rs::lexer::token::{Token, TokenType};
    #[test]
    fn test_lex_meaningless() -> Result<(), Error> {
        let path = path::Path::new("tests/fixtures/lexer.test.txt");
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
                "/* \nis \n*/".to_string(),
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
    #[test]
    fn test_lex_gcd() -> Result<(), Error> {
        let path = path::Path::new("tests/fixtures/lexer2.test.txt");
        let file = read_to_string(path)?;
        let mut lexer = Lexer::new(&file);

        let list = lexer.lex();
        assert_eq!(list.len(), 72);

        let mut iter = list.into_iter();
        assert_eq!(
            iter.next(),
            Some(Token::new(
                TokenType::COMMENT,
                "/* A program to perform Euclid\'s Algorithm to compute gcd. */".to_string(),
                1,
                1
            ))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::INT, "int".to_string(), 2, 1))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::ID, "gcd".to_string(), 2, 5))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::LPAREN, "(".to_string(), 2, 9))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::INT, "int".to_string(), 2, 10))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::ID, "u".to_string(), 2, 14))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::COMMA, ",".to_string(), 2, 15))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::INT, "int".to_string(), 2, 17))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::ID, "v".to_string(), 2, 21))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::RPAREN, ")".to_string(), 2, 22))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::LBRACE, "{".to_string(), 3, 1))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::IF, "if".to_string(), 4, 5))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::LPAREN, "(".to_string(), 4, 8))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::ID, "v".to_string(), 4, 9))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::EQ, "==".to_string(), 4, 11))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::NUM, "0".to_string(), 4, 14))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::RPAREN, ")".to_string(), 4, 15))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::RETURN, "return".to_string(), 4, 17))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::ID, "u".to_string(), 4, 24))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::SEMI, ";".to_string(), 4, 25))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::ELSE, "else".to_string(), 5, 5))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::RETURN, "return".to_string(), 5, 10))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::ID, "gcd".to_string(), 5, 17))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::LPAREN, "(".to_string(), 5, 20))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::ID, "v".to_string(), 5, 21))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::COMMA, ",".to_string(), 5, 22))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::ID, "u".to_string(), 5, 23))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::MINUS, "-".to_string(), 5, 24))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::ID, "u".to_string(), 5, 25))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::TIMES, "/".to_string(), 5, 26))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::ID, "v".to_string(), 5, 27))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::MULTIPLY, "*".to_string(), 5, 28))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::ID, "v".to_string(), 5, 29))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::RPAREN, ")".to_string(), 5, 30))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::SEMI, ";".to_string(), 5, 31))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(
                TokenType::COMMENT,
                "/* u-u/v*v == u mod v\n      that\n      something\n    */"
                .to_string(),
                6,
                5
            ))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::RBRACE, "}".to_string(), 10, 1))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::VOID, "void".to_string(), 12, 1))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::ID, "main".to_string(), 12, 6))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::LPAREN, "(".to_string(), 12, 10))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::VOID, "void".to_string(), 12, 11))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::RPAREN, ")".to_string(), 12, 15))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::LBRACE, "{".to_string(), 12, 17))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::INT, "int".to_string(), 13, 5))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::ID, "x".to_string(), 13, 9))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::SEMI, ";".to_string(), 13, 10))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::INT, "int".to_string(), 14, 5))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::ID, "y".to_string(), 14, 9))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::SEMI, ";".to_string(), 14, 10))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::ID, "x".to_string(), 15, 5))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::ASSIGN, "=".to_string(), 15, 7))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::ID, "input".to_string(), 15, 9))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::LPAREN, "(".to_string(), 15, 14))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::RPAREN, ")".to_string(), 15, 15))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::SEMI, ";".to_string(), 15, 16))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::ID, "y".to_string(), 16, 5))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::ASSIGN, "=".to_string(), 16, 7))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::ID, "input".to_string(), 16, 9))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::LPAREN, "(".to_string(), 16, 14))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::RPAREN, ")".to_string(), 16, 15))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::SEMI, ";".to_string(), 16, 16))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::ID, "output".to_string(), 17, 5))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::LPAREN, "(".to_string(), 17, 11))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::ID, "gcd".to_string(), 17, 12))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::LPAREN, "(".to_string(), 17, 15))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::ID, "x".to_string(), 17, 16))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::COMMA, ",".to_string(), 17, 17))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::ID, "y".to_string(), 17, 18))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::RPAREN, ")".to_string(), 17, 19))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::RPAREN, ")".to_string(), 17, 20))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::SEMI, ";".to_string(), 17, 21))
        );
        assert_eq!(
            iter.next(),
            Some(Token::new(TokenType::RBRACE, "}".to_string(), 18, 1))
        );
        Ok(())
    }
}
