use thiserror::Error;

#[derive(Error, Debug)]
pub enum NfaError {
    #[error("invalid regex: {0}")]
    InvalidRegex(String),
}
