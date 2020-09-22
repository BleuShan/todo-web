pub use sqlx::{
    error::{
        Error as SQLError,
        Result as SQLResult,
    },
    migrate::Migrate,
    postgres::{
        self,
        PgConnectOptions,
    },
    prelude::*,
    query,
    query_as,
    query_file,
    query_scalar,
    FromRow,
    PgPool,
};
use sqlx::{
    pool::PoolConnection,
    postgres::Postgres,
};

pub type PgPoolConnection = PoolConnection<Postgres>;
