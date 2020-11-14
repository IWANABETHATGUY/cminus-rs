use super::{
    state::State,
    token::{Position, Token, TokenType},
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
    line: usize,
    column: usize,
    last_line: usize,
    last_column: usize,
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
            last_line: 0,
            last_column: 0,
        }
    }
    pub fn lex(&mut self) -> Vec<Token> {
        let mut token_list = vec![];
        while let Some(token) = self.get_token() {
            token_list.push(token);
        }
        token_list
    }
    fn keyword_or_id_token(s: &str) -> TokenType {
        util::keyword_or_id(s)
    }

    fn unget_next_char(&mut self) {
        self.cursor -= 1;
        self.column = self.last_column;
        self.line = self.last_line;
    }

    fn next_char(&mut self) {
        self.cursor += 1;
        self.last_line = self.line;
        self.last_column = self.column;
        self.column += 1;
    }

    fn get_token(&mut self) -> Option<Token> {
        if self.cursor == self.length {
            return None;
        }
        let mut result = "".to_string();
        let mut save;
       
        let mut token: Option<Token> = None;
        let mut state = State::START;
        let mut cur_token_type = TokenType::Error;
        let mut cur_line = 0;
        let mut cur_column = 0;
        let mut flag = false;
        let mut start_index = self.cursor;
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
                            cur_token_type = TokenType::Lt;
                        }
                    }
                    '/' => {
                        state = State::INDIVIDE;
                        if self.cursor == self.length {
                            state = State::DONE;
                            cur_token_type = TokenType::Times;
                        }
                    }
                    '>' => {
                        state = State::INGREAT;
                        if self.cursor == self.length {
                            state = State::DONE;
                            cur_token_type = TokenType::Gt;
                        }
                    }
                    '!' => {
                        state = State::INNOTEQUAL;
                        if self.cursor == self.length {
                            state = State::DONE;
                            cur_token_type = TokenType::Error;
                        }
                    }
                    '=' => {
                        state = State::INASSIGN;
                        if self.cursor == self.length {
                            state = State::DONE;
                            cur_token_type = TokenType::Assign;
                        }
                    }
                    _ if util::is_digit(cur_char) => {
                        state = State::INNUM;
                        if self.cursor == self.length {
                            state = State::DONE;
                            cur_token_type = TokenType::NumberLiteral;
                        }
                    }
                    _ if util::is_letter(cur_char) => {
                        state = State::INID;
                        if self.cursor == self.length {
                            state = State::DONE;
                            cur_token_type = TokenType::Id;
                        }
                    }
                    _ if cur_char == '\n' => {
                        self.last_line = self.line;
                        self.line += 1;
                        self.column = 0;
                        save = false;
                        if self.cursor == self.length {
                            state = State::DONE;
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
                            '+' => cur_token_type = TokenType::Plus,
                            '*' => cur_token_type = TokenType::Multiply,
                            '-' => cur_token_type = TokenType::Minus,
                            '(' => cur_token_type = TokenType::Lparen,
                            ')' => cur_token_type = TokenType::Rparen,
                            ';' => cur_token_type = TokenType::Semi,
                            ',' => cur_token_type = TokenType::Comma,
                            '[' => cur_token_type = TokenType::Lbrack,
                            ']' => cur_token_type = TokenType::Rbrack,
                            '{' => cur_token_type = TokenType::Lbrace,
                            '}' => cur_token_type = TokenType::Rbrace,
                            _ => cur_token_type = TokenType::Error,
                        }
                    }
                },
                State::INDIVIDE => match cur_char {
                    '*' => {
                        state = State::INCOMMENT;
                        if self.cursor == self.length {
                            state = State::DONE;
                            cur_token_type = TokenType::Error;
                        }
                    }
                    _ => {
                        state = State::DONE;
                        self.unget_next_char();
                        save = false;
                        cur_token_type = TokenType::Times;
                    }
                },
                // State::INMULTPLY => {} // do nothing
                State::INNUM => {
                    if !util::is_digit(cur_char) {
                        self.unget_next_char();
                        save = false;
                        state = State::DONE;
                        cur_token_type = TokenType::NumberLiteral;
                    } else if self.cursor == self.length {
                        state = State::DONE;
                        cur_token_type = TokenType::NumberLiteral;
                    }
                }
                State::INID => {
                    if !util::is_letter(cur_char) {
                        self.unget_next_char();
                        save = false;
                        state = State::DONE;
                        cur_token_type = TokenType::Id;
                    } else if self.cursor == self.length {
                        state = State::DONE;
                        cur_token_type = TokenType::Id;
                    }
                }
                State::DONE => {} // do nothing
                State::INLESS => {
                    state = State::DONE;
                    if cur_char == '=' {
                        cur_token_type = TokenType::Le;
                    } else {
                        self.unget_next_char();
                        save = false;
                        cur_token_type = TokenType::Lt;
                    }
                }
                State::INGREAT => {
                    state = State::DONE;
                    if cur_char == '=' {
                        cur_token_type = TokenType::Ge;
                    } else {
                        self.unget_next_char();
                        save = false;
                        cur_token_type = TokenType::Gt;
                    }
                }
                State::INASSIGN => {
                    state = State::DONE;
                    if cur_char == '=' {
                        cur_token_type = TokenType::Eq;
                    } else {
                        self.unget_next_char();
                        save = false;
                        cur_token_type = TokenType::Assign;
                    }
                }
                State::INNOTEQUAL => {
                    state = State::DONE;
                    if cur_char == '=' {
                        cur_token_type = TokenType::Ne;
                    } else {
                        self.unget_next_char();
                        save = false;
                        cur_token_type = TokenType::Error;
                    }
                }
                State::INCOMMENT => {
                    if cur_char == '\n' {
                        self.last_line = self.line;
                        self.line += 1;
                        self.column = 0;
                        if self.cursor == self.length {
                            state = State::DONE;
                        }
                    }
                    if cur_char == '*' {
                        state = State::INECOMMENT;
                    }
                    if self.cursor == self.length {
                        state = State::DONE;
                        cur_token_type = TokenType::Error;
                    }
                }
                State::INECOMMENT => {
                    if cur_char == '/' {
                        state = State::START;
                        if self.display_comment {
                            state = State::DONE;
                            cur_token_type = TokenType::Comment;
                        } else {
                            save = false;
                            result = "".to_string();
                        }
                    } else if cur_char == '*' {
                    } else {
                        state = State::INCOMMENT;
                        if cur_char == '\n' {
                            self.last_line = self.line;
                            self.line += 1;
                            self.column = 0;
                        }
                        // self.unget_next_char();
                    }
                }
            }
            if save {
                result += &cur_char.to_string();
            }
            if state != State::START && !flag {
                cur_line = self.last_line;
                cur_column = self.last_column;
                start_index = self.cursor - 1;
                flag = true;
            }
            if state == State::DONE && !result.is_empty() {
                if cur_token_type == TokenType::Id {
                    cur_token_type = Self::keyword_or_id_token(&result);
                }
                token = Some(Token::new(
                    cur_token_type,
                    result,
                    Position::new(cur_line, cur_column),
                    Position::new(self.line, self.column),
                    start_index,
                    self.cursor
                ));
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
