pub use js_sys::JsString;
pub use todo_web_shared::prelude::*;
pub use wasm_bindgen::{
    prelude::*,
    JsCast,
};
pub use wasm_bindgen_futures::JsFuture;
pub type JSResult<T> = Result<T, JsValue>;
