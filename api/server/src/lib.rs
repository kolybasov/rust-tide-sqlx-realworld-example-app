pub mod auth;
pub mod state;

pub use auth::auth;
pub use hyper;
use hyper::server::Server as HyperServer;
use listenfd::ListenFd;
pub use state::{with_db, with_state, ServerState};
use std::convert::Infallible;
use std::net::SocketAddr;
pub use warp;
use warp::{Filter, Rejection, Reply};

pub struct Server;

impl Server {
    pub async fn run(
        url: &SocketAddr,
        routes: impl Filter<Extract = (impl Reply,), Error = Rejection> + Sync + Send + Clone + 'static,
    ) -> Result<(), anyhow::Error> {
        let service = warp::service(routes);
        let make_svc = hyper::service::make_service_fn(|_: _| {
            let service = service.clone();
            async move { Ok::<_, Infallible>(service) }
        });

        let mut listenfd = ListenFd::from_env();
        let server = if let Some(listener) = listenfd.take_tcp_listener(0)? {
            HyperServer::from_tcp(listener)?
        } else {
            HyperServer::bind(url)
        };

        Ok(server.serve(make_svc).await?)
    }
}
