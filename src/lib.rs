#![feature(box_patterns)]
#![feature(or_patterns)]
pub mod lexer;
pub mod parser;
pub mod interpreter;
pub mod error_emit;
mod macros;
pub use macros::*;