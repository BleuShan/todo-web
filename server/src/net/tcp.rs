use crate::prelude::*;
use listenfd::ListenFd;
pub use tokio::net::{
    TcpListener,
    TcpStream,
};
use tokio::{
    io,
    net::ToSocketAddrs,
};

#[instrument(err)]
pub async fn bind_listener<Addr>(addr: Addr) -> io::Result<TcpListener>
where
    Addr: ToSocketAddrs + Debug,
{
    match ListenFd::from_env().take_tcp_listener(0)? {
        Some(listener) => TcpListener::try_from(listener),
        None => TcpListener::bind(addr).await,
    }
}
