mod socket;
mod tls;

use crate::prelude::*;
use clap::Clap;
use once_cell::sync::Lazy;
pub use socket::Socket;
pub use tls::Tls;

static CURRENT_CONFIG: Lazy<Configuration> = Lazy::new(|| Configuration::parse());

#[derive(Clap, Debug)]
#[clap(author, about, version)]
pub struct Configuration {
    #[clap(flatten)]
    socket: Socket,
    #[clap(flatten)]
    tls: Tls,
}

impl Configuration {
    pub fn load() -> &'static Self {
        &CURRENT_CONFIG
    }

    pub fn socket(&self) -> &Socket {
        &self.socket
    }

    pub fn tls(&self) -> &Tls {
        &self.tls
    }
}
