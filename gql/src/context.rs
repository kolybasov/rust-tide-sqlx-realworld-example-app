use conduit::{PgPool, User};
use juniper::{FieldError, Value};
use server::{auth, warp, with_state, ServerState};
use std::sync::Arc;
use warp::{Filter, Rejection};

#[derive(Clone)]
pub struct Context {
    pub state: ServerState,
    pub user: Option<User>,
}

impl juniper::Context for Context {}

impl Context {
    pub fn extract(state: ServerState) -> impl Filter<Extract = (Context,), Error = Rejection> {
        with_state(Arc::clone(&state))
            .and(auth::optional(state))
            .map(|state: ServerState, user: Option<User>| Context { state, user })
    }

    pub async fn get_pool(&self) -> PgPool {
        self.state.read().await.db_pool.clone()
    }

    pub fn get_user(&self) -> Result<&User, FieldError> {
        self.user
            .as_ref()
            .ok_or_else(|| FieldError::new("Unauthorized", Value::Null))
    }

    pub fn get_user_id(&self) -> Option<i32> {
        self.user.as_ref().map(|user| user.id)
    }
}
