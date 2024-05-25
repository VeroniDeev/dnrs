use std::fs::read_to_string;

use language::{lexer::lex::Lexer, parser::parser::Parser};

pub mod language;
pub mod structs;

fn main() {
    let file = read_to_string("dns/zoubheir.com").unwrap();
    let mut lexer = Lexer::new(file);
    lexer.lexer();

    let mut parser = Parser::new(lexer.tokens);
    parser.parser();
    println!("{:?}", parser.properties);
}
