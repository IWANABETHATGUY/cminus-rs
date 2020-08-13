mod lexer;
mod parser;

use lexer::lex::Lexer;
use parser::error::ParseError;
use parser::parse::{Parser, Walk};
use std::fs::read_to_string;
use std::path;
fn main() -> Result<(), ParseError> {
    let path = path::Path::new("tests/fixtures/parser.test.txt");
    let a = read_to_string(path)?;
    let mut lex = Lexer::new(&a);
    let list = lex.lex();
    let mut parser = Parser::new(list);
    let res = parser.parse_program()?;

    res.walk(0);
    Ok(())
}
