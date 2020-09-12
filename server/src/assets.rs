pub use crate::prelude::*;
use rust_embed::RustEmbed;

#[derive(Debug, RustEmbed)]
#[folder = "assets"]
pub struct Assets;
