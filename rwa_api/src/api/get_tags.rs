use crate::db::{Tag, TagsResponse};
use crate::State;
use sqlx::query_as;
use tide::{Body, Request, Response, Result, StatusCode};

pub async fn get_tags(req: Request<State>) -> Result {
    let state = req.state();
    let tags = query_as!(Tag, "SELECT * FROM tags")
        .fetch_all(&state.db_pool)
        .await?;

    let body = TagsResponse {
        tags: tags.into_iter().map(|tag_row| tag_row.tag).collect(),
    };

    Ok(Response::builder(StatusCode::Ok)
        .body(Body::from_json(&body)?)
        .build())
}
