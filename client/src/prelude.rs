pub use wasm_bindgen::{
    prelude::*,
    JsCast,
};
pub use wasm_bindgen_futures::JsFuture;
pub type JSResult<T> = Result<T, JsValue>;
