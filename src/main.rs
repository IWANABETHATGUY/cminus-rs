#![feature(box_patterns)]
mod error_emit;
mod interpreter;
mod lexer;
mod parser;
use lexer::lex::Lexer;
mod util;
use parser::{parse::Parser, Walk};
use std::path;
use std::time::Instant;
use std::{fs::read_to_string};
#[cfg(target_arch = "x86_64")]
#[global_allocator]
static GLOBAL: mimallocator::Mimalloc = mimallocator::Mimalloc;
fn main() -> Result<(), std::io::Error> {
    let path = path::Path::new("tests/fixtures/vm/local.cm");
    let source_code = read_to_string(path)?;
    let mut lex = Lexer::new(&source_code);
    let list = lex.lex();
    // println!("{:?}", list);
    let mut parser = Parser::new(list, &source_code);
    let mut res = match parser.parse_program() {
        Ok(prog) => prog,
        Err(_) => {
            parser.error_reporter.emit_std()?;
            return Ok(());
        }
    };
    let start = Instant::now();
    match interpreter::interpret(&mut res, false) {
        Ok(env) => {
            println!("{}", env.get_std_simulator_string());
        }
        Err(_) => {
            println!("interpreter error",);
        }
    };
    println!("total: {:?}", start.elapsed());
    // println!("{}", res.walk(0));
    Ok(())
}
