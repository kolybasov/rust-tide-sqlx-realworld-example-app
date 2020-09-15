use conduit::{config::Config, jwt::JWT, PgPoolOptions};
use gql::{schema, Context};
use hyper::server::Server as HyperServer;
use listenfd::ListenFd;
use rest::{hyper, warp, Server, State, WarpState};
use std::convert::Infallible;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mut listenfd = ListenFd::from_env();
    let config = Config::load()?;

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    let jwt = JWT::new(&config.jwt_secret);

    let url = format!("{}:{}", config.host, config.port);
    let state: WarpState = State {
        db_pool: db_pool.clone(),
        config,
        jwt,
    }
    .into();

    let context = Context { db_pool };
    let ctx = warp::any().map(move || context.clone());
    let routes = Server::new(state)
        .or(warp::path!("graphiql").and(juniper_warp::graphiql_filter("/graphql", None)))
        .or(warp::path!("graphql").and(juniper_warp::make_graphql_filter(schema(), ctx.boxed())));
    let svc = warp::service(routes);
    let make_svc = hyper::service::make_service_fn(|_: _| {
        let svc = svc.clone();
        async move { Ok::<_, Infallible>(svc) }
    });

    let server = if let Some(listener) = listenfd.take_tcp_listener(0)? {
        HyperServer::from_tcp(listener)?
    } else {
        HyperServer::bind(&url.parse::<std::net::SocketAddr>()?)
    };

    server.serve(make_svc).await?;
    Ok(())
}
