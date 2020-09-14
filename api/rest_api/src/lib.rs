mod article;
mod comment;
mod filters;
mod profile;
mod state;
mod tag;
mod user;

pub use conduit::config::Config;
pub use filters::state::WarpState;
pub use hyper;
pub use state::State;
pub use warp;
use warp::{Filter, Rejection, Reply};

pub struct Server;

impl Server {
    pub fn new(state: WarpState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        let routes = tag::routes(state.clone())
            .or(comment::routes(state.clone()))
            .or(user::routes(state.clone()))
            .or(profile::routes(state.clone()))
            .or(article::routes(state.clone()));

        // Middlewares
        routes
            .with(warp::cors().allow_any_origin())
            .with(warp::compression::brotli())
    }
}
