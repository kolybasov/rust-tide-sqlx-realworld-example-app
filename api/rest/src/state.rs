use conduit::{config::Config, jwt::JWT, PgPool};

#[derive(Clone)]
pub struct State {
    pub db_pool: PgPool,
    pub config: Config,
    pub jwt: JWT,
}
