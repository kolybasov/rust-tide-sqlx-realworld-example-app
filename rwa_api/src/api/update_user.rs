use crate::db::{User, UserDto, UserResponse, UserUpdate};
use crate::State;
use serde::Deserialize;
use sqlx::query_as;
use tide::{Body, Request, Response, Result, StatusCode};

#[derive(Deserialize, Debug)]
struct UpdateUserPayload {
    user: UserUpdate,
}

pub async fn update_user(mut req: Request<State>) -> Result {
    let user = req.ext_mut::<User>().unwrap().clone();
    let payload: UpdateUserPayload = req.body_json().await?;
    let state = req.state();

    let user_id = user.id;
    let token = state.jwt.sign(&user)?;
    let payload = payload.user.update(user)?;

    let user = query_as!(
        User,
        r#"
            UPDATE users 
            SET email = $2,
                username = $3,
                password = $4,
                image = $5,
                bio = $6
            WHERE id = $1
            RETURNING *
        "#,
        user_id,
        payload.email,
        payload.username,
        payload.password,
        payload.image,
        payload.bio
    )
    .fetch_one(&state.db_pool)
    .await?;

    let body = UserResponse {
        user: UserDto::with_token(user, token),
    };

    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&body)?);

    Ok(res)
}
