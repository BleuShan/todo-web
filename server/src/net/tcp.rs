use crate::prelude::*;
use async_std::io;
pub use async_std::net::{
    TcpListener,
    TcpStream,
    ToSocketAddrs,
};
use listenfd::ListenFd;

#[instrument(err)]
pub async fn bind_listener<Addr>(addr: Addr) -> io::Result<TcpListener>
where
    Addr: ToSocketAddrs + Debug,
{
    match ListenFd::from_env().take_tcp_listener(0)? {
        Some(listener) => Ok(TcpListener::from(listener)),
        None => TcpListener::bind(addr).await,
    }
}
