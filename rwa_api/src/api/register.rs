use crate::db::{User, UserDto, UserResponse};
use crate::State;
use serde::Deserialize;
use sqlx::query_as;
use tide::{Body, Request, Response, Result, StatusCode};

#[derive(Debug, Deserialize)]
struct RegisterPayload {
    user: RegisterPayloadUser,
}

#[derive(Debug, Deserialize)]
struct RegisterPayloadUser {
    email: String,
    password: String,
    username: String,
}

pub async fn register(mut req: Request<State>) -> Result {
    let payload: RegisterPayload = req.body_json().await?;
    let state = req.state();

    let hash = bcrypt::hash(payload.user.password, bcrypt::DEFAULT_COST)?;

    let user: User = query_as!(
        User,
        r#"
            INSERT INTO "users" ("email", "password", "username") 
            VALUES ($1, $2, $3) 
            RETURNING *
        "#,
        payload.user.email,
        hash,
        payload.user.username
    )
    .fetch_one(&state.db_pool)
    .await?;

    let token = state.jwt.sign(&user)?;
    let mut res = Response::new(StatusCode::Created);
    let body = UserResponse {
        user: UserDto::with_token(&user, &token),
    };
    res.set_body(Body::from_json(&body)?);

    Ok(res)
}
