use actix_web::{
    body::Body,
    get,
    web,
    HttpResponse,
};
use std::borrow::Cow;
pub use todo_web_shared::rust_embed::{
    self,
    RustEmbed,
};

#[derive(Debug, RustEmbed)]
#[folder = "assets"]
pub struct Assets;

#[get("/{path}")]
pub async fn assets(url: web::Path<(String,)>) -> HttpResponse {
    let (path,) = url.into_inner();
    match Assets::get(&path) {
        Some(content) => {
            let body: Body = match content {
                Cow::Borrowed(bytes) => bytes.into(),
                Cow::Owned(bytes) => bytes.into(),
            };
            HttpResponse::Ok()
                .content_type(
                    mime_guess::from_path(&path)
                        .first_or_octet_stream()
                        .as_ref(),
                )
                .body(body)
        }
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}
