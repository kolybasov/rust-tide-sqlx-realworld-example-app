use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConduitError {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error(transparent)]
    BcryptError(#[from] bcrypt::BcryptError),
    #[error("invalid password")]
    InvalidPassword,
    #[error("cannot create token")]
    CreateTokenError,
}

pub type Result<T> = std::result::Result<T, ConduitError>;
