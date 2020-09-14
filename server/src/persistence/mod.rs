pub mod models;

use sqlx::migrate;
pub use sqlx::{
    error::{
        Error as SQLError,
        Result as SQLResult,
    },
    postgres::{
        self,
        PgConnectOptions,
    },
    prelude,
    query,
    query_as,
    query_file,
    query_scalar,
    FromRow,
    PgPool,
};

pub static MIGRATOR: migrate::Migrator = migrate!("./migrations");
