use std::collections::BTreeMap;

use crate::{language::lexer::token::Token, structs::question::Qtype};

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i32),
    Str(String),
    Qtype(Qtype),
    Object(Box<BTreeMap<String, Literal>>),
}

pub struct Parser {
    index: usize,
    tokens: Vec<Token>,
    pub properties: BTreeMap<String, Literal>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            index: 0,
            tokens,
            properties: BTreeMap::new(),
        }
    }

    pub fn parser(&mut self) {
        while self.index < self.tokens.len() {
            self.parse_statement();
        }
    }

    fn parse_statement(&mut self) {
        match self.tokens[self.index] {
            Token::Identifier(_) => {
                self.parse_expression();
            }
            _ => {}
        }
        self.index += 1;
    }

    fn parse_expression(&mut self) {
        let identifier: String = self.tokens[self.index].as_identifier().unwrap();
        self.index += 1;

        match self.tokens[self.index] {
            Token::Equals | Token::Colons => {}
            _ => {}
        }

        self.index += 1;

        match &self.tokens[self.index] {
            Token::ValueInt(value) => {
                self.properties
                    .insert(identifier, Literal::Int(value.clone()));
            }

            Token::Qtype(value) => {
                self.properties
                    .insert(identifier, Literal::Qtype(value.clone()));
            }

            Token::Guillemet => {
                self.index += 1;

                match &self.tokens[self.index] {
                    Token::ValueString(value) => {
                        self.properties
                            .insert(identifier, Literal::Str(value.clone()));

                        self.index += 1;
                    }
                    _ => {}
                }

                if self.tokens[self.index] != Token::Guillemet {}
            }

            Token::OpenBrace => {
                let object: BTreeMap<String, Literal> = self.parse_block();
                self.properties
                    .insert(identifier, Literal::Object(Box::new(object)));
            }

            _ => {}
        }
    }

    fn parse_block(&mut self) -> BTreeMap<String, Literal> {
        let mut identifier: String = String::new();
        self.index += 1;
        let mut literal: BTreeMap<String, Literal> = BTreeMap::new();

        while self.tokens[self.index] != Token::CloseBrace {
            match &self.tokens[self.index] {
                Token::OpenBrace => {
                    let object: BTreeMap<String, Literal> = self.parse_block();
                    literal.insert(identifier.clone(), Literal::Object(Box::new(object)));
                }

                Token::Identifier(value) => {
                    identifier = value.clone();
                }

                Token::Colons => {}

                Token::ValueInt(value) => {
                    literal.insert(identifier.clone(), Literal::Int(value.clone()));
                }

                Token::Qtype(value) => {
                    literal.insert(identifier.clone(), Literal::Qtype(value.clone()));
                }

                _ => {}
            }

            self.index += 1;
        }

        return literal;
    }
}
