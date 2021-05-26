use tinylang_rs::lexer::token::{Position, Token, TokenType};

fn lex_test_helper(
    source_file: &String,
    actual_token: Option<Token>,
    token_type: TokenType,
    start_position: Position,
    end_position: Position,
) {
    assert!(actual_token.is_some(), "expected {:?}", token_type);
    let actual_token = actual_token.unwrap();
    assert_eq!(actual_token.token_type, token_type);

    let (start, end) = (actual_token.start_index, actual_token.end_index);
    assert_eq!(actual_token.start_position, start_position);
    assert_eq!(actual_token.end_position, end_position);
    assert_eq!(source_file[start..end], actual_token.content);
}
#[cfg(test)]
mod test_lex {
    use crate::lex_test_helper;
    use std::fs::read_to_string;
    use std::path;
    use tinylang_rs::lexer::token::{Position, TokenType};
    use tinylang_rs::lexer::{lex::Lexer, token::KeywordType};
    #[test]
    fn test_lex_meaningless() -> Result<(), std::io::Error> {
        let path = path::Path::new("tests/fixtures/lexer/test.txt");
        let file = read_to_string(path)?;
        let mut lexer = Lexer::new(&file);

        let list = lexer.lex();
        assert_eq!(list.len(), 43);

        let mut iter = list.into_iter();
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::NumberLiteral,
            Position::new(0, 0),
            Position::new(0, 6),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::Id,
            Position::new(0, 7),
            Position::new(0, 11),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::Comment,
            Position::new(0, 12),
            Position::new(2, 2),
        );

        lex_test_helper(
            &file,
            iter.next(),
            TokenType::Id,
            Position::new(3, 0),
            Position::new(3, 4),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::Id,
            Position::new(4, 0),
            Position::new(4, 4),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::Keyword(KeywordType::VOID),
            Position::new(5, 0),
            Position::new(5, 4),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::Keyword(KeywordType::IF),
            Position::new(5, 5),
            Position::new(5, 7),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::NumberLiteral,
            Position::new(5, 8),
            Position::new(5, 13),
        );

        lex_test_helper(
            &file,
            iter.next(),
            TokenType::Keyword(KeywordType::WHILE),
            Position::new(6, 0),
            Position::new(6, 5),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::Id,
            Position::new(7, 0),
            Position::new(7, 1),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::Multiply,
            Position::new(7, 2),
            Position::new(7, 3),
        );

