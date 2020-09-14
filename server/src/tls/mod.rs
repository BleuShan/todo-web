mod pemfile;

use crate::prelude::*;
use async_std::{
    fs::File,
    io::{
        self,
        BufReader,
    },
    path::Path,
};
pub use rustls::*;

pub async fn load_certs<PathRef>(path: PathRef) -> io::Result<Vec<Certificate>>
where
    PathRef: AsRef<Path>,
{
    let cert_file = Box::pin(BufReader::new(File::open(path).await?));
    pemfile::certs(cert_file).await
}

pub async fn load_key<PathRef>(path: PathRef) -> io::Result<PrivateKey>
where
    PathRef: AsRef<Path>,
{
    let rsa_keys = async {
        let reader = Box::pin(BufReader::new(File::open(path.as_ref()).await?));
        let keys = pemfile::rsa_private_keys(reader).await?;
        keys.first().cloned().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("No valid rsa key found in {}", path.as_ref().display()),
            )
        })
    };

    let pkcs8_keys = async {
        let reader = Box::pin(BufReader::new(File::open(path.as_ref()).await?));
        let keys = pemfile::pkcs8_private_keys(reader).await?;
        keys.first().cloned().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("No valid pkcs8 key found in {}", path.as_ref().display()),
            )
        })
    };

    let (rsa_key, pkcs8_key) = future::join(rsa_keys, pkcs8_keys).await;

    if let Ok(key) = pkcs8_key {
        Ok(key.clone())
    } else if let Ok(key) = rsa_key {
        Ok(key.clone())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("No valid key found in {}", path.as_ref().display()),
        ))
    }
}
