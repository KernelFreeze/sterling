mod routes;
mod state;

use std::env::var;
use std::net::{IpAddr, SocketAddr};

use thiserror::Error;
use tokio::net::TcpListener;

use crate::state::State;

const ADDR_ENVVAR: &'static str = "STERLING_ADDR";
const PORT_ENVVAR: &'static str = "STERLING_PORT";

const DEFAULT_BIND_ADDR: IpAddr = IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0));
const DEFAULT_PORT: u16 = 8080;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to bind to address: {0}")]
    Bind(#[from] std::io::Error),

    #[error("Failed to serve: {0}")]
    Serve(#[from] axum::Error),

    #[error(transparent)]
    BindAddress(#[from] BindAddressError),
}

#[derive(Debug, Error)]
pub enum BindAddressError {
    #[error("Failed to parse address: {0}")]
    ParseAddress(#[from] std::net::AddrParseError),

    #[error("Failed to parse port: {0}")]
    ParsePort(#[from] std::num::ParseIntError),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = routes::router().with_state(State::new());

    let socket_addr = socket_address()?;
    let listener = TcpListener::bind(socket_addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

fn bind_address() -> Result<IpAddr, BindAddressError> {
    var(ADDR_ENVVAR)
        .map(|addr| addr.parse().map_err(BindAddressError::ParseAddress))
        .unwrap_or(Ok(DEFAULT_BIND_ADDR))
}

fn bind_port() -> Result<u16, BindAddressError> {
    var(PORT_ENVVAR)
        .map(|port| port.parse().map_err(BindAddressError::ParsePort))
        .unwrap_or(Ok(DEFAULT_PORT))
}

fn socket_address() -> Result<SocketAddr, BindAddressError> {
    let addr = bind_address()?;
    let port = bind_port()?;
    Ok(SocketAddr::new(addr, port))
}
