pub use super::TemplateData;
use crate::prelude::*;
pub use handlebars::{
    DecoratorDef,
    HelperDef,
    JsonRender,
    RenderError,
    Renderable,
    Template,
    TemplateFileError,
    TemplateRenderError,
};

pub trait RegistrableHelperDef = HelperDef + SendSync;
pub type RenderResult<T> = Result<T, RenderError>;
pub type TemplateRenderResult<T> = Result<T, TemplateRenderError>;
pub type TemplateFileResult<T> = Result<T, TemplateFileError>;
