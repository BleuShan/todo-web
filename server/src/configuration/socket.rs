use crate::prelude::*;
use clap::Clap;
use std::{
    net::{
        SocketAddr,
        ToSocketAddrs,
    },
    vec::IntoIter,
};

/// Network Socket configuration
#[derive(Clap, Debug)]
pub struct Socket {
    /// http server host
    #[clap(short, long, env = "HOST", default_value = "localhost")]
    host: String,
    /// http server port
    #[clap(short, long, env = "PORT", default_value = "3000")]
    port: u16,
}

impl ToSocketAddrs for &Socket {
    type Iter = IntoIter<SocketAddr>;

    fn to_socket_addrs(&self) -> std::io::Result<Self::Iter> {
        (self.host.as_str(), self.port).to_socket_addrs()
    }
}
