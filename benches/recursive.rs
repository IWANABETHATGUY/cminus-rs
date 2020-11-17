use std::{path, fs::read_to_string};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tinylang_rs::{lexer::lex::Lexer, interpreter, parser::{self, parse::Parser}};
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
            parser.error_reporter.emit_std();
            panic!("error when parsing");
        }
    };
    c.bench_function("fib 20", |b| b.iter(|| {
        interpreter::interpret(&mut res, false)
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
