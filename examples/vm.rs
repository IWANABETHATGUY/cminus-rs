use anyhow::Result;
use std::fs::read_to_string;
use std::path;
use std::time::Instant;
use tinylang_rs::{
    lexer::lex,
    parser::parse::Parser,
    vm::{EmitOperationCode, Vm},
};
#[cfg(target_arch = "x86_64")]
#[global_allocator]
static GLOBAL: mimallocator::Mimalloc = mimallocator::Mimalloc;
fn main() -> Result<()> {
    let path = path::Path::new("tests/fixtures/interpreter/test.txt");
    let content = read_to_string(path)?;
    let mut lex = lex::Lexer::new(&content);
    let source_code = read_to_string(path)?;
    let list = lex.lex();
    // println!("{:?}", list);
    let mut parser = Parser::new(list, &source_code);
    let now = Instant::now();
    let mut program = match parser.parse_program() {
        Ok(prog) => prog,
        Err(_) => {
            parser.error_reporter.emit_std()?;
            return Ok(());
        }
    };
    let mut vm = Vm::new();
    program.emit(&mut vm)?;
    // println!("{:?}", vm);
    vm.exec()?;
    // println!("{:?}",vm);
    // let start = Instant::now();
    println!("{:?}", now.elapsed());
    Ok(())
}
