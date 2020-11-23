use std::fmt;

#[derive(Copy, Debug, Clone, Eq, PartialEq)]
pub enum KeywordType {
    IF,
    ELSE,
    INT,
    RETURN,
    VOID,
    WHILE,
    BOOL,
}
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TokenType {
    // ENDFILE,
    Error,
    /* reserved words */
    /* multicharacter tokens */
    Id,
    NumberLiteral,
    BooleanLiteral,
    Keyword(KeywordType),
    /* special symbols */
    Plus,
    Minus,
    Multiply,
    Times,
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
    Semi,
    Comma,
    Lparen, // (
    Rparen, // )
    Lbrack, // [
    Rbrack, // ]
    Lbrace, // {
    Rbrace, // }
    Comment,
    Assign,
}
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            TokenType::Error => {
                "`error`"
            }
            TokenType::Id => {
                "id"
            }
            TokenType::NumberLiteral => {
                "numberLiteral"
            }
            TokenType::BooleanLiteral => {
                "booleanLiteral"
            }
            TokenType::Keyword(k) => {
                match k {
                    KeywordType::IF => "if",
                    KeywordType::ELSE => "else",
                    KeywordType::INT => "int",
                    KeywordType::RETURN => "return",
                    KeywordType::VOID => "void",
                    KeywordType::WHILE => "while",
                    KeywordType::BOOL => "bool",
                }
            }
            TokenType::Plus => "`+`",
            TokenType::Minus => {"`-`"}
            TokenType::Multiply => {"`*`"}
            TokenType::Times => {"`/`"}
            TokenType::Lt => {"`<`"}
            TokenType::Le => {"`<=`"}
            TokenType::Gt => {"`>`"}
            TokenType::Ge => {"`>=`"}
            TokenType::Eq => {"`==`"}
            TokenType::Ne => {"`!=`"}
            TokenType::Semi => {"`;`"}
            TokenType::Comma => {"`,`"}
            TokenType::Lparen => {"`(`"}
            TokenType::Rparen => {"`)`"}
            TokenType::Lbrack => {"`[`"}
            TokenType::Rbrack => {"`]`"}
            TokenType::Lbrace => {"`{`"}
            TokenType::Rbrace => {"`}`"}
            TokenType::Comment => {"comment"}
            TokenType::Assign => {"`=`"}
        };
        f.write_str(string)
    }
}
//left close, right open
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    /// zero based line and column
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
// pub struct Range {
//     pub start: Position,
//     pub end: Position,
// }
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
    pub fn range(&self) -> impl Into<std::ops::Range<usize>> {
        self.start_index..self.end_index
    }
    // pub fn get_token_type(&self) -> TokenType {
    //     self.token_type
    // }

    // pub fn get_token_string(&self) -> String {
    //     self.content.clone()
    // }

    // pub fn get_token_range(&self) -> Range {
    //     Range {
    //         start: self.start_position.clone(),
    //         end: self.end_position.clone(),
    //     }
    // }
}
