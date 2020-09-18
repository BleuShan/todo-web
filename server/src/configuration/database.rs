use crate::prelude::*;
use clap::Clap;
use sqlx::postgres::{
    PgConnectOptions,
    PgPool,
};

/// Database Connection configuration
#[derive(Clap, Debug)]
pub struct DatabaseConfiguration {
    /// Postgres database connection string
    #[clap(short = "db", long, name = "DATABASE_URL", env = "DATABASE_URL")]
    inner: PgConnectOptions,
}

impl From<&DatabaseConfiguration> for PgPool {
    fn from(config: &DatabaseConfiguration) -> Self {
        Self::connect_lazy_with(config.inner.clone())
    }
}
