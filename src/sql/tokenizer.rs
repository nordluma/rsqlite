#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Select,
    As,
    From,
    Star,
    Comma,
    SemiColon,
    Identifier(String),
}

impl Token {
    pub fn as_identifier(&self) -> Option<&str> {
        match self {
            Token::Identifier(ident) => Some(ident),
            _ => None,
        }
    }
}
