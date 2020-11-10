mod utils;

use tinylang_rs::{
    lexer::lex::Lexer,
    parser::parse::{Parser, Walk},
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
    let mut parser = Parser::new(list);
    let res = parser.parse_program();
    match res {
        Ok(program) => program.walk(0),
        Err(err) => err.to_string(),
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
