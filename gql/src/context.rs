use crate::error::Result;
use conduit::{PgPool, User};
use server::{auth, warp, with_state, ServerError, ServerState};
use std::convert::Infallible;
use std::sync::Arc;
use warp::Filter;

#[derive(Clone)]
pub struct Context {
    pub state: ServerState,
    pub user: Option<User>,
}

impl juniper::Context for Context {}

impl Context {
    pub fn extract(state: ServerState) -> impl Filter<Extract = (Context,), Error = Infallible> {
        with_state(Arc::clone(&state))
            .and(auth::optional(state))
            .map(|state: ServerState, user: Option<User>| Context { state, user })
    }

    pub async fn get_pool(&self) -> PgPool {
        self.state.read().await.db_pool.clone()
    }

    pub fn get_user(&self) -> Result<&User> {
        Ok(self
            .user
            .as_ref()
            .ok_or_else(|| ServerError::Unauthorized)?)
    }

    pub fn get_user_id(&self) -> Option<i32> {
        self.user.as_ref().map(|user| user.id)
    }
}
