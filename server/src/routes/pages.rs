use crate::{
    extractors::RequestHeader,
    prelude::*,
};
use actix_web::{
    get,
    http::header::AcceptLanguage,
    Responder,
};
use todo_web_shared::views::layouts::Page;

#[instrument(skip(accept_language))]
#[get("/")]
pub async fn index(accept_language: RequestHeader<AcceptLanguage>) -> impl Responder {
    let lang = accept_language.map_or_else(
        || "en".to_owned(),
        |value| {
            value
                .first()
                .map(|value| value.item.language.clone())
                .flatten()
                .unwrap_or_else(|| "en".to_owned())
        },
    );
    let title = env!("CARGO_PKG_NAME").to_owned();
    Page::new(title, lang)
}
