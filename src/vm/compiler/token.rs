use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Char(char),
    Alternate,
    ZeroOrOne,
    ZeroOrMore,
    OneOrMore,
    LeftBracket,
    RightBracket,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Char(c) => write!(f, "{}", c),
            Self::Alternate => write!(f, "|"),
            Self::ZeroOrOne => write!(f, "?"),
            Self::ZeroOrMore => write!(f, "*"),
            Self::OneOrMore => write!(f, "+"),
            Self::LeftBracket => write!(f, "("),
            Self::RightBracket => write!(f, ")"),
        }
    }
}
