use crate::{
    net::tls::{
        server::ServerConfigCache,
        ServerConfig,
    },
    prelude::*,
};
use clap::Clap;
use std::{
    path::PathBuf,
    sync::Arc,
};

#[derive(Clap, Debug)]
pub struct TLSConfiguration {
    /// tls certificate file
    #[clap(long = "certs", name = "TLS_CERTFILE", env = "TLS_CERTFILE")]
    certs: Option<PathBuf>,
    /// tls key file
    #[clap(long = "key", name = "TLS_KEYFILE", env = "TLS_KEYFILE")]
    key: Option<PathBuf>,
}

impl TLSConfiguration {
    #[instrument(err, skip(self))]
    pub async fn load_server_config(&self) -> io::Result<Arc<ServerConfig>> {
        if self.certs.is_none() || self.key.is_none() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No CERFILE or KEYFILE Provided",
            ));
        }
        let cert = self.certs.as_ref().unwrap();
        let key = self.key.as_ref().unwrap();
        ServerConfigCache::current().load(cert, key).await
    }
}
