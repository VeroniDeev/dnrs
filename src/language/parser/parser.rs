use crate::{language::lexer::token::Token, structs::question::Qtype};
use std::collections::BTreeMap;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(String),
    UnexpectedEndOfInput,
    InvalidLiteral(String),
    MissingToken(String),
    InvalidSyntax(String),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i32),
    Str(String),
    Qtype(Qtype),
    Object(Box<BTreeMap<String, Literal>>),
}

impl Literal {
    pub fn as_int(&self) -> Option<i32> {
        match self {
            Literal::Int(value) => return Some(value.clone()),
            _ => return None,
        }
    }

    pub fn as_str(&self) -> Option<String> {
        match self {
            Literal::Str(value) => return Some(value.clone()),
            _ => return None,
        }
    }

    pub fn as_qtype(&self) -> Option<Qtype> {
        match self {
            Literal::Qtype(value) => return Some(value.clone()),
            _ => return None,
        }
    }

    pub fn as_object(&self) -> Option<Box<BTreeMap<String, Literal>>> {
        match self {
            Literal::Object(value) => return Some(value.clone()),
            _ => return None,
        }
    }
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

    pub fn parser(&mut self) -> Result<(), ParseError> {
        while self.index < self.tokens.len() {
            match self.parse_statement() {
                Ok(_) => {}
                Err(err) => return Err(err),
            }
        }
        Ok(())
    }

    fn parse_statement(&mut self) -> Result<(), ParseError> {
        match self.tokens[self.index] {
            Token::Identifier(_) => match self.parse_expression() {
                Ok(_) => {}
                Err(err) => return Err(err),
            },
            _ => {
                return Err(ParseError::MissingToken(String::from(
                    "We are exept to receive a name",
                )))
            }
        }
        self.index += 1;
        Ok(())
    }

    fn parse_expression(&mut self) -> Result<(), ParseError> {
        let identifier: String = self.tokens[self.index].as_identifier().unwrap();
        self.index += 1;

        match self.tokens[self.index] {
            Token::Equals | Token::Colons => {
                self.index += 1;
            }
            _ => {
                return Err(ParseError::MissingToken(String::from(
                    "We are except to receive a equal or colons",
                )))
            }
        }

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
                    _ => {
                        return Err(ParseError::InvalidLiteral(String::from(
                            "We are expect to receive a string",
                        )))
                    }
                }

                if self.tokens[self.index] != Token::Guillemet {}
            }

            Token::OpenBrace => match self.parse_block() {
                Ok(object) => {
                    self.properties
                        .insert(identifier, Literal::Object(Box::new(object)));
                }
                Err(err) => return Err(err),
            },

            _ => {
                return Err(ParseError::UnexpectedToken(String::from(
                    "Unexpected value",
                )))
            }
        }

        Ok(())
    }

    fn parse_block(&mut self) -> Result<BTreeMap<String, Literal>, ParseError> {
        let mut identifier: String = String::new();
        self.index += 1;
        let mut literal: BTreeMap<String, Literal> = BTreeMap::new();
        let mut is_brace_closed = false;

        while let Some(token) = self.tokens.get(self.index) {
            match token {
                Token::OpenBrace => match self.parse_block() {
                    Ok(object) => {
                        literal.insert(identifier.clone(), Literal::Object(Box::new(object)));
                    }
                    Err(_) => {
                        return Err(ParseError::InvalidSyntax(String::from("Invalid syntax")));
                    }
                },

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

                Token::CloseBrace => {
                    is_brace_closed = true;
                    break;
                }

                _ => {}
            }

            self.index += 1;
        }

        if is_brace_closed {
            Ok(literal)
        } else {
            Err(ParseError::MissingToken(String::from(
                "We are expecting a closing brace for the object",
            )))
        }
    }
}
