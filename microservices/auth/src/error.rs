use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Wrong credentials")]
    WrongCredentials,

    #[error("Failed to create token")]
    TokenCreateFailed,
}
