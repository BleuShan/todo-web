use super::prelude::*;
use crate::prelude::*;

#[derive(Debug, Template)]
#[template(path = "layouts/page.jinja")]
pub struct Page {
    title: String,
    lang: String,
}

impl Page {
    pub fn new(title: String, lang: String) -> Self {
        Self { title, lang }
    }
}
