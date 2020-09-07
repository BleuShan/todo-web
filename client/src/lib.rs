#![forbid(future_incompatible)]
#![warn(missing_debug_implementations, nonstandard_style, rust_2018_idioms)]
#![feature(format_args_capture)]

mod prelude;

use self::prelude::*;
use console_error_panic_hook;
use web_sys::HtmlElement;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc<'_> = WeeAlloc::INIT;

#[wasm_bindgen]
pub fn render(root: HtmlElement) -> JSResult<()> {
    console_error_panic_hook::set_once();
    let window = web_sys::window().expect("Failed to acquire window");
    let document = window.document().expect("Failed to acquire document");

    while let Some(ref child) = root.first_child() {
        root.remove_child(child)?;
    }

    let text = document.create_text_node("Hi!");
    let content = document.create_element("h1")?;
    content.append_child(&text)?;
    root.append_child(&content)?;


    Ok(())
}
