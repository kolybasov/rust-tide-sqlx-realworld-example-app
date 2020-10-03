mod config;
mod error;

use conduit::{ConduitError, PgPoolOptions};
use config::Config;
use error::Error;
use gql::Gql;
use rest::Rest;
use server::{state::State, warp, Server, ServerState, JWT};
use std::sync::Arc;
use tracing_subscriber;
use warp::Filter;
use web::Web;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().init();

    let config = Config::load()?;

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .map_err(ConduitError::from)?;
    let jwt = JWT::new(&config.jwt_secret);
    let state: ServerState = State { db_pool, jwt }.into();

    let routes = Gql::routes(Arc::clone(&state))
        .or(Rest::routes(Arc::clone(&state)))
        .or(Web::routes(Arc::clone(&state)));

    Ok(Server::run(&config.url().parse()?, routes).await?)
}
