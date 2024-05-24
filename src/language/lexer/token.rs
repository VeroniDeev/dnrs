use crate::structs::question::Qtype;

#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Equals,
    OpenBrace,
    Colons,
    CloseBrace,
    Guillemet,
    ValueString(String),
    ValueInt(i32),
    NewLines,
    Qtype(Qtype),
}
