use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error(transparent)]
    BcryptError(#[from] bcrypt::BcryptError),
    #[error(transparent)]
    JWTError(#[from] jsonwebtoken::errors::Error),
    #[error("invalid password")]
    InvalidPassword,
    #[error("unauthorized")]
    Unauthorized,
}

pub type Result<T> = std::result::Result<T, Error>;

impl warp::reject::Reject for Error {}

impl From<Error> for warp::Rejection {
    fn from(err: Error) -> Self {
        warp::reject::custom(err)
    }
}
