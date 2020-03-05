use super::{
    state::State,
    token::{Token, TokenType},
    util,
};

pub struct Lexer {
    cursor: usize,
    length: usize,
    pub file: String,
    // line_count: usize,
    // total_len: usize,
    display_comment: bool,
    file_vec: Vec<char>,
    line: u32,
    column: u32,
}

impl Lexer {
    pub fn new(file: &str) -> Self {
        let file_vec = file.chars().collect();
        Lexer {
            cursor: 0,
            file: file.to_string(),
            display_comment: true,
            length: file.len(),
            file_vec,
            line: 0,
            column: 0,
        }
    }
    pub fn lex(&mut self) -> Vec<Token> {
        let mut token_list = vec![];
        while let Some(token) = self.get_token() {
            token_list.push(token);
        }
        token_list
    }
    fn keyword_or_id_token(s: &String) -> TokenType {
        util::keyword_or_id(s)
    }

    fn unget_next_char(&mut self) {
        self.cursor -= 1;
    }

    fn next_char(&mut self) {
        self.cursor += 1;
    }

    fn get_token(&mut self) -> Option<Token> {
        if self.cursor == self.length {
            return None;
        }
        let mut result = "".to_string();
        let mut save = false;
        // let mut start_index = 0;
        // let mut token =
        let mut token: Option<Token> = None;
        let mut state = State::START;
        let mut cur_token = TokenType::ERROR;
        let cur_line = self.line;
        let cur_column = self.column;
        while state != State::DONE && self.cursor < self.length {
            let cur_char = self.file_vec[self.cursor];
            self.next_char();
            save = true;
            match state {
                State::START => match cur_char {
                    '<' => {
                        state = State::INLESS;
                        if self.cursor == self.length {
                            state = State::DONE;
                            cur_token = TokenType::LT;
                        }
                    }
                    '/' => {
                        state = State::INDIVIDE;
                        if self.cursor == self.length {
                            state = State::DONE;
                            cur_token = TokenType::TIMES;
                        }
                    }
                    '>' => {
                        state = State::INGREAT;
                        if self.cursor == self.length {
                            state = State::DONE;
                            cur_token = TokenType::GT;
                        }
                    }
                    '!' => {
                        state = State::INNOTEQUAL;
                        if self.cursor == self.length {
                            state = State::DONE;
                            cur_token = TokenType::ERROR;
                        }
                    }
                    '=' => {
                        state = State::INASSIGN;
                        if self.cursor == self.length {
                            state = State::DONE;
                            cur_token = TokenType::ASSIGN;
                        }
                    }
                    _ if util::is_digit(cur_char) => {
                        state = State::INNUM;
                        if self.cursor == self.length {
                            state = State::DONE;
                            cur_token = TokenType::NUM;
                        }
                    }
                    _ if util::is_letter(cur_char) => {
                        state = State::INID;
                        if self.cursor == self.length {
                            state = State::DONE;
                            cur_token = TokenType::ID;
                        }
                    }
                    _ if cur_char.is_whitespace() => {
                        save = false;
                        if self.cursor == self.length {
                            state = State::DONE;
                        }
                    }
                    _ => {
                        state = State::DONE;
                        match cur_char {
                            '+' => cur_token = TokenType::PLUS,
                            '*' => cur_token = TokenType::MULTIPLY,
                            '-' => cur_token = TokenType::MINUS,
                            '(' => cur_token = TokenType::LPAREN,
                            ')' => cur_token = TokenType::RPAREN,
                            ';' => cur_token = TokenType::SEMI,
                            ',' => cur_token = TokenType::COMMA,
                            '[' => cur_token = TokenType::LBRACK,
                            ']' => cur_token = TokenType::RBRACK,
                            '{' => cur_token = TokenType::LBRACE,
                            '}' => cur_token = TokenType::RBRACE,
                            _ => cur_token = TokenType::ERROR,
                        }
                    }
                },
                State::INDIVIDE => match cur_char {
                    '*' => {
                        state = State::INCOMMENT;
                        if self.cursor == self.length {
                            state = State::DONE;
                            cur_token = TokenType::ERROR;
                        }
                    }
                    _ => {
                        state = State::DONE;
                        self.unget_next_char();
                        save = false;
                        cur_token = TokenType::TIMES;
                    }
                },
                State::INMULTPLY => {} // do nothing
                State::INNUM => {
                    if !util::is_digit(cur_char) {
                        self.unget_next_char();
                        save = false;
                        state = State::DONE;
                        cur_token = TokenType::NUM;
                    } else if self.cursor == self.length {
                        state = State::DONE;
                        cur_token = TokenType::NUM;
                    }
                }
                State::INID => {
                    if !util::is_letter(cur_char) {
                        self.unget_next_char();
                        save = false;
                        state = State::DONE;
                        cur_token = TokenType::ID;
                    } else if self.cursor == self.length {
                        state = State::DONE;
                        cur_token = TokenType::ID;
                    }
                }
                State::DONE => {} // do nothing
                State::INLESS => {
                    state = State::DONE;
                    if cur_char == '=' {
                        cur_token = TokenType::LE;
                    } else {
                        self.unget_next_char();
                        save = false;
                        cur_token = TokenType::LT;
                    }
                }
                State::INGREAT => {
                    state = State::DONE;
                    if cur_char == '=' {
                        cur_token = TokenType::GE;
                    } else {
                        self.unget_next_char();
                        save = false;
                        cur_token = TokenType::GT;
                    }
                }
                State::INASSIGN => {
                    state = State::DONE;
                    if cur_char == '=' {
                        cur_token = TokenType::EQ;
                    } else {
                        self.unget_next_char();
                        save = false;
                        cur_token = TokenType::ASSIGN;
                    }
                }
                State::INNOTEQUAL => {
                    state = State::DONE;
                    if cur_char == '=' {
                        cur_token = TokenType::EQ;
                    } else {
                        self.unget_next_char();
                        save = false;
                        cur_token = TokenType::ERROR;
                    }
                }
                State::INCOMMENT => {
                    if cur_char == '*' {
                        state = State::INECOMMENT;
                    }
                    if self.cursor == self.length {
                        state = State::DONE;
                        cur_token = TokenType::ERROR;
                    }
                }
                State::INECOMMENT => {
                    if cur_char == '/' {
                        state = State::START;
                        if self.display_comment {
                            state = State::DONE;
                            cur_token = TokenType::COMMENT;
                        } else {
                            save = false;
                            result = "".to_string();
                        }
                    } else if cur_char == '*' {
                        state = State::INECOMMENT;
                    } else {
                        state = State::INCOMMENT;
                    }
                }
            }
            if save {
                result += &cur_char.to_string();
            }
            if state == State::DONE {
                if cur_token == TokenType::ID {
                    cur_token = Self::keyword_or_id_token(&result);
                }
                token = Some(Token::new(cur_token, result, cur_line, cur_column));
                break;
            }
        }
        token
    }
}

// fn confirm_state_and_token(
//     is_none: bool,
//     state: &mut State,
//     cur_token: &mut TokenType,
//     final_token_type: TokenType,
// ) {
//     if is_none {
//         *state = State::DONE;
//         *cur_token = final_token_type;
//     }
// }
