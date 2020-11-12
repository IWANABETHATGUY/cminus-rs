mod error_emit;
mod lexer;
mod parser;
use lexer::lex::Lexer;
use parser::{parse::Parser, Walk};
use std::fs::read_to_string;
use std::path;
fn main() -> Result<(), std::io::Error> {
    let path = path::Path::new("tests/fixtures/lexer2.test.txt");
    let a = read_to_string(path)?;
    let mut lex = Lexer::new(&a);
    let list = lex.lex();
    let mut parser = Parser::new(list);
    let res = match parser.parse_program() {
        Ok(prog) => prog,
        Err(_) => {
            return Ok(());
        }
    };
    println!("{}", res.walk(0));
    Ok(())
}
