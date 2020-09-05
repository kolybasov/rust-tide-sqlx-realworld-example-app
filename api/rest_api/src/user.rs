use crate::State;
use conduit::{LoginParams, RegisterParams, UpdateUserParams, User, UserDto, UserService};
use serde::{Deserialize, Serialize};
use tide::{Body, Request, Response, Result, StatusCode};

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub user: UserDto,
}

impl From<UserDto> for UserResponse {
    fn from(user: UserDto) -> Self {
        UserResponse { user }
    }
}

pub async fn get_user(req: Request<State>) -> Result {
    let state = req.state();
    let user = req.ext::<User>().unwrap();

    let token = state.jwt.sign(user)?;
    let user_dto = UserDto::with_token(user.clone(), token);

    let res = UserResponse::from(user_dto);
    Ok(Body::from_json(&res)?.into())
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

    let res = UserResponse::from(user);
    Ok(Body::from_json(&res)?.into())
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

    let body = UserResponse::from(user);
    Ok(Response::builder(StatusCode::Created)
        .body(Body::from_json(&body)?)
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

    let res = UserResponse::from(updated_user);
    Ok(Body::from_json(&res)?.into())
}
