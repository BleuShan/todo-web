#![forbid(future_incompatible)]
#![warn(missing_debug_implementations, nonstandard_style, rust_2018_idioms)]
#![feature(format_args_capture)]

mod logger;
mod prelude;
mod tls;

use self::{
    logger::Logger,
    prelude::*,
};
use actix_files as fs;
use actix_web::{
    get,
    http::ContentEncoding,
    middleware,
    App,
    HttpServer,
};
use listenfd::ListenFd;

#[get("/")]
async fn index() -> &'static str {
    "Hello World!"
}

#[actix_web::main]
async fn main() -> Result<()> {
    let _logger = Logger::init()?;
    color_eyre::install()?;
    let config = tls::config().await?;
    let mut server = HttpServer::new(|| {
        let files = fs::Files::new("/", "./assets").index_file("index.html");
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::new(ContentEncoding::Auto))
            .default_service(files)
    });



    let mut listenfd = ListenFd::from_env();
    server = if let Some(listener) = listenfd.take_tcp_listener(0)? {
        server.listen_rustls(listener, config)?
    } else {
        server.bind_rustls("localhost:3000", config)?
    };

    server.run().await?;
    Ok(())
}
