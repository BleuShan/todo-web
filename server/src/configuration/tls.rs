use crate::{
    prelude::*,
    tls,
};
use async_std::path::PathBuf;
use clap::Clap;

/// http server TLS configuration
#[derive(Clap, Debug)]
pub struct Tls {
    /// tls certificate file
    #[clap(long = "certs", name = "CERTFILE", env = "TLS_CERTFILE")]
    certs: Option<PathBuf>,
    /// tls key file
    #[clap(long = "key", name = "KEYFILE", env = "TLS_KEYFILE")]
    key: Option<PathBuf>,
}

impl Tls {
    pub async fn load_server_config(&self) -> Result<tls::ServerConfig> {
        ensure!(self.certs.is_some(), "No CERTFILE provided");
        ensure!(self.key.is_some(), "No KEYFILE provided");
        let mut config = tls::ServerConfig::new(tls::NoClientAuth::new());
        let certs = tls::load_certs(self.certs.as_ref().unwrap());
        let key = tls::load_key(self.key.as_ref().unwrap());

        let (cert_chain, key_der) = certs.try_join(key).await?;
        config.set_single_cert(cert_chain, key_der)?;

        Ok(config)
    }
}
