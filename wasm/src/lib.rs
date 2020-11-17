mod utils;

use tinylang_rs::{
    interpreter,
    lexer::lex::Lexer,
    parser::{parse::Parser, Walk},
};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn parse(source_code: String) -> String {
    let mut lex = Lexer::new(&source_code);
    let list = lex.lex();
    let mut parser = Parser::new(list, &source_code);
    let res = parser.parse_program();
    match res {
        Ok(program) => program.walk(0),
        Err(_) => parser.error_reporter.emit_string(),
    }
}

#[wasm_bindgen]
pub fn tokenize(source_code: String) -> String {
    let mut lex = Lexer::new(&source_code);
    let list = lex.lex();
    list.into_iter()
        .map(|token| format!("{:?}", token))
        .collect::<Vec<String>>()
        .join("\n")
}

#[wasm_bindgen]
pub fn interpret(source_code: String) -> String {
    let mut lex = Lexer::new(&source_code);
    let list = lex.lex();
    let mut parser = Parser::new(list, &source_code);
    let res = parser.parse_program();
    match res {
        Ok(mut program) => match interpreter::interpret(&mut program, false) {
            Ok(env) => {
                env.get_std_simulator_string()
            }
            Err(_) => {
                format!("interpreter error",)
            }
        },
        Err(_) => parser.error_reporter.emit_string(),
    }
}
