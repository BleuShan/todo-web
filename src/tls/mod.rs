mod pemfile;

use crate::prelude::*;
use async_std::{
    fs::File,
    io::BufReader,
    path::Path,
};
use rustls::{
    Certificate,
    NoClientAuth,
    PrivateKey,
    ServerConfig,
};

async fn load_tls_certs<PathRef>(path: PathRef) -> Result<Vec<Certificate>>
where
    PathRef: AsRef<Path>,
{
    let cert_file = Box::pin(BufReader::new(File::open(path).await?));
    pemfile::certs(cert_file).await
}

async fn load_tls_key<PathRef>(path: PathRef) -> Result<PrivateKey>
where
    PathRef: AsRef<Path>,
{
    let rsa_keys = async {
        let reader = Box::pin(BufReader::new(File::open(path.as_ref()).await?));
        let keys = pemfile::rsa_private_keys(reader).await?;
        keys.first()
            .cloned()
            .ok_or_else(|| eyre!("No rsa key found"))
    };

    let pkcs8_keys = async {
        let reader = Box::pin(BufReader::new(File::open(path.as_ref()).await?));
        let keys = pemfile::pkcs8_private_keys(reader).await?;
        keys.first()
            .cloned()
            .ok_or_else(|| eyre!("No pkcs8_keys key found"))
    };

    let (rsa_key, pkcs8_key) = rsa_keys.join(pkcs8_keys).await;

    if let Ok(key) = pkcs8_key {
        Ok(key.clone())
    } else if let Ok(key) = rsa_key {
        Ok(key.clone())
    } else {
        Err(eyre!("No key found"))
    }
}

pub async fn config() -> Result<ServerConfig> {
    let mut config = ServerConfig::new(NoClientAuth::new());
    config.set_single_cert(
        load_tls_certs("localhost.pem").await?,
        load_tls_key("localhost-key.pem").await?,
    )?;

    Ok(config)
}
