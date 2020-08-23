use crate::db::{CommentDto, CommentsResponse, ProfileDto, User};
use crate::State;
use sqlx::query;
use tide::{Body, Request, Response, Result, StatusCode};

pub async fn get_comments(req: Request<State>) -> Result {
    let state = req.state();
    let slug: String = req.param("slug")?;
    let current_user_id = req.ext::<User>().map(|user| user.id).or(None);

    let comments = query!(
        r#"
            SELECT c.*,
                   u.username "author_username!",
                   u.bio "author_bio",
                   u.image "author_image",
                   (uf.follower_id IS NOT NULL) "author_following!"
            FROM comments c
            INNER JOIN articles a ON a.id = c.article_id
            INNER JOIN users u 
                LEFT JOIN users_followers uf ON uf.leader_id = u.id AND uf.follower_id = $2
                ON u.id = c.author_id
            WHERE a.slug = $1
        "#,
        slug,
        current_user_id,
    )
    .fetch_all(&state.db_pool)
    .await?;

    let body = CommentsResponse {
        comments: comments
            .into_iter()
            .map(|comment| CommentDto {
                id: comment.id,
                body: comment.body,
                created_at: comment.created_at,
                updated_at: comment.updated_at,
                author: ProfileDto {
                    username: comment.author_username,
                    bio: comment.author_bio,
                    image: comment.author_image,
                    following: comment.author_following,
                },
            })
            .collect(),
    };

    Ok(Response::builder(StatusCode::Ok)
        .body(Body::from_json(&body)?)
        .build())
}
