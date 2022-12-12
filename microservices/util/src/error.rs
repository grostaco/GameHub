use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Token is invalid: {0:?}")]
    InvalidToken(jsonwebtoken::errors::Error),

    #[error("Failed to create token")]
    TokenCreateFailed,
}
