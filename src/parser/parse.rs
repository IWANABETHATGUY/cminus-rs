use crate::lexer::{
    lex,
    token::{Token, TokenType},
};
use std::{
    fmt::{Debug, Display},
    ops::Index,
};

pub struct ParseError {
    error: String,
}
impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}
impl Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}
impl From<std::io::Error>  for ParseError {
    fn from(error: std::io::Error) -> Self {
        ParseError {
            error: error.to_string(),
        }
    }
}
impl From<String> for ParseError {
    fn from(error: String) -> Self {
        ParseError { error }
    }
}
impl From<&str> for ParseError {
    fn from(error: &str) -> Self {
        ParseError {
            error: error.to_string(),
        }
    }
}
pub struct Parser {
    token_list: Vec<Token>,
    cursor: usize,
}

impl Parser {
    pub fn new(token_list: Vec<Token>) -> Parser {
        Self {
            token_list,
            cursor: 0,
        }
    }
    fn next_token(&mut self) -> Option<&Token> {
        self.token_list.get(self.cursor)
    }
    fn match_token(&mut self, token_type: TokenType) -> bool {
        let next_token = self.next_token();
        if next_token.is_some() && next_token.unwrap().token_type == token_type {
            return true;
        }
        false
    }
    fn consume(&mut self, step: usize) {
        self.cursor += step;
    }
    fn match_and_consume(&mut self, token_type: TokenType) -> Result<Token, ParseError> {
        // let mut token = None;
        // {
        //     token = self.next_token();
        // }
        let token = self.next_token();
        if token.is_none() {
            return Err(ParseError::from(format!("expected {:?}", token_type)));
        }
        let token = token.unwrap().clone();
        if token.token_type == token_type {
            self.consume(1);
            return Ok(token);
        }

        return Err(ParseError::from(format!("expected {:?}", token_type)));
    }
    fn backtrack(&mut self, step: usize) {
        self.cursor -= step;
    }
    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut declarations = vec![];
        while self.cursor < self.token_list.len() {
            let declaration = self.parse_declaration()?;
            declarations.push(declaration);
        }
        Ok(Program { declarations })
    }
    fn parse_declaration(&mut self) -> Result<Declaration, ParseError> {
        if self.match_token(TokenType::INT) || self.match_token(TokenType::VOID) {
            self.consume(1);
        }
        if self.match_token(TokenType::ID) {
            self.consume(1);
        }
        if self.match_token(TokenType::LPAREN) {
            self.backtrack(2);
            return self.parse_function_declaration();
        } else {
            self.backtrack(2);
            return self.parse_variable_declaration();
        }
        // return Err(ParseError::from("expected `int`,`void`"));
    }

    fn parse_variable_declaration(&mut self) -> Result<Declaration, ParseError> {
        let type_specifier = self.parse_type_specifier()?;
        let id_token = self.match_and_consume(TokenType::ID)?;
        let identifier = Identifier {
            value: id_token.content,
        };
        let num = None;
        if self.match_token(TokenType::LBRACE) {}
        self.match_and_consume(TokenType::SEMI)?;
        Ok(Declaration::VarDeclaration {
            type_specifier,
            id: identifier,
            num,
        })
    }
    // fn parse_variable_declaration(&self) {

    // }
    fn parse_function_declaration(&mut self) -> Result<Declaration, ParseError> {
        let type_specifier = self.parse_type_specifier()?;
        Ok(Declaration::FunctionDeclaration {})
    }

    fn parse_type_specifier(&mut self) -> Result<TypeSpecifier, ParseError> {
        if let Some(token) = self.next_token() {
            match token.token_type {
                TokenType::INT => {
                    self.consume(1);
                    return Ok(TypeSpecifier {
                        kind: TypeSpecifierKind::Int,
                    })
                }
                TokenType::VOID => {
                    self.consume(1);
                    return Ok(TypeSpecifier {
                        kind: TypeSpecifierKind::Void,
                    })
                }
                _ => return Err(ParseError::from("expected `int` or `void`")),
            }
        }
        return Err(ParseError::from("expected `int` or `void`"));
    }
}

#[derive(Debug)]
pub struct Program {
    declarations: Vec<Declaration>,
}

// impl Debug for Program {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

//     }

// }

#[derive(Debug)]
pub enum Declaration {
    FunctionDeclaration {},
    VarDeclaration {
        type_specifier: TypeSpecifier,
        id: Identifier,
        num: Option<NumberLiteral>,
    },
}
#[derive(Debug)]
pub struct Identifier {
    value: String,
}
#[derive(Debug)]
pub struct NumberLiteral {
    value: i32,
}
#[derive(Debug)]
pub struct TypeSpecifier {
    kind: TypeSpecifierKind,
}
#[derive(Debug)]
enum TypeSpecifierKind {
    Int,
    Void,
}
