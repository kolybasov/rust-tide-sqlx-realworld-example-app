use crate::services::{
    LoginParams, RegisterParams, UpdateUserParams, User, UserDto, UserResponse, UserService,
};
use crate::State;
use serde::Deserialize;
use tide::{Body, Request, Response, Result, StatusCode};

pub async fn get_user(req: Request<State>) -> Result {
    let state = req.state();
    let user = req.ext::<User>().unwrap();

    let token = state.jwt.sign(user)?;
    let body = Body::from_json(&UserResponse::from(UserDto::with_token(
        user.clone(),
        token,
    )))?;
    Ok(Response::builder(StatusCode::Ok).body(body).build())
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    user: LoginParams,
}

pub async fn login(mut req: Request<State>) -> Result {
    let payload: LoginPayload = req.body_json().await?;
    let state = req.state();

    let user = UserService::new(&state.db_pool)
        .login(&payload.user, &state.jwt)
        .await?;

    let body = Body::from_json(&UserResponse::from(user))?;
    Ok(Response::builder(StatusCode::Ok).body(body).build())
}

#[derive(Debug, Deserialize)]
struct RegisterPayload {
    user: RegisterParams,
}

pub async fn register(mut req: Request<State>) -> Result {
    let payload: RegisterPayload = req.body_json().await?;
    let state = req.state();

    let user = UserService::new(&state.db_pool)
        .register(&payload.user, &state.jwt)
        .await?;

    let body = Body::from_json(&UserResponse::from(user))?;
    Ok(Response::builder(StatusCode::Created).body(body).build())
}

#[derive(Deserialize, Debug)]
struct UpdateUserPayload {
    user: UpdateUserParams,
}

pub async fn update_user(mut req: Request<State>) -> Result {
    let payload: UpdateUserPayload = req.body_json().await?;
    let user = req.ext::<User>().unwrap();
    let state = req.state();

    let updated_user = UserService::new(&state.db_pool)
        .update_user(&payload.user, &user, &state.jwt)
        .await?;

    let body = Body::from_json(&UserResponse::from(updated_user))?;
    Ok(Response::builder(StatusCode::Ok).body(body).build())
}
