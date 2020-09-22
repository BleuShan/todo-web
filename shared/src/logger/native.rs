use super::*;

use tracing_subscriber::fmt::{
    self,
    format::{
        DefaultFields,
        Format,
    },
    Layer as FormatLayer,
    MakeWriter,
};

impl<SubscriberType> LoggerConfig<SubscriberType>
where
    SubscriberType:
        Subscriber + Sized + Send + Sync + Into<Dispatch> + for<'span> LookupSpan<'span>,
{
    pub fn install(mut self) -> Result<Logger<SubscriberType>> {
        let registry = self
            .inner
            .take()
            .ok_or_else(|| eyre!("Already installed"))?;

        registry.init();
        color_eyre::install()?;
        Ok(Logger::from(self))
    }

    pub fn with_default_output(
        mut self,
    ) -> Result<
        LoggerConfig<
            Layered<
                FormatLayer<SubscriberType, DefaultFields, Format, impl MakeWriter>,
                SubscriberType,
            >,
        >,
    > {
        let writer = self.make_default_writer()?;
        self.with_writer_factory_output(writer)
    }

    pub fn with_writer_factory_output<WriterFactory>(
        self,
        factory: WriterFactory,
    ) -> Result<
        LoggerConfig<
            Layered<
                FormatLayer<SubscriberType, DefaultFields, Format, impl MakeWriter>,
                SubscriberType,
            >,
        >,
    >
    where
        WriterFactory: MakeWriter + 'static + Send + Sync,
    {
        let layer = fmt::layer().with_writer(factory);
        self.with(layer)
    }

    cfg_not_tracing_appender! {
        fn make_default_writer(&mut self) -> Result<impl MakeWriter> {
            std::io::stderr
        }
    }

    cfg_tracing_appender! {
        pub fn with_writer_output<Writer>(
            mut self,
            writer: Writer,
        ) -> Result<
            LoggerConfig<
                Layered<
                    FormatLayer<SubscriberType, DefaultFields, Format, impl MakeWriter>,
                    SubscriberType,
                >,
            >,
        >
        where
            Writer: std::io::Write + Send + Sync + 'static,
        {
            let factory = self.wrap_writer(writer)?;
            self.with_writer_factory_output(factory)
        }

        fn make_default_writer(&mut self) -> Result<impl MakeWriter> {
            self.wrap_writer(std::io::stderr())
        }

        fn wrap_writer<Writer>(&mut self, writer: Writer) -> Result<impl MakeWriter>
        where
            Writer: std::io::Write + Send + Sync + 'static,
        {
            ensure!(self.guard.get().is_none(), "Already initialized");
            let (writer, guard) = tracing_appender::non_blocking(writer);
            self.guard.set(guard).ok();
            Ok(writer)
        }
    }
}
