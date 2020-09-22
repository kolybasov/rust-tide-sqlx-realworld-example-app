use conduit::{PgPool, User};
use server::{auth, warp, with_state, ServerState};
use warp::{Filter, Rejection};

#[derive(Clone)]
pub struct Context {
    pub state: ServerState,
    pub user: Option<User>,
}

impl juniper::Context for Context {}

impl Context {
    pub fn extract(state: ServerState) -> impl Filter<Extract = (Context,), Error = Rejection> {
        with_state(state.clone())
            .and(auth::optional(state))
            .map(|state: ServerState, user: Option<User>| Context { state, user })
    }

    pub async fn get_pool(&self) -> PgPool {
        self.state.read().await.db_pool.clone()
    }
}
