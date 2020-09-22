mod database;
mod socket;
mod tls;

use crate::prelude::*;
use clap::Clap;
pub use database::DatabaseConfiguration;
pub use once_cell::sync::Lazy;
pub use socket::SocketConfiguration;
pub use tls::TLSConfiguration;

static CURRENT_CONFIG: Lazy<Configuration> = Lazy::new(Configuration::parse);

#[derive(Clap, Debug)]
#[clap(author, about, version)]
pub struct Configuration {
    #[clap(flatten)]
    database: DatabaseConfiguration,
    #[clap(flatten)]
    socket: SocketConfiguration,
    #[clap(flatten)]
    tls: TLSConfiguration,
}

impl Configuration {
    pub fn current() -> &'static Self {
        CURRENT_CONFIG.deref()
    }

    pub fn database(&self) -> &DatabaseConfiguration {
        &self.database
    }

    pub fn socket(&self) -> &SocketConfiguration {
        &self.socket
    }

    pub fn tls(&self) -> &TLSConfiguration {
        &self.tls
    }
}
