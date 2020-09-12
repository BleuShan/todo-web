pub use super::TemplateData;
use crate::prelude::*;
pub use handlebars::{
    DecoratorDef,
    HelperDef,
    JsonRender,
    Renderable,
    Template,
};

pub trait RegistrableHelperDef = HelperDef + SendSync;
