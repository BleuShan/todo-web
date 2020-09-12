use crate::{
    prelude::*,
    AppState,
};
use actix_web::{
    error::ErrorInternalServerError,
    get,
    http::header::{
        AcceptLanguage,
        ContentType,
        Header,
    },
    web,
    HttpRequest,
    HttpResponse,
    Responder,
};
use mime_guess::mime::TEXT_HTML;
use todo_web_shared::views::layouts::Page;

#[instrument(skip(request, app))]
#[get("/")]
pub async fn index(request: HttpRequest, app: web::Data<AppState>) -> impl Responder {
    match AcceptLanguage::parse(&request) {
        Ok(accept_language) => {
            let lang = accept_language
                .0
                .first()
                .map(|tag| tag.item.language.clone())
                .flatten()
                .unwrap_or_else(|| "en".to_owned());

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
        Err(error) => error.error_response(),
    }
}
