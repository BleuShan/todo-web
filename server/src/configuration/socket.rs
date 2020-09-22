use crate::prelude::*;
use clap::Clap;
use std::net::IpAddr;

#[derive(Clap, Debug, Clone)]
pub struct SocketConfiguration {
    /// http server host
    #[clap(short, long, name = "HOST", env = "HOST", default_value = "::1")]
    host: IpAddr,
    /// http server port
    #[clap(short, long, name = "PORT", env = "PORT", default_value = "3000")]
    port: u16,
}

impl SocketConfiguration {
    pub fn as_tuple(&self) -> (IpAddr, u16) {
        (self.host, self.port)
    }
}
