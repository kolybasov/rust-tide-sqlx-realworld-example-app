mod config;
mod error;

use conduit::{ConduitError, PgPoolOptions};
use config::Config;
use error::Error;
use gql::Gql;
use rest::Rest;
use server::{state::State, warp, Server, ServerState, JWT};
use std::sync::Arc;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = Config::load()?;

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .map_err(ConduitError::from)?;
    let jwt = JWT::new(&config.jwt_secret);
    let state: ServerState = State { db_pool, jwt }.into();

    let routes = Rest::new(Arc::clone(&state)).or(Gql::new(state));

    Ok(Server::run(&config.url().parse()?, routes).await?)
}
