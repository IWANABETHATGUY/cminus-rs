#[derive(Debug)]
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

pub struct Token {
    ttype: TokenType,
    content: String,
    pos: usize,
}

impl Token {
    pub fn new(ttype: TokenType, content: String, pos: usize) -> Self {
        Self {
            ttype,
            content,
            pos,
        }
    }
}
