use thiserror::Error;

use super::token::Token;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unexpected token: {0}")]
    UnexpectedToken(Token),
    #[error("Unexpected EOF")]
    UnexpectedEOF,
}
