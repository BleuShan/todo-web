use crate::prelude::*;
use once_cell::sync::OnceCell;
use std::sync::Arc;
use tracing::{
    Dispatch,
    Subscriber,
};
#[cfg(feature = "tracing-appender")]
use tracing_appender::non_blocking::WorkerGuard;
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    layer::Layered,
    registry::LookupSpan,
    EnvFilter,
    Layer,
    Registry,
};

#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(all(feature = "wasm-bindgen", target_arch = "wasm32"))]
mod wasm;
#[cfg(all(feature = "wasm-bindgen", target_arch = "wasm32"))]
pub use self::wasm::*;

pub struct LoggerConfig<SubscriberType = Registry>
where
    SubscriberType:
        Subscriber + Sized + Send + Sync + Into<Dispatch> + for<'span> LookupSpan<'span>,
{
    inner: OnceCell<SubscriberType>,
    #[cfg(feature = "tracing-appender")]
    guard: OnceCell<WorkerGuard>,
}

impl Debug for LoggerConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(std::any::type_name::<Self>()).finish()
    }
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            inner: OnceCell::from(Registry::default()),
            #[cfg(feature = "tracing-appender")]
            guard: OnceCell::new(),
        }
    }
}

impl<SubscriberType> LoggerConfig<SubscriberType>
where
    SubscriberType:
        Subscriber + Sized + Send + Sync + Into<Dispatch> + for<'span> LookupSpan<'span>,
{
    pub fn with<LayerType>(
        mut self,
        layer: LayerType,
    ) -> Result<LoggerConfig<Layered<LayerType, SubscriberType>>>
    where
        LayerType: Layer<SubscriberType> + Send + Sync,
    {
        let inner = self
            .inner
            .take()
            .map(|value| value.with(layer))
            .ok_or_else(|| eyre!("Failed to apply layer"))?;

        Ok(LoggerConfig {
            inner: OnceCell::from(inner),
            #[cfg(feature = "tracing-appender")]
            guard: self.guard,
        })
    }

    pub fn with_default_env_filter(
        self,
    ) -> Result<LoggerConfig<Layered<EnvFilter, SubscriberType>>> {
        let filter = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::from_str("info"))?;
        self.with(filter)
    }

    pub fn with_env_filter(
        self,
        level: &str,
    ) -> Result<LoggerConfig<Layered<EnvFilter, SubscriberType>>> {
        let filter = EnvFilter::from_str(level)?;
        self.with(filter)
    }

    pub fn with_default_error_layer(
        self,
    ) -> Result<LoggerConfig<Layered<ErrorLayer<SubscriberType>, SubscriberType>>> {
        self.with(ErrorLayer::default())
    }
}

pub struct Logger<SubscriberType = Registry>(Arc<LoggerConfig<SubscriberType>>)
where
    SubscriberType:
        Subscriber + Sized + Send + Sync + Into<Dispatch> + for<'span> LookupSpan<'span>;

impl Logger {
    pub fn new() -> LoggerConfig {
        LoggerConfig::default()
    }
}

impl<SubscriberType> Debug for Logger<SubscriberType>
where
    SubscriberType:
        Subscriber + Sized + Send + Sync + Into<Dispatch> + for<'span> LookupSpan<'span>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(std::any::type_name::<Self>()).finish()
    }
}

impl<SubscriberType> From<LoggerConfig<SubscriberType>> for Logger<SubscriberType>
where
    SubscriberType:
        Subscriber + Sized + Send + Sync + Into<Dispatch> + for<'span> LookupSpan<'span>,
{
    fn from(config: LoggerConfig<SubscriberType>) -> Self {
        Self(Arc::from(config))
    }
}

impl<SubscriberType> Clone for Logger<SubscriberType>
where
    SubscriberType:
        Subscriber + Sized + Send + Sync + Into<Dispatch> + for<'span> LookupSpan<'span>,
{
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}
