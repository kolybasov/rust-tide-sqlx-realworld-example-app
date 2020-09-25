mod config;

use conduit::PgPoolOptions;
use config::Config;
use gql::Gql;
use rest::Rest;
use server::{state::State, warp, Server, ServerState, JWT};
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = Config::load()?;

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;
    let jwt = JWT::new(&config.jwt_secret);
    let state: ServerState = State { db_pool, jwt }.into();

    let routes = Rest::new(state.clone()).or(Gql::new(state));

    Server::run(&config.url().parse()?, routes).await
}
