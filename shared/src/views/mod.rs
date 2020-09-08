use askama::Template;

#[derive(Clone, Debug, Default, PartialEq, Eq, Template)]
#[template(path = "layout.html")]
pub struct Layout {
    title: String,
}

impl Layout {
    pub fn new(title: String) -> Self {
        Self { title }
    }
}
