use server::warp;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GqlError {
    #[error(transparent)]
    ConduitError(#[from] conduit::ConduitError),
    #[error(transparent)]
    ServerError(#[from] server::ServerError),
}

impl warp::reject::Reject for GqlError {}

impl From<GqlError> for warp::Rejection {
    fn from(err: GqlError) -> Self {
        warp::reject::custom(err)
    }
}
