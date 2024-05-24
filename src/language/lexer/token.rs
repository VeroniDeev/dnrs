use crate::structs::question::Qtype;

#[derive(Debug)]
pub enum Token {
    Domain,
    TTL,
    Type,
    Subdomain,
    Equals,
    OpenBrace,
    Colons,
    CloseBrace,
    Guillemet,
    ValueString(String),
    ValueInt(i32),
    NewLines,
    Qtype(Qtype),
    Key(String),
}
