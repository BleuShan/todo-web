mod socket;
mod tls;

use crate::prelude::*;
use clap::Clap;
pub use socket::Socket;
pub use tls::Tls;

#[derive(Clap, Debug)]
#[clap(author, about, version)]
pub struct Configuration {
    #[clap(flatten)]
    socket: Socket,
    #[clap(flatten)]
    tls: Tls,
}

impl Configuration {
    pub fn load() -> Self {
        Self::parse()
    }

    pub fn socket(&self) -> &Socket {
        &self.socket
    }

    pub fn tls(&self) -> &Tls {
        &self.tls
    }
}
