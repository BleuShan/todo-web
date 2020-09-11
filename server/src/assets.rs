pub use todo_web_shared::rust_embed::{
    self,
    RustEmbed,
};

#[derive(Debug, RustEmbed)]
#[folder = "assets"]
pub struct Assets;
