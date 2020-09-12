use crate::assets::Assets;
use actix_web::{
    body::Body,
    get,
    web::{
        self,
        ServiceConfig,
    },
    HttpRequest,
    HttpResponse,
    Responder,
};
use std::borrow::Cow;

#[get("/")]
async fn index(request: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/{path:.+}")]
pub async fn assets(url: web::Path<(String,)>) -> impl Responder {
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

pub fn routes(config: &mut ServiceConfig) {
    config.service(index).service(assets);
}
