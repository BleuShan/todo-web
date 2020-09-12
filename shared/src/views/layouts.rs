use super::prelude::*;
use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
    title: String,
    lang: String,
}

impl Page {
    pub fn new(title: String, lang: String) -> Self {
        Self { title, lang }
    }
}

impl TemplateData for Page {
    const TEMPLATE_NAME: &'static str = "layouts/page.hbs";
}
