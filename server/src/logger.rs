use crate::prelude::*;
use std::io::stderr;
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    fmt,
    registry,
    EnvFilter,
};

pub struct Logger;

impl Logger {
    pub fn init() -> Result<Self> {
        let fmt_layer = fmt::layer().with_writer(stderr);
        let filter_layer =
            EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info"))?;
        registry()
            .with(filter_layer)
            .with(fmt_layer)
            .with(ErrorLayer::default())
            .init();

        Ok(Self)
    }
}
