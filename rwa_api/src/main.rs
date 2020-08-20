pub(crate) mod api;
pub(crate) mod config;
pub(crate) mod db;
pub(crate) mod jwt;
pub(crate) mod server;

use config::Config;
use jwt::JWT;
use server::{Server, State};
use sqlx::postgres::PgPoolOptions;

#[async_std::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = Config::load()?;

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    let jwt = JWT::new(config.jwt_secret.clone());

    let url = format!("{}:{}", config.host, config.port);
    let app = Server::new(State {
        db_pool,
        config,
        jwt,
    });
    app.listen(url).await?;

    Ok(())
}
