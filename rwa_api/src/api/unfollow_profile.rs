use crate::db::{ProfileResponse, User};
use crate::State;
use sqlx::{query, query_as};
use tide::{Body, Request, Response, Result, StatusCode};

pub async fn unfollow_profile(req: Request<State>) -> Result {
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
        "DELETE FROM users_followers WHERE follower_id = $1 AND leader_id = $2",
        current_user_id,
        user.id
    )
    .execute(&state.db_pool)
    .await?;

    let body = ProfileResponse {
        profile: (&user).into(),
    };
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&body)?);

    Ok(res)
}
