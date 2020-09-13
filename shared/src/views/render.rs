use super::{
    prelude::*,
    templates::{
        TemplateSourceFile,
        TemplateSourceFiles,
    },
};
use crate::prelude::*;
use handlebars::Handlebars;
use once_cell::sync::OnceCell;
use std::{
    io::prelude::*,
    sync::{
        Arc,
        RwLock,
    },
};

static CURRENT_RENDERER_STATE: OnceCell<Arc<RendererState<'static>>> = OnceCell::new();

#[derive(Debug)]
struct RendererState<'state> {
    registry: RwLock<Handlebars<'state>>,
}

impl<'state> RendererState<'state> {
    #[instrument]
    fn new() -> Result<Self> {
        info!("Initializing template registry.");
        let mut registry = Handlebars::new();

        registry.register_escape_fn(|s| v_htmlescape::escape(s).to_string());
        info!("Registering templates");
        for ref filename in TemplateSourceFiles::iter() {
            let mut file = TemplateSourceFile::open(filename)?;
            registry.register_template_source(filename, &mut file)?;
            debug!(r#"registred: "{}""#, filename);
        }

        Ok(Self {
            registry: RwLock::from(registry),
        })
    }

    fn read_registry<RegistryFn, RegistryFnOutput>(
        &self,
        callback: RegistryFn,
    ) -> Result<RegistryFnOutput>
    where
        RegistryFn: FnOnce(&Handlebars<'_>) -> Result<RegistryFnOutput>,
    {
        let reader = self
            .registry
            .read()
            .map_err(|_| eyre!("Failed to acquire registry reader"))?;
        callback(&reader)
    }

    fn update_registry<RegistryFn, RegistryFnOutput>(
        &self,
        callback: RegistryFn,
    ) -> Result<RegistryFnOutput>
    where
        RegistryFn: FnOnce(&mut Handlebars<'state>) -> RegistryFnOutput,
    {
        let mut writer = self
            .registry
            .write()
            .map_err(|_| eyre!("Failed to acquire registry writer"))?;
        Ok(callback(&mut writer))
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Renderer(Arc<RendererState<'static>>);

impl Clone for Renderer {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl Renderer {
    #[instrument]
    pub fn current() -> Result<Self> {
        CURRENT_RENDERER_STATE
            .get_or_try_init(|| {
                info!("initializing global view renderer");
                let state: RendererState<'static> = RendererState::new()?;
                Ok(Arc::from(state))
            })
            .map(|state| Self(Arc::clone(state)))
    }

    pub fn render<Ser>(&self, name: &str, data: &Ser) -> Result<String>
    where
        Ser: Serialize,
    {
        self.0.read_registry(|registry| {
            registry
                .render(name, data)
                .map_err(|error| eyre!("Failed to render template.").error(error))
        })
    }

    pub fn render_template_data<Target>(&self, data: &Target) -> Result<String>
    where
        Target: TemplateData,
    {
        self.render(Target::TEMPLATE_NAME, data)
    }

    pub fn render_template_data_to_write<Target, Writer>(
        &self,
        data: &Target,
        writer: Writer,
    ) -> Result<()>
    where
        Target: TemplateData,
        Writer: Write,
    {
        self.render_to_write(Target::TEMPLATE_NAME, data, writer)
    }

    pub fn render_to_write<Ser, Writer>(&self, name: &str, data: &Ser, writer: Writer) -> Result<()>
    where
        Ser: Serialize,
        Writer: Write,
    {
        self.0.read_registry(move |registry| {
            registry
                .render_to_write(name, data, writer)
                .map_err(|error| eyre!("Failed to render template.").error(error))
        })
    }

    pub fn register_helper<HelperDefinition>(
        &self,
        name: &str,
        def: HelperDefinition,
    ) -> Result<Option<Box<dyn RegistrableHelperDef + 'static>>>
    where
        HelperDefinition: RegistrableHelperDef + 'static,
    {
        self.0
            .update_registry(|registry| registry.register_helper(name, Box::new(def)))
    }
}
