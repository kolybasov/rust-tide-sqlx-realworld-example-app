use crate::error::{Result, ServerError};
use crate::state::{with_state, ServerState};
use conduit::{User, UserService};
use std::convert::Infallible;
use warp::{Filter, Rejection};

pub fn auth(state: ServerState) -> impl Filter<Extract = (User,), Error = Rejection> + Clone {
    base(state).and_then(|auth_header: Option<String>, state: ServerState| async {
        get_user_from_header(auth_header, state)
            .await
            .map_err(warp::reject::custom)
    })
}

pub fn optional(
    state: ServerState,
) -> impl Filter<Extract = (Option<User>,), Error = Infallible> + Clone {
    base(state).and_then(|auth_header: Option<String>, state: ServerState| async {
        Ok::<_, Infallible>(get_user_from_header(auth_header, state).await.ok())
    })
}

pub fn set_cookie_token(token: &str) -> String {
    format!(
        "authorization=Token {}; SameSite=Strict; HttpOnly; Path=/",
        token
    )
}

fn base(
    state: ServerState,
) -> impl Filter<Extract = (Option<String>, ServerState), Error = Infallible> + Clone {
    warp::header("authorization")
        .or(warp::cookie("authorization"))
        .unify()
        .map(Some)
        .or(warp::any().map(|| None))
        .unify()
        .and(with_state(state))
}

async fn get_user_from_header(auth_header_raw: Option<String>, state: ServerState) -> Result<User> {
    let state = state.read().await;
    let auth_header = auth_header_raw.ok_or(ServerError::Unauthorized)?;
    let token = auth_header
        .split_whitespace()
        .last()
        .ok_or(ServerError::Unauthorized)?;
    let claims = state.jwt.verify(token)?;
    Ok(UserService::new(&state.db_pool)
        .get_user_by_id(claims.data.id)
        .await?)
}
