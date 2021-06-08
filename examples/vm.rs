use anyhow::Result;
use std::fs::read_to_string;
use tinylang_rs::{lexer::lex, parser::parse::Parser, vm::{EmitOperationCode, Vm}};
use std::path;
fn main() -> Result<()> {
    let path = path::Path::new("tests/fixtures/vm/global.cm");
    let content = read_to_string(path)?;
    let mut lex = lex::Lexer::new(&content);
    let source_code = read_to_string(path)?;
    let list = lex.lex();
    // println!("{:?}", list);
    let mut parser = Parser::new(list, &source_code);
    let mut program = match parser.parse_program() {
        Ok(prog) => prog,
        Err(_) => {
            parser.error_reporter.emit_std()?;
            return Ok(());
        }
    };
    let mut vm = Vm::new();
    program.emit(&mut vm)?;
    println!("{:?}", vm);
    vm.exec()?;
    println!("{:?}",vm);
    // let start = Instant::now();
    Ok(())
}
