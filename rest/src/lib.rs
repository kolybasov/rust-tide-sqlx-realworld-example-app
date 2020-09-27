mod article;
mod comment;
mod profile;
mod tag;
mod user;

use server::{warp, ServerState};
use warp::{Filter, Rejection, Reply};

pub struct Rest;

impl Rest {
    pub fn new(state: ServerState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        let routes = tag::routes(state.clone())
            .or(comment::routes(state.clone()))
            .or(user::routes(state.clone()))
            .or(profile::routes(state.clone()))
            .or(article::routes(state.clone()))
            .boxed();

        // Middlewares
        routes
    }
}
