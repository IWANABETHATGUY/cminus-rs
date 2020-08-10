mod lexer;
mod parser;

use lexer::lex::Lexer;
use lexer::token::{Token, TokenType};
use parser::parse::{ParseError, Parser};
use std::fs::{read_to_string, File};
use std::io::Error;
use std::path;

fn main() -> Result<(), ParseError> {
    let path = path::Path::new("tests/fixtures/parser.test.txt");
    let a = read_to_string(path)?;
    let mut lex = Lexer::new(&a);
    let list = lex.lex();
    let mut parser = Parser::new(list);
    let res = parser.parse_program()?;
    println!("{:?}", res);
    Ok(())
}