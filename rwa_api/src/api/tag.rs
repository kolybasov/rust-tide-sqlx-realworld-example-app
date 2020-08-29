use crate::services::TagService;
use crate::State;
use serde::Serialize;
use tide::{Body, Request, Response, Result, StatusCode};

pub async fn get_tags(req: Request<State>) -> Result {
    let state = req.state();
    let tags = TagService::new(&state.db_pool).get_tags().await?;

    let body = Body::from_json(&TagsResponse::from(tags))?;
    Ok(Response::builder(StatusCode::Ok).body(body).build())
}

#[derive(Debug, Serialize)]
pub struct TagsResponse {
    pub tags: Vec<String>,
}

impl From<Vec<String>> for TagsResponse {
    fn from(tags: Vec<String>) -> Self {
        TagsResponse { tags }
    }
}
