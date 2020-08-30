use crate::services::{CommentDto, CommentService, CreateCommentParams, User};
use crate::State;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use tide::{Body, Request, Response, Result, StatusCode};

#[derive(Serialize, Debug)]
pub struct CommentResponse {
    pub comment: CommentDto,
}

impl TryFrom<CommentDto> for Body {
    type Error = tide::Error;

    fn try_from(comment: CommentDto) -> Result<Self> {
        let res = CommentResponse { comment };
        Body::from_json(&res)
    }
}

#[derive(Serialize, Debug)]
pub struct CommentsResponse {
    pub comments: Vec<CommentDto>,
}

impl From<Vec<CommentDto>> for CommentsResponse {
    fn from(comments: Vec<CommentDto>) -> Self {
        CommentsResponse { comments }
    }
}

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

    Ok(Response::builder(StatusCode::Created)
        .body(Body::try_from(comment)?)
        .build())
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
