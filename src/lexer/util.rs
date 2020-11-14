use crate::lexer::token::Keyword;
use crate::lexer::token::TokenType;
pub fn is_digit(ch: char) -> bool {
    ch.is_digit(10)
}

// pub fn is_id(s: String) -> bool {
//     !s.is_empty() && s.chars().all(is_letter)
// }

pub fn is_letter(c: char) -> bool {
    c.is_ascii_alphabetic()
}
// pub fn is_num(s: String) -> bool {
//     !s.is_empty() && s.chars().all(|c: char| c.is_digit(10))
// }

pub fn keyword_or_id(s: &str) -> TokenType {
    match s {
        "else" => TokenType::KEYWORD(Keyword::ELSE),
        "if" => TokenType::KEYWORD(Keyword::IF),
        "int" => TokenType::KEYWORD(Keyword::INT),
        "return" => TokenType::KEYWORD(Keyword::RETURN),
        "void" => TokenType::KEYWORD(Keyword::VOID),
        "while" => TokenType::KEYWORD(Keyword::WHILE),
        "bool" => TokenType::KEYWORD(Keyword::BOOL),
        _ => TokenType::ID,
    }
}
