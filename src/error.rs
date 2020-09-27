use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ConduitError(#[from] conduit::ConduitError),
    #[error(transparent)]
    ServerError(#[from] server::ServerError),
    #[error(transparent)]
    VarError(#[from] std::env::VarError),
    #[error(transparent)]
    DotenvError(#[from] dotenv::Error),
    #[error(transparent)]
    AddrParseError(#[from] std::net::AddrParseError),
}
