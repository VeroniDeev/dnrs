use std::{
    collections::BTreeMap,
    fs::{self, ReadDir},
};

use self::{
    lexer::lex::Lexer,
    parser::parser::{Literal, ParseError, Parser},
};

pub mod lexer;
pub mod parser;

fn read_file(path: String) -> String {
    let source: String = fs::read_to_string(path).unwrap();

    return source;
}

pub fn file_finder(domain: String) -> Option<String> {
    let paths: ReadDir = fs::read_dir("dns").unwrap();

    for path in paths {
        let path_string: String = path.unwrap().path().display().to_string();

        let source: String = read_file(path_string);

        if let Some(_) = source.find(&domain) {
            return Some(source);
        }
    }

    return None;
}

pub fn analyse(source: String) -> Result<BTreeMap<String, Literal>, ParseError> {
    let mut lexer: Lexer = Lexer::new(source);
    lexer.lexer();

    let mut parser: Parser = Parser::new(lexer.tokens.clone());

    match parser.parser() {
        Ok(_) => Ok(parser.properties),
        Err(err) => Err(err),
    }
}
