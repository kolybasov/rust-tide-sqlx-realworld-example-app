use crate::db::{User, UserDto, UserResponse};
use crate::State;
use serde::Deserialize;
use sqlx::query_as;
use tide::{Body, Request, Response, Result, StatusCode};

#[derive(Debug, Deserialize)]
struct LoginPayload {
    user: LoginPayloadUser,
}

#[derive(Debug, Deserialize)]
struct LoginPayloadUser {
    email: String,
    password: String,
}

pub async fn login(mut req: Request<State>) -> Result {
    let payload: LoginPayload = req.body_json().await?;
    let state = req.state();

    let user: User = query_as!(
        User,
        r#"SELECT * FROM users WHERE email = $1"#,
        payload.user.email
    )
    .fetch_one(&state.db_pool)
    .await?;

    let is_valid = bcrypt::verify(payload.user.password, &user.password)?;

    Ok(if is_valid {
        let mut res = Response::new(StatusCode::Ok);

        let token = state.jwt.sign(&user)?;
        let body = UserResponse {
            user: UserDto::with_token(user, token),
        };
        res.set_body(Body::from_json(&body)?);

        res
    } else {
        Response::new(StatusCode::Unauthorized)
    })
}
