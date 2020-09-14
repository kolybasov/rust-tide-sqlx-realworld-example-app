use conduit::{config::Config, jwt::JWT, PgPoolOptions};
use hyper::server::Server as HyperServer;
use listenfd::ListenFd;
use rest_api::{hyper, warp, Server, State, WarpState};
use std::convert::Infallible;

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
        db_pool,
        config,
        jwt,
    }
    .into();

    let svc = warp::service(Server::new(state));
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
