mod article;
mod comment;
mod profile;
mod tag;
mod user;

pub use conduit::config::Config;
use server::{warp, ServerState};
use warp::{http::Method, Filter, Rejection, Reply};

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
            .with(
                warp::cors()
                    .allow_any_origin()
                    .allow_methods(&[Method::GET, Method::POST, Method::PUT, Method::DELETE])
                    .allow_headers(vec!["content-type", "authorization"]),
            )
            .with(warp::compression::brotli())
            .boxed()
    }
}
