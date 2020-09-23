use crate::JWT;
use conduit::PgPool;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::Filter;

#[derive(Clone)]
pub struct State {
    pub db_pool: PgPool,
    pub jwt: JWT,
}

pub type ServerState = Arc<RwLock<State>>;

impl From<State> for ServerState {
    fn from(state: State) -> Self {
        Arc::new(RwLock::new(state))
    }
}

pub fn with_state(
    state: ServerState,
) -> impl Filter<Extract = (ServerState,), Error = Infallible> + Clone {
    warp::any().map(move || state.clone())
}

pub fn with_db(state: ServerState) -> impl Filter<Extract = (PgPool,), Error = Infallible> + Clone {
    with_state(state).and_then(|state: ServerState| async move {
        let state = state.read().await;
        Ok::<_, Infallible>(state.db_pool.clone())
    })
}
