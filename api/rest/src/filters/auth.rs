use crate::filters::state::{with_state, WarpState};
use conduit::{error::Error, User, UserService};
use std::convert::Infallible;
use warp::{Filter, Rejection};

pub fn auth(state: WarpState) -> impl Filter<Extract = (User,), Error = Rejection> + Clone {
    base(state).and_then(auth_internal)
}

pub fn optional(
    state: WarpState,
) -> impl Filter<Extract = (Option<User>,), Error = Rejection> + Clone {
    base(state).and_then(optional_internal)
}

async fn auth_internal(auth_header: Option<String>, state: WarpState) -> Result<User, Rejection> {
    get_user_from_header(auth_header, state)
        .await
        .map_err(|err| warp::reject::custom(err))
}

async fn optional_internal(
    auth_header: Option<String>,
    state: WarpState,
) -> Result<Option<User>, Infallible> {
    Ok(get_user_from_header(auth_header, state).await.ok())
}

fn base(
    state: WarpState,
) -> impl Filter<Extract = (Option<String>, WarpState), Error = Rejection> + Clone {
    warp::header::optional("authorization").and(with_state(state))
}

async fn get_user_from_header(
    auth_header_raw: Option<String>,
    state: WarpState,
) -> conduit::error::Result<User> {
    let state = state.read().await;
    let auth_header = auth_header_raw.ok_or(Error::Unauthorized)?;
    let token = auth_header
        .split_whitespace()
        .last()
        .ok_or(Error::Unauthorized)?;
    let claims = state.jwt.verify(token)?;
    UserService::new(&state.db_pool)
        .get_user_by_id(claims.data.id)
        .await
}
