pub mod auth;
pub mod error;
mod jwt;
pub mod state;

pub use crate::error::ServerError;
pub use auth::auth;
pub use hyper;
use hyper::server::Server as HyperServer;
pub use jwt::JWT;
use listenfd::ListenFd;
pub use state::{with_db, with_state, ServerState};
use std::convert::Infallible;
use std::net::SocketAddr;
pub use warp;
use warp::{http::Method, Filter, Rejection, Reply};

pub struct Server;

impl Server {
    pub async fn run(
        url: &SocketAddr,
        routes: impl Filter<Extract = (impl Reply,), Error = Rejection> + Sync + Send + Clone + 'static,
    ) -> Result<(), ServerError> {
        let routes = routes
            .with(
                warp::cors()
                    .allow_any_origin()
                    .allow_methods(&[Method::GET, Method::POST, Method::PUT, Method::DELETE])
                    .allow_headers(vec!["content-type", "authorization"]),
            )
            .with(warp::compression::deflate())
            .with(warp::trace::request())
            .boxed();

        let mut listenfd = ListenFd::from_env();
        if let Some(listener) = listenfd.take_tcp_listener(0)? {
            Server::run_from_listener(routes, listener).await?;
        } else {
            Server::run_from_url(routes, url).await;
        };

        Ok(())
    }

    async fn run_from_url(
        routes: impl Filter<Extract = (impl Reply,), Error = Rejection> + Sync + Send + Clone + 'static,
        url: &SocketAddr,
    ) {
        warp::serve(routes)
            .tls()
            .cert_path("certs/cert.pem")
            .key_path("certs/key.pem")
            .run(*url)
            .await;
    }

    async fn run_from_listener(
        routes: impl Filter<Extract = (impl Reply,), Error = Rejection> + Sync + Send + Clone + 'static,
        listener: std::net::TcpListener,
    ) -> Result<(), ServerError> {
        let service = warp::service(routes);
        let make_svc = hyper::service::make_service_fn(|_: _| {
            let service = service.clone();
            async move { Ok::<_, Infallible>(service) }
        });

        let server = HyperServer::from_tcp(listener)?;
        Ok(server.serve(make_svc).await?)
    }
}
