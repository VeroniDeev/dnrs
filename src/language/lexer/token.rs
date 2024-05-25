use crate::structs::question::Qtype;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    Equals,
    OpenBrace,
    Colons,
    CloseBrace,
    Guillemet,
    ValueString(String),
    ValueInt(i32),
    Qtype(Qtype),
}

impl Token {
    pub fn as_identifier(&self) -> Option<String> {
        match self {
            Self::Identifier(value) => return Some(value.clone()),
            _ => None,
        }
    }
}
