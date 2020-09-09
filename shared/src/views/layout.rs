use askama::Template;

#[derive(Clone, Debug, Template)]
#[template(path = "layout.html")]
pub struct Layout {
    lang: String,
    title: String,
}

impl Layout {
    pub fn new(lang: String, title: String) -> Self {
        Self { title, lang }
    }
}
