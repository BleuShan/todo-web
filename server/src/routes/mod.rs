mod files;
mod pages;

use actix_web::web::ServiceConfig;

pub fn pages(config: &mut ServiceConfig) {
    config.service(pages::index);
}

pub fn files(config: &mut ServiceConfig) {
    config.service(files::assets);
}
