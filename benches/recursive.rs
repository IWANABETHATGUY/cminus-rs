use std::{fs::read_to_string, path};

use criterion::{criterion_group, criterion_main, Criterion};
use tinylang_rs::{interpreter, lexer::lex::Lexer, parser::parse::Parser};
fn criterion_benchmark(c: &mut Criterion) {
    let path = path::Path::new("benches/corpus/fibonacci.cm");
    let source_code = read_to_string(path).unwrap();
    let mut lex = Lexer::new(&source_code);
    let list = lex.lex();
    // println!("{:?}", list);
    let mut parser = Parser::new(list, &source_code);
    let mut res = match parser.parse_program() {
        Ok(prog) => prog,
        Err(()) => {
            parser.error_reporter.emit_std().unwrap();
            panic!("error when parsing");
        }
    };
    c.bench_function("lexer", |b| {
        let path = path::Path::new("benches/corpus/fibonacci.cm");
        let source_code = read_to_string(path).unwrap();
        b.iter(|| {
            let mut lex = Lexer::new(&source_code);
            let _list = lex.lex();
        });
    });
    c.bench_function("parser", |b| {
        let path = path::Path::new("benches/corpus/fibonacci.cm");
        let source_code = read_to_string(path).unwrap();
        let mut lex = Lexer::new(&source_code);
        let list = lex.lex();
        b.iter(|| {
            let mut parser = Parser::new(list.clone(), &source_code);
            let mut _res = match parser.parse_program() {
                Ok(prog) => prog,
                Err(()) => {
                    parser.error_reporter.emit_std().unwrap();
                    panic!("error when parsing");
                }
            };
        });
    });
    c.bench_function("fib 20", |b| {
        b.iter(|| interpreter::interpret(&mut res, false))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
