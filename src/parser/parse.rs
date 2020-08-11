#![feature(type_alias_enum_variants)]
use crate::lexer::{
    lex,
    token::{Token, TokenType},
};
use std::fmt::{Debug, Display};
pub trait Walk {
    fn walk(&self, level: usize);
}
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
impl From<std::io::Error> for ParseError {
    fn from(error: std::io::Error) -> Self {
        ParseError {
            error: error.to_string(),
        }
    }
}
impl From<std::num::ParseIntError> for ParseError {
    fn from(error: std::num::ParseIntError) -> Self {
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
    fn match_type_specifier(&mut self) -> bool {
        // let next_token = self.next_token();
        if let Some(token) = self.next_token() {
            return token.token_type == TokenType::VOID || token.token_type == TokenType::INT;
        }
        false
    }
    fn consume(&mut self, step: usize) {
        self.cursor += step;
    }
    fn match_and_consume(&mut self, token_type: TokenType) -> Result<Token, ParseError> {
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
        let mut num = None;
        if self.match_token(TokenType::LBRACK) {
            self.match_and_consume(TokenType::LBRACK)?;
            let num_token = self.match_and_consume(TokenType::NUM)?;
            num = Some(NumberLiteral {
                value: num_token.content.parse::<i32>()?,
            });
            self.match_and_consume(TokenType::RBRACK)?;
        }
        self.match_and_consume(TokenType::SEMI)?;
        Ok(Declaration::VarDeclaration(VarDeclaration {
            type_specifier,
            id: identifier,
            num,
        }))
    }
    // fn parse_variable_declaration(&self) {

    // }
    fn parse_function_declaration(&mut self) -> Result<Declaration, ParseError> {
        let type_specifier = self.parse_type_specifier()?;
        let id_token = self.match_and_consume(TokenType::ID)?;
        let mut params: Params = Params::Void;
        let identifier = Identifier {
            value: id_token.content,
        };
        self.match_and_consume(TokenType::LPAREN)?;
        match self.next_token() {
            Some(token) => match token.token_type {
                TokenType::VOID => {
                    self.consume(1);
                    params = Params::Void;
                }
                TokenType::LPAREN => {}
                _ => {
                    let mut params_list = vec![];
                    params_list.push(self.parse_param()?);
                    while !self.match_token(TokenType::RPAREN) {
                        self.match_and_consume(TokenType::COMMA)?;
                        params_list.push(self.parse_param()?);
                    }
                    params = Params::ParamsList {
                        params: params_list,
                    }
                }
            },
            None => {}
        }
        self.match_and_consume(TokenType::RPAREN)?;
        let body = self.parse_compound_statement()?;
        Ok(Declaration::FunctionDeclaration(FunctionDeclaration {
            type_specifier,
            id: identifier,
            params,
            body,
        }))
    }

    fn parse_compound_statement(&mut self) -> Result<CompoundStatement, ParseError> {
        self.match_and_consume(TokenType::LBRACE)?;
        let mut local_declaration = vec![];
        while self.match_type_specifier() {
            match self.parse_variable_declaration() {
                Ok(decl) => match decl {
                    Declaration::FunctionDeclaration(_) => {
                        return Err(ParseError::from("Unexpected function declaration"));
                    }
                    Declaration::VarDeclaration(var_decl) => {
                        local_declaration.push(var_decl);
                    }
                },
                Err(err) => {
                    return Err(err);
                }
            }
        }
        self.match_and_consume(TokenType::RBRACE)?;
        Ok(CompoundStatement { local_declaration })
    }
    fn parse_param(&mut self) -> Result<Parameter, ParseError> {
        let type_specifier = self.parse_type_specifier()?;
        let id_token = self.match_and_consume(TokenType::ID)?;
        let identifier = Identifier {
            value: id_token.content,
        };
        let mut is_array = false;
        if self.match_token(TokenType::LBRACK) {
            self.match_and_consume(TokenType::LBRACK)?;
            self.match_and_consume(TokenType::RBRACK)?;
            is_array = true;
        }
        Ok(Parameter {
            type_specifier,
            id: identifier,
            is_array,
        })
    }
    fn parse_type_specifier(&mut self) -> Result<TypeSpecifier, ParseError> {
        if let Some(token) = self.next_token() {
            match token.token_type {
                TokenType::INT => {
                    self.consume(1);
                    return Ok(TypeSpecifier {
                        kind: TypeSpecifierKind::Int,
                    });
                }
                TokenType::VOID => {
                    self.consume(1);
                    return Ok(TypeSpecifier {
                        kind: TypeSpecifierKind::Void,
                    });
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
impl Walk for Program {
    fn walk(&self, level: usize) {
        println!("{}Program", " ".repeat(2 * level));
        for decl in self.declarations.iter() {
            decl.walk(level + 1);
        }
    }
}
// impl Debug for Program {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

//     }

// }
#[derive(Debug)]
pub struct FunctionDeclaration {
    type_specifier: TypeSpecifier,
    id: Identifier,
    params: Params,
    body: CompoundStatement,
}
impl Walk for FunctionDeclaration {
    fn walk(&self, level: usize) {
        println!("{}FunctionDeclaration", " ".repeat(2 * level));
        self.type_specifier.walk(level + 1);
        self.id.walk(level + 1);
        self.params.walk(level + 1);
        self.body.walk(level + 1);
    }
}
#[derive(Debug)]
pub struct VarDeclaration {
    type_specifier: TypeSpecifier,
    id: Identifier,
    num: Option<NumberLiteral>,
}
impl Walk for VarDeclaration {
    fn walk(&self, level: usize) {
        println!("{}VarDeclaration", " ".repeat(2 * level));
        self.id.walk(level + 1);
        self.type_specifier.walk(level + 1);
        if let Some(ref num) = self.num {
            num.walk(level + 1);
        }
    }
}
#[derive(Debug)]
pub enum Declaration {
    FunctionDeclaration(FunctionDeclaration),
    VarDeclaration(VarDeclaration),
}
impl Walk for Declaration {
    fn walk(&self, level: usize) {
        match &self {
            Declaration::VarDeclaration(var_decl) => {
                var_decl.walk(level);
            }
            Declaration::FunctionDeclaration(func_decl) => {
                func_decl.walk(level);
            }
        }
    }
}
#[derive(Debug)]
pub struct Identifier {
    value: String,
}
impl Walk for Identifier {
    fn walk(&self, level: usize) {
        println!("{}Identifier({})", " ".repeat(2 * level), self.value);
    }
}
#[derive(Debug)]
pub struct NumberLiteral {
    value: i32,
}
impl Walk for NumberLiteral {
    fn walk(&self, level: usize) {
        println!("{}NumberLiteral({})", " ".repeat(2 * level), self.value);
    }
}
#[derive(Debug)]
pub struct TypeSpecifier {
    kind: TypeSpecifierKind,
}

impl Walk for TypeSpecifier {
    fn walk(&self, level: usize) {
        println!("{}TypeSpecifier({:?})", " ".repeat(2 * level), self.kind);
    }
}
#[derive(Debug)]
enum TypeSpecifierKind {
    Int,
    Void,
}
#[derive(Debug)]
pub enum Params {
    Void,
    ParamsList { params: Vec<Parameter> },
}

impl Walk for Params {
    fn walk(&self, level: usize) {
        match self {
            Params::Void => {
                println!("{}Void", " ".repeat(2 * level));
            }
            Params::ParamsList { params } => {
                println!("{}ParameterList", " ".repeat(2 * level));
                for param in params.iter() {
                    param.walk(level + 1);
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Parameter {
    type_specifier: TypeSpecifier,
    id: Identifier,
    is_array: bool,
}

impl Walk for Parameter {
    fn walk(&self, level: usize) {
        println!(
            "{}Parameter({:?} {}{})",
            " ".repeat(2 * level),
            self.type_specifier.kind,
            self.id.value,
            if self.is_array { "[]" } else { "" }
        );
    }
}

#[derive(Debug)]
struct CompoundStatement {
    local_declaration: Vec<VarDeclaration>,
}

impl Walk for CompoundStatement {
    fn walk(&self, level: usize) {
        println!("{}Body", " ".repeat(2 * level));
        for var_decl in self.local_declaration.iter() {
            var_decl.walk(level + 1);
        }
    }
    
}
