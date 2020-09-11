use crate::{
    assets::Assets,
    prelude::*,
};
use actix_web::{
    body::Body,
    get,
    http::header::{
        AcceptLanguage,
        Header,
    },
    web::{
        self,
        ServiceConfig,
    },
    HttpRequest,
    HttpResponse,
};
use std::borrow::Cow;
use todo_web_shared::views::Layout;

#[get("/")]
async fn index(request: HttpRequest) -> WebResult<Layout> {
    let lang = AcceptLanguage::parse(&request)?
        .first()
        .map(|tag| tag.clone().item.language)
        .flatten()
        .unwrap_or_else(|| "en".to_owned());
    Ok(Layout::new(lang, env!("CARGO_PKG_NAME").to_owned()))
}

#[get("/{path:.+}")]
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

pub fn root(config: &mut ServiceConfig) {
    config.service(index).service(assets);
}
