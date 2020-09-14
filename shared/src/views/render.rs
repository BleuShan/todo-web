use super::{
    prelude::*,
    templates::{
        TemplateSourceFile,
        TemplateSourceFiles,
    },
};
use crate::prelude::*;
use handlebars::Handlebars;
use parking_lot::RwLock;
use std::{
    io::prelude::*,
    sync::Arc,
};

#[derive(Debug)]
#[repr(transparent)]
pub struct Renderer(Arc<RwLock<Handlebars<'static>>>);

impl Clone for Renderer {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl From<Handlebars<'static>> for Renderer {
    fn from(registry: Handlebars<'static>) -> Self {
        Self(Arc::from(RwLock::from(registry)))
    }
}

impl Renderer {
    #[instrument(err)]
    pub fn new() -> TemplateFileResult<Self> {
        info!("Initializing template registry.");
        let mut registry = Handlebars::new();

        registry.register_escape_fn(|s| v_htmlescape::escape(s).to_string());
        info!("Registering templates");
        for ref filename in TemplateSourceFiles::iter() {
            let mut file = TemplateSourceFile::open(filename)?;
            registry.register_template_source(filename, &mut file)?;
            debug!(r#"registered: "{}""#, filename);
        }

        Ok(registry.into())
    }

    pub fn render<Ser>(&self, name: &str, data: &Ser) -> TemplateRenderResult<String>
    where
        Ser: Serialize,
    {
        let registry = self.0.read();
        Ok(registry.render(name, data)?)
    }

    pub fn render_template_data<Target>(&self, data: &Target) -> TemplateRenderResult<String>
    where
        Target: TemplateData,
    {
        self.render(Target::TEMPLATE_NAME, data)
    }

    pub fn render_template_data_to_write<Target, Writer>(
        &self,
        data: &Target,
        writer: Writer,
    ) -> TemplateRenderResult<()>
    where
        Target: TemplateData,
        Writer: Write,
    {
        self.render_to_write(Target::TEMPLATE_NAME, data, writer)
    }

    pub fn render_to_write<Ser, Writer>(
        &self,
        name: &str,
        data: &Ser,
        writer: Writer,
    ) -> TemplateRenderResult<()>
    where
        Ser: Serialize,
        Writer: Write,
    {
        let registry = self.0.read();
        Ok(registry.render_to_write(name, data, writer)?)
    }

    pub fn register_helper<HelperDefinition>(
        &self,
        name: &str,
        def: HelperDefinition,
    ) -> Option<Box<dyn RegistrableHelperDef + 'static>>
    where
        HelperDefinition: RegistrableHelperDef + 'static,
    {
        let mut registry = self.0.write();
        registry.register_helper(name, Box::new(def))
    }
}
