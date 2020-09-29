mod article;
mod error;
mod render;

pub use error::WebError;
pub use render::render;
use server::{warp, ServerState};
use std::sync::Arc;
use warp::{Filter, Rejection, Reply};

pub struct Web;

impl Web {
    pub fn new(state: ServerState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        let static_dir = warp::path("static").and(warp::fs::dir("web/static"));
        let articles = article::routes(Arc::clone(&state));

        static_dir.or(articles)
    }
}
