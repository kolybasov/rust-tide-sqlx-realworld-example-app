use crate::filters;
use conduit::{CommentDto, CommentService, CreateCommentParams, User};
use filters::state::{with_db, PgPool, WarpState};
use serde::{Deserialize, Serialize};
use warp::{Filter, Rejection, Reply};

pub fn routes(state: WarpState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    // GET /articles/:slug/comments
    let get_comments = warp::path!("articles" / String / "comments")
        .and(warp::get())
        .and(filters::auth::optional(state.clone()))
        .and(with_db(state.clone()))
        .and_then(get_comments_handler);

    // DELETE /articles/:slug/comments/:id
    let delete_comment = warp::path!("articles" / String / "comments" / i32)
        .and(warp::delete())
        .and(filters::auth(state.clone()))
        .and(with_db(state.clone()))
        .and_then(delete_comment_handler);

    // POST /articles/:slug/comments
    let create_comment = warp::path!("articles" / String / "comments")
        .and(warp::post())
        .and(warp::body::json())
        .and(filters::auth(state.clone()))
        .and(with_db(state.clone()))
        .and_then(create_comment_handler);

    get_comments.or(delete_comment).or(create_comment)
}

#[derive(Serialize, Debug)]
pub struct CommentResponse {
    pub comment: CommentDto,
}

impl From<CommentDto> for CommentResponse {
    fn from(comment: CommentDto) -> Self {
        CommentResponse { comment }
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

async fn create_comment_handler(
    slug: String,
    payload: CreateCommentPayload,
    author: User,
    db_pool: PgPool,
) -> Result<impl Reply, Rejection> {
    let comment = CommentService::new(&db_pool)
        .create_comment(&payload.comment, &slug, author.id)
        .await?;

    let body = CommentResponse::from(comment);
    Ok(warp::reply::with_status(
        warp::reply::json(&body),
        warp::http::StatusCode::CREATED,
    ))
}

async fn delete_comment_handler(
    _slug: String,
    comment_id: i32,
    user: User,
    db_pool: PgPool,
) -> Result<impl Reply, Rejection> {
    CommentService::new(&db_pool)
        .delete_comment(comment_id, user.id)
        .await?;

    Ok(warp::reply::with_status(
        warp::reply(),
        warp::http::StatusCode::NO_CONTENT,
    ))
}

async fn get_comments_handler(
    slug: String,
    user: Option<User>,
    db_pool: PgPool,
) -> Result<impl Reply, Rejection> {
    let current_user_id = user.map(|user| user.id).or(None);

    let comments = CommentService::new(&db_pool)
        .get_comments(&slug, current_user_id)
        .await?;

    Ok(warp::reply::json(&CommentsResponse::from(comments)))
}
