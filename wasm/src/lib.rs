mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn parse(source_code: String) -> String {
    let path = path::Path::new("tests/fixtures/parser.test.txt");
    let a = read_to_string(path)?;
    let mut lex = Lexer::new(&a);
    let list = lex.lex();
    let mut parser = Parser::new(list);
    let res = parser.parse_program()?;

}