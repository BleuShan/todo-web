use crate::{
    net::tls,
    prelude::*,
};
use once_cell::sync::Lazy;
use std::{
    collections::BTreeMap,
    path::{
        Path,
        PathBuf,
    },
    sync::Arc,
};
use tokio::{
    io,
    sync::RwLock,
};
pub use tokio_rustls::server::TlsStream;

type ServerConfigCacheState = BTreeMap<ServerConfigCacheKey, Arc<tls::ServerConfig>>;
type SharedServerConfigCacheState = Arc<RwLock<ServerConfigCacheState>>;

static CURRENT_SERVER_CACHE: Lazy<ServerConfigCache> =
    Lazy::new(|| ServerConfigCache(Arc::new(RwLock::new(BTreeMap::new()))));

#[derive(Clone, Debug, Default, PartialEq, Eq, Ord, PartialOrd, Hash)]
struct ServerConfigCacheKey {
    cert: PathBuf,
    key: PathBuf,
}

impl ServerConfigCacheKey {
    fn new<CertFilePath, KeyFilePath>(cert: &CertFilePath, key: &KeyFilePath) -> Self
    where
        CertFilePath: AsRef<Path>,
        KeyFilePath: AsRef<Path>,
    {
        Self {
            key: key.as_ref().to_owned(),
            cert: cert.as_ref().to_owned(),
        }
    }
}

#[repr(transparent)]
pub struct ServerConfigCache(SharedServerConfigCacheState);

impl ServerConfigCache {
    pub fn current() -> &'static Self {
        CURRENT_SERVER_CACHE.deref()
    }

    pub async fn load<CertFilePath, KeyFilePath>(
        &self,
        cert: CertFilePath,
        key: KeyFilePath,
    ) -> io::Result<Arc<tls::ServerConfig>>
    where
        CertFilePath: AsRef<Path>,
        KeyFilePath: AsRef<Path>,
    {
        let cache_key = ServerConfigCacheKey::new(&cert, &key);
        if let Some(config) = self.get(&cache_key).await {
            return Ok(config);
        }

        let (cert_chain, key_der) = try_join!(tls::load_certs(cert), tls::load_key(key))?;
        let mut config = tls::ServerConfig::new(tls::NoClientAuth::new());
        config
            .set_single_cert(cert_chain, key_der)
            .map_err(|error| io::Error::new(io::ErrorKind::Other, error))?;
        let config = Arc::from(config);
        self.set(cache_key, Arc::clone(&config)).await;

        Ok(config)
    }

    async fn get(&self, key: &ServerConfigCacheKey) -> Option<Arc<tls::ServerConfig>> {
        let lock = Arc::clone(&self.0);
        let guard = lock.read().await;
        guard.get(key).cloned()
    }

    async fn set(&self, key: ServerConfigCacheKey, config: Arc<tls::ServerConfig>) {
        let lock = Arc::clone(&self.0);
        let mut guard = lock.write().await;
        guard.insert(key, config);
    }
}

impl Clone for ServerConfigCache {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}
