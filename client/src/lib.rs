#![forbid(future_incompatible)]
#![warn(missing_debug_implementations, nonstandard_style, rust_2018_idioms)]
#![feature(
    format_args_capture,
    never_type,
    trait_alias,
    box_patterns,
    box_syntax,
    type_alias_impl_trait
)]

mod prelude;

use self::prelude::*;
use todo_web_shared::LoggerConfig;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc<'_> = WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn start() -> JSResult<()> {
    let _logger = LoggerConfig::default()
        .with_default_env_filter()
        .map_err(|_| "Failed to initialize logger")?
        .with_default_error_layer()
        .map_err(|_| "Failed to initialize logger")?
        .with_default_output()
        .map_err(|_| "Failed to initialize logger")?
        .install()
        .expect_throw("Failed to install logger");

    Ok(())
}

#[wasm_bindgen]
pub fn render(selector: &str) -> JSResult<()> {
    let window = web_sys::window().expect("Failed to acquire window");
    let document = window.document().expect("Failed to acquire document");

    let root = document
        .query_selector(&selector)?
        .expect("Failed to acquire root");

    while let Some(ref child) = root.first_child() {
        root.remove_child(child)?;
    }

    let text = document.create_text_node("Hello World!");
    let content = document.create_element("h1")?;
    content.append_child(&text)?;
    root.append_child(&content)?;
    info!("Rendered");

    Ok(())
}
