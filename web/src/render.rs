use crate::WebError;
use askama::Template;
use server::warp;
use warp::{Rejection, Reply};

pub fn render<T: Template>(template: &T) -> Result<impl Reply, Rejection> {
    let html = template.render().map_err(WebError::from)?;
    Ok(warp::reply::html(html))
}
