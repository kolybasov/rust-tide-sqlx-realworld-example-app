use crate::db::{User, UserDto, UserResponse};
use crate::State;
use serde::Deserialize;
use sqlx::query_as;
use tide::{Body, Request, Response, Result, StatusCode};

#[derive(Deserialize, Debug)]
struct UpdateUserPayload {
    user: UpdateUserUser,
}

#[derive(Deserialize, Debug)]
pub struct UpdateUserUser {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub image: Option<String>,
    pub bio: Option<String>,
}

pub async fn update_user(mut req: Request<State>) -> Result {
    let payload: UpdateUserPayload = req.body_json().await?;
    let user = req.ext::<User>().unwrap();
    let state = req.state();

    let user_id = user.id;
    let token = state.jwt.sign(&user)?;

    let password = if let Some(new_password) = payload.user.password {
        let hash = bcrypt::hash(new_password, bcrypt::DEFAULT_COST)?;
        Some(hash)
    } else {
        None
    };

    let user = query_as!(
        User,
        r#"
            UPDATE users u
            SET email = COALESCE($2, u.email),
                username = COALESCE($3, u.username),
                password = COALESCE($4, u.password),
                image = $5,
                bio = $6
            WHERE id = $1
            RETURNING *
        "#,
        user_id,
        payload.user.email,
        payload.user.username,
        password,
        payload.user.image.as_deref().or(user.image.as_deref()),
        payload.user.bio.as_deref().or(user.bio.as_deref())
    )
    .fetch_one(&state.db_pool)
    .await?;

    let body = UserResponse {
        user: UserDto::with_token(&user, &token),
    };

    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&body)?);

    Ok(res)
}
