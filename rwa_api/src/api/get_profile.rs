use crate::db::{ProfileDto, ProfileResponse, User};
use crate::State;
use sqlx::query_as;
use tide::{Body, Request, Response, Result, StatusCode};

pub async fn get_profile(req: Request<State>) -> Result {
    let state = req.state();
    let username: String = req.param("username")?;
    let current_user_id = req.ext::<User>().map(|user| user.id).or(None);

    let profile = query_as!(
        ProfileDto,
        r#"
            SELECT username, bio, image, (uf.following_id IS NOT NULL) "following!"  FROM users u
            LEFT JOIN users_followers uf ON uf.following_id = u.id AND uf.follower_id = $2
            WHERE u.username = $1
        "#,
        username,
        current_user_id
    )
    .fetch_one(&state.db_pool)
    .await?;

    let body = ProfileResponse { profile };
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&body)?);

    Ok(res)
}
