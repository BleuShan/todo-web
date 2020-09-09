use crate::prelude::*;
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    registry,
    EnvFilter,
};

#[cfg(not(target_arch = "wasm32"))]
use tracing_subscriber::fmt::{
    self,
    format::{
        DefaultFields,
        Format,
    },
    Layer as FormatLayer,
};

#[cfg(all(feature = "wasm-bindgen", target_arch = "wasm32"))]
use tracing_wasm::{
    WASMLayer,
    WASMLayerConfig,
};

#[derive(Debug)]
pub struct Logger;

impl Logger {
    pub fn init() -> Result<Self> {
        let filter_layer =
            EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info"))?;
        let registry = registry()
            .with(filter_layer)
            .with(ErrorLayer::default())
            .with(Self::make_output_layer());

        Self::install_panic_hook(registry)?;

        Ok(Self)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn install_panic_hook<RegistryType>(registry: RegistryType) -> Result<()>
    where
        RegistryType: Into<tracing::Dispatch>,
    {
        registry.init();
        color_eyre::install()?;
        Ok(())
    }

    #[cfg(all(feature = "wasm-bindgen", target_arch = "wasm32"))]
    fn install_panic_hook<RegistryType>(registry: RegistryType) -> Result<()>
    where
        RegistryType: Into<tracing::Dispatch>,
    {
        console_error_panic_hook::set_once();
        registry.init();
        color_eyre::install()?;
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn make_output_layer<S>() -> FormatLayer<S, DefaultFields, Format, impl Fn() -> std::io::Stderr>
    {
        fmt::layer().with_writer(|| std::io::stderr())
    }

    #[cfg(all(feature = "wasm-bindgen", target_arch = "wasm32"))]
    fn make_output_layer() -> WASMLayer {
        let config = WASMLayerConfig::default();
        WASMLayer::new(config)
    }
}
