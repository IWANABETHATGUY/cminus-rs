use super::{
    state::State,
    token::{Token, TokenType},
    util,
};

pub struct Lexer {
    cursor: usize,
    line: String,
    line_count: usize,
    total_len: usize,
    display_comment: bool,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            cursor: 0,
            line: "".to_string(),
            line_count: 0,
            total_len: 0,
            display_comment: false,
        }
    }
    fn is_keyword_token(s: String) -> TokenType {
        match util::is_keyword(s) {
            true => TokenType::KEYWORD,
            false => TokenType::ID,
        }
    }

    fn unget_next_char(&mut self) {
        self.cursor -= 1;
    }
}
