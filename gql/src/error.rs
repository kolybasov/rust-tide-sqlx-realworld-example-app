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
