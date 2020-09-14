use crate::{
    prelude::*,
    tls,
};
use async_std::{
    io,
    path::PathBuf,
};
use clap::Clap;

/// http server TLS configuration
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
    pub async fn load_server_config(&self) -> io::Result<tls::ServerConfig> {
        let certs = async {
            if let Some(ref certs) = self.certs {
                tls::load_certs(certs).await
            } else {
                Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "No CERFILE Provided",
                ))
            }
        };
        let key = async {
            if let Some(ref key) = self.key {
                tls::load_key(key).await
            } else {
                Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "No KEYFILE Provided",
                ))
            }
        };

        let (cert_chain, key_der) = future::try_join(certs, key).await?;
        let mut config = tls::ServerConfig::new(tls::NoClientAuth::new());
        config
            .set_single_cert(cert_chain, key_der)
            .map_err(|error| io::Error::new(io::ErrorKind::Other, error))?;

        Ok(config)
    }
}
