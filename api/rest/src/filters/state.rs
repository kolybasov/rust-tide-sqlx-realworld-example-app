use crate::state::State;
pub use conduit::PgPool;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::Filter;

pub type WarpState = Arc<RwLock<State>>;

impl From<State> for WarpState {
    fn from(state: State) -> Self {
        Arc::new(RwLock::new(state))
    }
}

pub fn with_state(
    state: WarpState,
) -> impl Filter<Extract = (WarpState,), Error = Infallible> + Clone {
    warp::any().map(move || state.clone())
}

pub fn with_db(state: WarpState) -> impl Filter<Extract = (PgPool,), Error = Infallible> + Clone {
    with_state(state).and_then(with_db_internal)
}

async fn with_db_internal(state: WarpState) -> Result<PgPool, Infallible> {
    let state = state.read().await;
    Ok(state.db_pool.clone())
}
