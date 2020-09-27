use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error(transparent)]
    JWTError(#[from] jsonwebtoken::errors::Error),
    #[error("unauthorized")]
    Unauthorized,
    #[error(transparent)]
    ConduitError(#[from] conduit::ConduitError),
    #[error(transparent)]
    HyperError(#[from] hyper::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, ServerError>;

impl warp::reject::Reject for ServerError {}