        lex_test_helper(
            &file,
            iter.next(),
            TokenType::NumberLiteral,
            Position::new(7, 4),
            Position::new(7, 5),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::Keyword(KeywordType::INT),
            Position::new(8, 0),
            Position::new(8, 3),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::Id,
            Position::new(8, 4),
            Position::new(8, 5),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::Assign,
            Position::new(8, 6),
            Position::new(8, 7),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::NumberLiteral,
            Position::new(8, 8),
            Position::new(8, 9),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::Semi,
            Position::new(8, 9),
            Position::new(8, 10),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::Id,
            Position::new(9, 0),
            Position::new(9, 1),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::Eq,
            Position::new(9, 2),
            Position::new(9, 4),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::NumberLiteral,
            Position::new(9, 5),
            Position::new(9, 6),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::Lparen,
            Position::new(10, 0),
            Position::new(10, 1),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::Rparen,
            Position::new(10, 1),
            Position::new(10, 2),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::Lbrack,
            Position::new(10, 2),
            Position::new(10, 3),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::Rbrack,
            Position::new(10, 3),
            Position::new(10, 4),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::Lbrace,
            Position::new(10, 4),
            Position::new(10, 5),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::Rbrace,
            Position::new(10, 5),
            Position::new(10, 6),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::Keyword(KeywordType::BOOL),
            Position::new(11, 0),
            Position::new(11, 4),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::Id,
            Position::new(11, 5),
            Position::new(11, 6),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::Semi,
            Position::new(11, 6),
            Position::new(11, 7),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::Id,
            Position::new(12, 0),
            Position::new(12, 1),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::Assign,
            Position::new(12, 2),
            Position::new(12, 3),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::BooleanLiteral,
            Position::new(12, 4),
            Position::new(12, 8),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::Semi,
            Position::new(12, 8),
            Position::new(12, 9),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::Id,
            Position::new(13, 0),
            Position::new(13, 1),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::Assign,
            Position::new(13, 2),
            Position::new(13, 3),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::BooleanLiteral,
            Position::new(13, 4),
            Position::new(13, 9),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::Semi,
            Position::new(13, 9),
            Position::new(13, 10),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::Id,
            Position::new(14, 0),
            Position::new(14, 1),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::And,
            Position::new(14, 2),
            Position::new(14, 4),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::Id,
            Position::new(14, 5),
            Position::new(14, 6),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::Id,
            Position::new(15, 0),
            Position::new(15, 1),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::Or,
            Position::new(15, 2),
            Position::new(15, 4),
        );
        lex_test_helper(
            &file,
            iter.next(),
            TokenType::Id,
            Position::new(15, 5),
            Position::new(15, 6),
        );
        Ok(())
    }
    // #[test]
    // fn test_lex_gcd() -> Result<(), std::io::Error> {
    //     let path = path::Path::new("tests/fixtures/lexer2.test.txt");
    //     let file = read_to_string(path)?;
    //     let mut lexer = Lexer::new(&file);

    //     let list = lexer.lex();
    //     assert_eq!(list.len(), 73);

    //     let mut iter = list.into_iter();
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(
    //             TokenType::COMMENT,
    //             "/* A program to perform Euclid\'s Algorithm to compute gcd. */".to_string(),
    //             1,
    //             1
    //         ))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::INT, "int".to_string(), 2, 1))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::ID, "gcd".to_string(), 2, 5))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::LPAREN, "(".to_string(), 2, 9))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::INT, "int".to_string(), 2, 10))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::ID, "u".to_string(), 2, 14))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::COMMA, ",".to_string(), 2, 15))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::INT, "int".to_string(), 2, 17))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::ID, "v".to_string(), 2, 21))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::RPAREN, ")".to_string(), 2, 22))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::LBRACE, "{".to_string(), 3, 1))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::IF, "if".to_string(), 4, 5))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::LPAREN, "(".to_string(), 4, 8))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::ID, "v".to_string(), 4, 9))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::EQ, "==".to_string(), 4, 11))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::NUM, "0".to_string(), 4, 14))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::RPAREN, ")".to_string(), 4, 15))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::RETURN, "return".to_string(), 4, 17))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::ID, "u".to_string(), 4, 24))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::SEMI, ";".to_string(), 4, 25))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::ELSE, "else".to_string(), 5, 5))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::RETURN, "return".to_string(), 5, 10))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::ID, "gcd".to_string(), 5, 17))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::LPAREN, "(".to_string(), 5, 20))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::ID, "v".to_string(), 5, 21))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::COMMA, ",".to_string(), 5, 22))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::ID, "u".to_string(), 5, 23))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::MINUS, "-".to_string(), 5, 24))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::ID, "u".to_string(), 5, 25))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::TIMES, "/".to_string(), 5, 26))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::ID, "v".to_string(), 5, 27))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::MULTIPLY, "*".to_string(), 5, 28))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::ID, "v".to_string(), 5, 29))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::RPAREN, ")".to_string(), 5, 30))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::SEMI, ";".to_string(), 5, 31))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(
    //             TokenType::COMMENT,
    //             "/* u-u/v*v == u mod v/*\r\n      that\r\n      something\r\n    */".to_string(),
    //             6,
    //             5
    //         ))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::RBRACE, "}".to_string(), 10, 1))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::VOID, "void".to_string(), 12, 1))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::ID, "main".to_string(), 12, 6))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::LPAREN, "(".to_string(), 12, 10))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::VOID, "void".to_string(), 12, 11))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::RPAREN, ")".to_string(), 12, 15))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::LBRACE, "{".to_string(), 12, 17))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::INT, "int".to_string(), 13, 5))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::ID, "x".to_string(), 13, 9))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::SEMI, ";".to_string(), 13, 10))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::INT, "int".to_string(), 14, 5))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::ID, "y".to_string(), 14, 9))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::SEMI, ";".to_string(), 14, 10))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::ID, "x".to_string(), 15, 5))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::ASSIGN, "=".to_string(), 15, 7))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::ID, "input".to_string(), 15, 9))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::LPAREN, "(".to_string(), 15, 14))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::RPAREN, ")".to_string(), 15, 15))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::SEMI, ";".to_string(), 15, 16))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::ID, "y".to_string(), 16, 5))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::ASSIGN, "=".to_string(), 16, 7))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::ID, "input".to_string(), 16, 9))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::LPAREN, "(".to_string(), 16, 14))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::RPAREN, ")".to_string(), 16, 15))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::SEMI, ";".to_string(), 16, 16))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::ID, "output".to_string(), 17, 5))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::LPAREN, "(".to_string(), 17, 11))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::ID, "gcd".to_string(), 17, 12))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::LPAREN, "(".to_string(), 17, 15))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::ID, "x".to_string(), 17, 16))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::COMMA, ",".to_string(), 17, 17))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::ID, "y".to_string(), 17, 18))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::RPAREN, ")".to_string(), 17, 19))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::RPAREN, ")".to_string(), 17, 20))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::SEMI, ";".to_string(), 17, 21))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::RBRACE, "}".to_string(), 18, 1))
    //     );
    //     assert_eq!(
    //         iter.next(),
    //         Some(Token::new(TokenType::ERROR, "/* u-u/v*v == u mod v/*".to_string(), 19, 1))
    //     );
    //     Ok(())
    // }
}
