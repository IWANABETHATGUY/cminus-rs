mod lexer;
use lexer::lexer::Lexer;
use lexer::token::{Token, TokenType};
fn main() {
    let a = "a\nfuck".to_string();
    a.chars().for_each(|ch| {
        if ch == '\n' {
            println!("fuck",);
        } else {
            println!("{}", ch);
        }
    });
}
