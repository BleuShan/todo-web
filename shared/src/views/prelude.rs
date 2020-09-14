pub use askama::{
    self,
    SizedTemplate,
    Template,
};
#[cfg(feature = "actix")]
pub use askama_actix::{
    self,
    TemplateIntoResponse,
};
