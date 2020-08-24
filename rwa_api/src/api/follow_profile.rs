use crate::db::{ProfileDto, ProfileResponse, User};
use crate::State;
use sqlx::{query, query_as};
use tide::{Body, Request, Response, Result, StatusCode};

pub async fn follow_profile(req: Request<State>) -> Result {
    let state = req.state();
    let username: String = req.param("username")?;
    let current_user_id = req.ext::<User>().unwrap().id;

    let user = query_as!(
        User,
        r#"SELECT *  FROM users WHERE username = $1"#,
        username,
    )
    .fetch_one(&state.db_pool)
    .await?;

    query!(
        "INSERT INTO users_followers (follower_id, leader_id) VALUES ($1, $2)",
        current_user_id,
        user.id
    )
    .execute(&state.db_pool)
    .await?;

    let mut profile = ProfileDto::from(&user);
    profile.following = true;

    let body = ProfileResponse { profile };
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&body)?);

    Ok(res)
}
