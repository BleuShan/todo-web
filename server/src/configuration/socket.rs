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
pub struct SocketConfiguration {
    /// http server host
    #[clap(short, long, name = "HOST", env = "HOST", default_value = "localhost")]
    host: String,
    /// http server port
    #[clap(short, long, name = "PORT", env = "PORT", default_value = "3000")]
    port: u16,
}

impl ToSocketAddrs for &SocketConfiguration {
    type Iter = IntoIter<SocketAddr>;

    fn to_socket_addrs(&self) -> std::io::Result<Self::Iter> {
        (self.host.as_str(), self.port).to_socket_addrs()
    }
}
