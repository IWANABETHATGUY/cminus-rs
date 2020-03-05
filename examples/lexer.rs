use tinylang_rs::lexer::lex;
use std::fs::read_to_string;
use std::io::Error;
fn main() -> Result<(), Error>{
    let path = std::path::Path::new("test.txt");
    let content = read_to_string(path)?;
    let mut a = lex::Lexer::new(&content);
    let tokens = a.lex();
    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}
