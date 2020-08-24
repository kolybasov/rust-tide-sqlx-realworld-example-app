use crate::db::{ProfileDto, ProfileResponse, User};
use crate::State;
use sqlx::query;
use tide::{Body, Request, Response, Result, StatusCode};

pub async fn get_profile(req: Request<State>) -> Result {
    let state = req.state();
    let username: String = req.param("username")?;
    let current_user_id = req.ext::<User>().map(|user| user.id).or(None);

    let profile = query!(
        r#"
            SELECT username, bio, image, (uf.leader_id IS NOT NULL) "following!"  FROM users u
            LEFT JOIN users_followers uf ON uf.leader_id = u.id AND uf.follower_id = $2
            WHERE u.username = $1
        "#,
        username,
        current_user_id,
    )
    .fetch_one(&state.db_pool)
    .await?;

    let body = ProfileResponse {
        profile: ProfileDto {
            username: &profile.username,
            bio: profile.bio.as_deref(),
            image: profile.image.as_deref(),
            following: profile.following,
        },
    };
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&body)?);

    Ok(res)
}
