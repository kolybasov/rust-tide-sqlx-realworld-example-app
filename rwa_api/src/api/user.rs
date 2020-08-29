use crate::services::{LoginParams, RegisterParams, UpdateUserParams, User, UserDto, UserService};
use crate::State;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use tide::{Body, Request, Response, Result, StatusCode};

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub user: UserDto,
}

impl TryFrom<UserDto> for Body {
    type Error = tide::Error;

    fn try_from(user: UserDto) -> Result<Self> {
        let res = UserResponse { user };
        Body::from_json(&res)
    }
}

pub async fn get_user(req: Request<State>) -> Result {
    let state = req.state();
    let user = req.ext::<User>().unwrap();

    let token = state.jwt.sign(user)?;
    let user_dto = UserDto::with_token(user.clone(), token);

    Ok(Response::builder(StatusCode::Ok)
        .body(Body::try_from(user_dto)?)
        .build())
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

    Ok(Response::builder(StatusCode::Ok)
        .body(Body::try_from(user)?)
        .build())
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

    Ok(Response::builder(StatusCode::Created)
        .body(Body::try_from(user)?)
        .build())
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

    Ok(Response::builder(StatusCode::Ok)
        .body(Body::try_from(updated_user)?)
        .build())
}
