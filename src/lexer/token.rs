#[derive(Debug, PartialEq, Copy, Clone)]
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
    LPAREN,
    RPAREN,
    LBRACK,
    RBRACK,
    LBRACE,
    RBRACE,
    COMMENT,
    ASSIGN,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    token_type: TokenType,
    content: String,
    // pos: usize,
    line: i32,
    column: i32,
}

impl Token {
    pub fn new(ttype: TokenType, content: String, line: i32, column: i32) -> Self {
        Self {
            token_type: ttype,
            content,
            line,
            column,
        }
    }

    pub fn get_token_type(&self) -> TokenType {
        self.token_type
    }

    pub fn get_token_string(&self) -> String {
        self.content.clone()
    }

    pub fn get_token_position(&self) -> (i32, i32) {
        (self.line, self.column)
    }
}
