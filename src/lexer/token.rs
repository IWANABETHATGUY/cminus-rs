#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TokenType {
    ENDFILE,
    ERROR,
    /* reserved words */
    IF,
    ELSE,
    INT,
    RETURN,
    VOID,
    WHILE,
    /* multicharacter tokens */
    ID,
    NUM,
    KEYWORD,
    /* special symbols */
    PLUS,
    MINUS,
    MULTIPLY,
    TIMES,
    LT,
    LE,
    GT,
    GE,
    EQ,
    NE,
    SEMI,
    COMMA,
    LPAREN, // (
    RPAREN, // )
    LBRACK, // [
    RBRACK, // ]
    LBRACE, // {
    RBRACE, // }
    COMMENT,
    ASSIGN,
}
//left close, right open
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub content: String,
    // pos: usize,
    pub start_position: Position,
    pub end_position: Position,
    pub start_index: usize,
    pub end_index: usize,
}
pub struct Range {
    start: Position,
    end: Position,
}
impl Token {
    pub fn new(
        token_type: TokenType,
        content: String,
        start_position: Position,
        end_position: Position,
        start_index: usize,
        end_index: usize,
    ) -> Self {
        Self {
            token_type,
            content,
            start_position,
            end_position,
            start_index,
            end_index,
        }
    }

    pub fn get_token_type(&self) -> TokenType {
        self.token_type
    }

    pub fn get_token_string(&self) -> String {
        self.content.clone()
    }

    pub fn get_token_range(&self) -> Range {
        Range {
            start: self.start_position.clone(),
            end: self.end_position.clone(),
        }
    }
}
