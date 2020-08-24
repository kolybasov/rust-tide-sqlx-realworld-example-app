use crate::db::{Comment, CommentDto, CommentResponse, ProfileDto, User};
use crate::State;
use serde::Deserialize;
use sqlx::{query, query_as};
use tide::{Body, Request, Response, Result, StatusCode};

#[derive(Debug, Deserialize)]
struct CreateCommentPayload {
    comment: CreateCommentPayloadComment,
}

#[derive(Debug, Deserialize)]
struct CreateCommentPayloadComment {
    body: String,
}

pub async fn create_comment(mut req: Request<State>) -> Result {
    let payload: CreateCommentPayload = req.body_json().await?;
    let state = req.state();
    let slug: String = req.param("slug")?;
    let author = req.ext::<User>().unwrap();

    let article = query!("SELECT id FROM articles WHERE slug = $1", slug)
        .fetch_one(&state.db_pool)
        .await?;

    let comment = query_as!(
        Comment,
        "INSERT INTO comments (body, article_id, author_id) VALUES ($1, $2, $3) RETURNING *",
        payload.comment.body,
        article.id,
        author.id
    )
    .fetch_one(&state.db_pool)
    .await?;

    let body = CommentResponse {
        comment: CommentDto {
            id: comment.id,
            body: &comment.body,
            created_at: comment.created_at,
            updated_at: comment.updated_at,
            author: ProfileDto {
                username: &author.username,
                bio: author.bio.as_deref(),
                image: author.image.as_deref(),
                following: false,
            },
        },
    };

    Ok(Response::builder(StatusCode::Created)
        .body(Body::from_json(&body)?)
        .build())
}
