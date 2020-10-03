mod article;
mod comment;
mod error;
mod profile;
mod tag;
mod user;

use error::{handle_rejection, RestError};
use server::{warp, ServerState};
use std::sync::Arc;
use warp::{Filter, Rejection, Reply};

pub struct Rest;

impl Rest {
    pub fn routes(
        state: ServerState,
    ) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        let rest_routes = tag::routes(Arc::clone(&state))
            .or(comment::routes(Arc::clone(&state)))
            .or(user::routes(Arc::clone(&state)))
            .or(profile::routes(Arc::clone(&state)))
            .or(article::routes(Arc::clone(&state)))
            .recover(handle_rejection)
            .boxed();

        warp::path("api").and(rest_routes)
    }
}
