use handlebars::Handlebars;
use rust_embed::RustEmbed;

#[derive(Debug, RustEmbed)]
#[folder = "templates"]
struct Templates;
