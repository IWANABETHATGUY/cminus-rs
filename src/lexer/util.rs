use crate::lexer::token::KeywordType;
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
        "else" => TokenType::Keyword(KeywordType::ELSE),
        "if" => TokenType::Keyword(KeywordType::IF),
        "int" => TokenType::Keyword(KeywordType::INT),
        "return" => TokenType::Keyword(KeywordType::RETURN),
        "void" => TokenType::Keyword(KeywordType::VOID),
        "while" => TokenType::Keyword(KeywordType::WHILE),
        "bool" => TokenType::Keyword(KeywordType::BOOL),
        "true" | "false" => TokenType::BooleanLiteral,
        _ => TokenType::Id,
    }
}
