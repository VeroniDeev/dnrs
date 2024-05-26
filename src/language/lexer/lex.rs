use crate::structs::question::Qtype;

use super::token::Token;

pub struct Lexer {
    line: usize,
    column: usize,
    pub tokens: Vec<Token>,
    pub source: String,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            line: 1,
            column: 1,
            tokens: Vec::new(),
            source,
        }
    }

    pub fn lexer(&mut self) {
        let mut token = String::new();
        let mut openbrace = false;

        for c in self.source.chars() {
            let char: String = c.to_string();
            self.column += 1;

            if openbrace && char != "\"" {
                token.push(c);
                continue;
            } else if openbrace && char == "\"" {
                self.tokens.push(Token::ValueString(token.clone()));
                self.tokens.push(Token::Guillemet);
                openbrace = false;
                token = String::new();
                continue;
            }

            match char.as_str() {
                ":" => {
                    if !token.is_empty() {
                        self.tokens.push(Token::Identifier(token));
                    }
                    self.tokens.push(Token::Colons);
                    token = String::new();
                }
                "=" => {
                    self.tokens.push(Token::Equals);
                    token = String::new();
                }
                "{" => self.tokens.push(Token::OpenBrace),
                "}" => self.tokens.push(Token::CloseBrace),
                "\"" => {
                    self.tokens.push(Token::Guillemet);
                    openbrace = true;
                    continue;
                }
                " " => {
                    if !token.is_empty() {
                        let token_type = self.check_type(token.clone());
                        self.tokens.push(token_type);
                    }
                    token = String::new();
                }
                "\t" => {}
                "\n" => {
                    if !token.is_empty() {
                        let token_type = self.check_type(token.clone());
                        self.tokens.push(token_type);
                    }
                    token = String::new();
                    self.column = 1;
                    self.line += 1;
                }
                _ => token.push(c),
            }

            match token.as_str() {
                "domain" => {
                    self.tokens.push(Token::Identifier(token));
                    token = String::new();
                }
                "ttl" => {
                    self.tokens.push(Token::Identifier(token));
                    token = String::new();
                }
                "type" => {
                    self.tokens.push(Token::Identifier(token));
                    token = String::new();
                }
                "subdomain" => {
                    self.tokens.push(Token::Identifier(token));
                    token = String::new();
                }
                "ip" => {
                    self.tokens.push(Token::Identifier(token));
                    token = String::new();
                }
                _ => {}
            }
        }
    }

    fn check_type(&self, token: String) -> Token {
        if let Ok(val) = token.parse::<i32>() {
            return Token::ValueInt(val);
        } else {
            return Token::Qtype(Qtype::with_string(token));
        }
    }
}
