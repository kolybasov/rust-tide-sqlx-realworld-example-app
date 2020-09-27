use server::warp;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RestError {
    #[error(transparent)]
    ConduitError(#[from] conduit::ConduitError),
    #[error(transparent)]
    ServerError(#[from] server::ServerError),
}

impl warp::reject::Reject for RestError {}

impl From<RestError> for warp::Rejection {
    fn from(err: RestError) -> Self {
        warp::reject::custom(err)
    }
}
