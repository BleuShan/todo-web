use crate::{
    persistence::{
        PgPool,
        MIGRATOR,
    },
    prelude::*,
    Configuration,
};
use actix_web::{
    dev::Payload,
    error::ErrorInternalServerError,
    Error as WebError,
    FromRequest,
    HttpRequest,
};
use async_std::sync::Arc;
use future::Ready;
use parking_lot::{
    MappedRwLockReadGuard,
    RwLock,
    RwLockReadGuard,
};

#[derive(Debug)]
struct Inner {
    database: PgPool,
}

#[derive(Debug)]
pub struct AppData(Arc<RwLock<Inner>>);

impl AppData {
    #[instrument(err, skip(config))]
    pub async fn load(config: &Configuration) -> Result<Self> {
        let database: PgPool = config.database().into();
        info!("updating database");
        MIGRATOR.run(&database).await?;

        Ok(Inner { database }.into())
    }

    pub fn database(&self) -> MappedRwLockReadGuard<'_, PgPool> {
        RwLockReadGuard::map(self.0.read(), |data| &data.database)
    }
}

impl Clone for AppData {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl From<Inner> for AppData {
    fn from(data: Inner) -> Self {
        AppData(Arc::new(RwLock::from(data)))
    }
}

impl FromRequest for AppData {
    type Error = WebError;
    type Config = ();
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(request: &HttpRequest, _: &mut Payload) -> Self::Future {
        let result = request
            .app_data::<Self>()
            .cloned()
            .ok_or_else(|| ErrorInternalServerError("AppData instance not configured.".to_owned()));
        future::ready(result)
    }
}
