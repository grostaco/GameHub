use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Token is invalid")]
    InvalidToken,

    #[error("Failed to create token")]
    TokenCreateFailed,
}
