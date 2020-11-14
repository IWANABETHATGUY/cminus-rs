mod error_emit;
mod lexer;
mod parser;
use lexer::lex::Lexer;
use parser::{parse::Parser, Walk};
use std::fs::read_to_string;
use std::path;
fn main() -> Result<(), std::io::Error> {
    let path = path::Path::new("tests/fixtures/parser/test.txt");
    let source_code = read_to_string(path)?;
    let mut lex = Lexer::new(&source_code);
    let list = lex.lex();
    // println!("{:?}", list);
    let mut parser = Parser::new(list, &source_code);
    let res = match parser.parse_program() {
        Ok(prog) => prog,
        Err(_) => {
            parser.error_reporter.emit_std()?;
            return Ok(());
        }
    };
    // parser.error_reporter.emit_std()?;
    println!("{}", res.walk(0));
    Ok(())
}
