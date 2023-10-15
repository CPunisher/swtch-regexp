use std::iter::Peekable;

use super::{ast::*, error::ParseError, token::Token};

pub struct Parser<T>
where
    T: Iterator<Item = Token>,
{
    tokens: Peekable<T>,
}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    pub fn new(tokens: T) -> Self {
        Parser {
            tokens: tokens.peekable(),
        }
    }

    fn advance(&mut self) -> ParseResult<Token> {
        self.tokens.next().ok_or(ParseError::UnexpectedEOF)
    }

    fn expected(&mut self, token: Token) -> ParseResult<Token> {
        self.advance().and_then(|token2| {
            if token == token2 {
                Ok(token2)
            } else {
                Err(ParseError::UnexpectedToken(token2))
            }
        })
    }
}

type ParseResult<T> = Result<T, ParseError>;

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    pub fn parse_group(&mut self) -> ParseResult<Group> {
        let mut exprs = vec![];
        while let Some(Token::LeftBracket) = self.tokens.peek() {
            self.expected(Token::LeftBracket)?;
            exprs.push(self.parse_expr()?);
            self.expected(Token::RightBracket)?;
        }
        Ok(Group(exprs))
    }

    pub fn parse_expr(&mut self) -> ParseResult<Expr> {
        let mut factor_conns = vec![self.parse_factor_conn()?];
        while let Some(Token::Alternate) = self.tokens.peek() {
            self.advance()?;
            let next_factor_conn = self.parse_factor_conn()?;
            factor_conns.push(next_factor_conn);
        }
        Ok(Expr(factor_conns))
    }

    pub fn parse_factor_conn(&mut self) -> ParseResult<FactorConn> {
        let mut factors = vec![self.parse_factor()?];
        while let Some(&token) = self.tokens.peek() {
            if token != Token::Alternate && token != Token::RightBracket {
                let next_factor = self.parse_factor()?;
                factors.push(next_factor);
            } else {
                break;
            }
        }
        Ok(FactorConn(factors))
    }

    pub fn parse_factor(&mut self) -> ParseResult<Factor> {
        let term = self.parse_term()?;
        match self.tokens.peek() {
            Some(token) => match token {
                Token::ZeroOrOne => {
                    self.advance()?;
                    Ok(Factor::ZeroOrOne(term))
                }
                Token::ZeroOrMore => {
                    self.advance()?;
                    Ok(Factor::ZeroOrMore(term))
                }
                Token::OneOrMore => {
                    self.advance()?;
                    Ok(Factor::OneOrMore(term))
                }
                _ => Ok(Factor::Plain(term)),
            },
            None => Ok(Factor::Plain(term)),
        }
    }

    pub fn parse_term(&mut self) -> ParseResult<Term> {
        let token = self.advance()?;
        match token {
            Token::LeftBracket => {
                let group = self.parse_group()?;
                Ok(Term::Group(group))
            }
            Token::Char(c) => Ok(Term::Char(c)),
            _ => Err(ParseError::UnexpectedToken(token)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::vm::compiler::lexer::Lexer;

    use super::*;

    #[test]
    fn test_parser() {
        let lexer = Lexer::new("(a(b|c)*d)".chars());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_group();
        assert!(program.is_ok())
    }
}
