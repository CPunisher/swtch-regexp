use super::token::Token;

pub struct Lexer<T>
where
    T: Iterator<Item = char>,
{
    iter: T,
}

impl<T> Lexer<T>
where
    T: Iterator<Item = char>,
{
    pub fn new(iter: T) -> Self {
        Lexer { iter }
    }
}

impl<T> Iterator for Lexer<T>
where
    T: Iterator<Item = char>,
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some('|') => Some(Token::Alternate),
            Some('?') => Some(Token::ZeroOrOne),
            Some('*') => Some(Token::ZeroOrMore),
            Some('+') => Some(Token::OneOrMore),
            Some('(') => Some(Token::LeftBracket),
            Some(')') => Some(Token::RightBracket),
            Some(c) => Some(Token::Char(c)),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::new("(a|b)*a?".chars());
        assert_eq!(lexer.next(), Some(Token::LeftBracket));
        assert_eq!(lexer.next(), Some(Token::Char('a')));
        assert_eq!(lexer.next(), Some(Token::Alternate));
        assert_eq!(lexer.next(), Some(Token::Char('b')));
        assert_eq!(lexer.next(), Some(Token::RightBracket));
        assert_eq!(lexer.next(), Some(Token::ZeroOrMore));
        assert_eq!(lexer.next(), Some(Token::Char('a')));
        assert_eq!(lexer.next(), Some(Token::ZeroOrOne));
        assert_eq!(lexer.next(), None);
    }
}
