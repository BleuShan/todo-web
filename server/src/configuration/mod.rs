mod database;
mod socket;
mod tls;

use crate::prelude::*;
use clap::Clap;
pub use database::DatabaseConfiguration;
pub use once_cell::sync::OnceCell;
pub use socket::SocketConfiguration;
pub use tls::TLSConfiguration;

static CURRENT_CONFIG: OnceCell<Configuration> = OnceCell::new();

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
    pub fn load() -> &'static Self {
        CURRENT_CONFIG.get_or_init(Self::parse)
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
