use super::*;

impl<SubscriberType> LoggerConfig<SubscriberType>
where
    SubscriberType:
        Subscriber + Sized + Send + Sync + Into<Dispatch> + for<'span> LookupSpan<'span>,
{
    pub fn install(mut self) -> Result<Logger<SubscriberType>> {
        console_error_panic_hook::set_once();
        let registry = self
            .inner
            .take()
            .ok_or_else(|| eyre!("Already installed"))?;
        registry.init();
        color_eyre::install()?;
        Ok(Logger::from(self))
    }

    pub fn with_default_output(self) -> Result<LoggerConfig<Layered<WASMLayer, SubscriberType>>> {
        let config = WASMLayerConfig::default();
        self.with(WASMLayer::new(config))
    }
}
