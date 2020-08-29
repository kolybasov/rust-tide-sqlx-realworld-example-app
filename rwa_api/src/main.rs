pub(crate) mod api;
pub(crate) mod config;
pub(crate) mod jwt;
pub(crate) mod query;
pub(crate) mod services;

use api::{Server, State};
use config::Config;
use jwt::JWT;
use listenfd::ListenFd;
use sqlx::postgres::PgPoolOptions;

#[async_std::main]
async fn main() -> Result<(), anyhow::Error> {
    let mut listenfd = ListenFd::from_env();
    let config = Config::load()?;

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    let jwt = JWT::new(&config.jwt_secret);

    let url = format!("{}:{}", config.host, config.port);
    let app = Server::new(State {
        db_pool,
        config,
        jwt,
    });

    if let Some(listener) = listenfd.take_tcp_listener(0)? {
        app.listen(listener).await
    } else {
        app.listen(url).await
    }?;

    Ok(())
}
