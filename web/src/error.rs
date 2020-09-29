use server::warp;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WebError {
    #[error(transparent)]
    ConduitError(#[from] conduit::ConduitError),
    #[error(transparent)]
    ServerError(#[from] server::ServerError),
    #[error(transparent)]
    AskamaError(#[from] askama::Error),
}

impl warp::reject::Reject for WebError {}

impl From<WebError> for warp::Rejection {
    fn from(err: WebError) -> Self {
        warp::reject::custom(err)
    }
}
