pub use async_std::prelude::*;
pub use std::pin::Pin;
pub use todo_web_shared::prelude::*;

pub type WebError = actix_web::Error;

pub type WebResult<T, E = WebError> = Result<T, E>;
