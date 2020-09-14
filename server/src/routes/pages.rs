use crate::{
    http::{
        header::{
            AcceptLanguage,
            ContentType,
        },
        RequestHeader,
    },
    prelude::*,
    AppData,
};
use actix_web::{
    error::ErrorInternalServerError,
    get,
    HttpResponse,
    Responder,
};
use mime_guess::mime::TEXT_HTML;
use todo_web_shared::views::layouts::Page;

#[instrument(skip(app, accept_language))]
#[get("/")]
pub async fn index(app: AppData, accept_language: RequestHeader<AcceptLanguage>) -> impl Responder {
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
    let page = Page::new(title, lang);

    match app.renderer().render_template_data(&page) {
        Ok(content) => HttpResponse::Ok().set(ContentType(TEXT_HTML)).body(content),
        Err(report) => {
            error!("{:?}", report);

            ErrorInternalServerError(report)
                .as_response_error()
                .error_response()
        }
    }
}
