mod lexer;
use lexer::lex::Lexer;
use lexer::token::{Token, TokenType};

use std::fs::{read_to_string, File};
use std::io::Error;
use std::path;
fn main() -> Result<(), Error> {
    let path = path::Path::new("test.txt");
    let a = read_to_string(path)?;
    let mut lex = Lexer::new(&a);
    let list = lex.lex();
    for token in list {
        println!("{:?}", token);
    }
    Ok(())
}
