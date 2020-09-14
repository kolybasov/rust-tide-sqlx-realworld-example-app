use crate::{
    filters,
    filters::state::{with_state, WarpState},
};
use conduit::{
    error::Error, LoginParams, RegisterParams, UpdateUserParams, User, UserDto, UserService,
};
use serde::{Deserialize, Serialize};
use warp::{Filter, Rejection, Reply};

pub fn routes(state: WarpState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    // GET /user
    let get_user = warp::path!("user")
        .and(warp::get())
        .and(with_state(state.clone()))
        .and(filters::auth(state.clone()))
        .and_then(get_user_handler);

    // PUT /user
    let update_user = warp::path!("user")
        .and(warp::put())
        .and(warp::body::json())
        .and(with_state(state.clone()))
        .and(filters::auth(state.clone()))
        .and_then(update_user_handler);

    // POST /users/login
    let login = warp::path!("users" / "login")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_state(state.clone()))
        .and_then(login_handler);

    // POST /users
    let register = warp::path!("users")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_state(state.clone()))
        .and_then(register_handler);

    get_user.or(update_user).or(login).or(register)
}

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub user: UserDto,
}

impl From<UserDto> for UserResponse {
    fn from(user: UserDto) -> Self {
        UserResponse { user }
    }
}

async fn get_user_handler(state: WarpState, user: User) -> Result<impl Reply, Rejection> {
    let state = state.read().await;
    let token = state.jwt.sign(&user).map_err(Error::from)?;
    let user_dto = UserDto::with_token(user, token);
    Ok(warp::reply::json(&UserResponse::from(user_dto)))
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    user: LoginParams,
}

async fn login_handler(payload: LoginPayload, state: WarpState) -> Result<impl Reply, Rejection> {
    let state = state.read().await;

    let user = UserService::new(&state.db_pool)
        .login(&payload.user, &state.jwt)
        .await?;

    Ok(warp::reply::json(&UserResponse::from(user)))
}

#[derive(Debug, Deserialize)]
struct RegisterPayload {
    user: RegisterParams,
}

async fn register_handler(
    payload: RegisterPayload,
    state: WarpState,
) -> Result<impl Reply, Rejection> {
    let state = state.read().await;

    let user = UserService::new(&state.db_pool)
        .register(&payload.user, &state.jwt)
        .await?;

    let body = UserResponse::from(user);
    Ok(warp::reply::with_status(
        warp::reply::json(&body),
        warp::http::StatusCode::CREATED,
    ))
}

#[derive(Deserialize, Debug)]
struct UpdateUserPayload {
    user: UpdateUserParams,
}

async fn update_user_handler(
    payload: UpdateUserPayload,
    state: WarpState,
    user: User,
) -> Result<impl Reply, Rejection> {
    let state = state.read().await;

    let updated_user = UserService::new(&state.db_pool)
        .update_user(&payload.user, &user, &state.jwt)
        .await?;

    Ok(warp::reply::json(&UserResponse::from(updated_user)))
}
