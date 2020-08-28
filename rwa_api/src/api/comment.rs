use crate::db::{CommentResponse, CommentsResponse, User};
use crate::services::{CommentService, CreateCommentParams};
use crate::State;
use serde::Deserialize;
use tide::{Body, Request, Response, Result, StatusCode};

#[derive(Debug, Deserialize)]
struct CreateCommentPayload {
    comment: CreateCommentParams,
}

pub async fn create_comment(mut req: Request<State>) -> Result {
    let payload: CreateCommentPayload = req.body_json().await?;
    let state = req.state();
    let slug: String = req.param("slug")?;
    let author = req.ext::<User>().unwrap();

    let comment = CommentService::new(&state.db_pool)
        .create_comment(&payload.comment, &slug, author.id)
        .await?;

    let body = Body::from_json(&CommentResponse::from(comment))?;
    Ok(Response::builder(StatusCode::Created).body(body).build())
}

pub async fn delete_comment(req: Request<State>) -> Result {
    let state = req.state();
    let comment_id: i32 = req.param("id")?;
    let current_user_id = req.ext::<User>().unwrap().id;

    CommentService::new(&state.db_pool)
        .delete_comment(comment_id, current_user_id)
        .await?;

    Ok(Response::new(StatusCode::NoContent))
}

pub async fn get_comments(req: Request<State>) -> Result {
    let state = req.state();
    let slug: String = req.param("slug")?;
    let current_user_id = req.ext::<User>().map(|user| user.id).or(None);

    let comments = CommentService::new(&state.db_pool)
        .get_comments(&slug, current_user_id)
        .await?;

    let body = Body::from_json(&CommentsResponse::from(comments))?;
    Ok(Response::builder(StatusCode::Ok).body(body).build())
}
