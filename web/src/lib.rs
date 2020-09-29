mod article;
mod error;
mod user;

use askama::Template;
pub use error::WebError;
use server::{warp, ServerState};
use std::sync::Arc;
use warp::{Filter, Rejection, Reply};

pub struct Web;

impl Web {
    pub fn new(state: ServerState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        let static_dir = warp::path("static").and(warp::fs::dir("web/static"));

        static_dir
            .or(article::routes(Arc::clone(&state)))
            .or(user::routes(Arc::clone(&state)))
    }
}

pub fn render<T: Template>(template: &T) -> Result<impl Reply, Rejection> {
    let html = template.render().map_err(WebError::from)?;
    Ok(warp::reply::html(html))
}
