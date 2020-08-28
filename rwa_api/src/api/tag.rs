use crate::db::TagsResponse;
use crate::services::TagService;
use crate::State;
use tide::{Body, Request, Response, Result, StatusCode};

pub async fn get_tags(req: Request<State>) -> Result {
    let state = req.state();
    let tags = TagService::new(&state.db_pool).get_tags().await?;

    let body = Body::from_json(&TagsResponse::from(tags))?;
    Ok(Response::builder(StatusCode::Ok).body(body).build())
}
