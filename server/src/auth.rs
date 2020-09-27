use crate::error::{ServerError, Result};
use crate::state::{with_state, ServerState};
use conduit::{User, UserService};
use std::convert::Infallible;
use warp::{Filter, Rejection};

pub fn auth(state: ServerState) -> impl Filter<Extract = (User,), Error = Rejection> + Clone {
    base(state).and_then(|auth_header: Option<String>, state: ServerState| async {
        get_user_from_header(auth_header, state)
            .await
            .map_err(|err| warp::reject::custom(err))
    })
}

pub fn optional(
    state: ServerState,
) -> impl Filter<Extract = (Option<User>,), Error = Rejection> + Clone {
    base(state).and_then(|auth_header: Option<String>, state: ServerState| async {
        Ok::<_, Infallible>(get_user_from_header(auth_header, state).await.ok())
    })
}

fn base(
    state: ServerState,
) -> impl Filter<Extract = (Option<String>, ServerState), Error = Rejection> + Clone {
    warp::header::optional("authorization").and(with_state(state))
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
