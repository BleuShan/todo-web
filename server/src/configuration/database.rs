use crate::prelude::*;
use clap::Clap;
use sqlx::postgres::PgConnectOptions;

#[derive(Clap, Debug)]
pub struct DatabaseConfiguration {
    /// Postgres database connection string
    #[clap(long = "db", name = "DATABASE_URL", env = "DATABASE_URL")]
    inner: PgConnectOptions,
}

impl From<&DatabaseConfiguration> for PgConnectOptions {
    #[inline]
    fn from(config: &DatabaseConfiguration) -> Self {
        config.inner.clone()
    }
}
