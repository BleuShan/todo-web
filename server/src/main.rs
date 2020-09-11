#![forbid(future_incompatible)]
#![warn(missing_debug_implementations, nonstandard_style, rust_2018_idioms)]
#![feature(format_args_capture)]

mod assets;
mod prelude;
mod routes;
mod tls;

use self::prelude::*;
use actix_web::{
    http::ContentEncoding,
    middleware,
    App,
    HttpServer,
};
use listenfd::ListenFd;
use todo_web_shared::Logger;

#[actix_rt::main]
async fn main() -> Result<()> {
    let _logger = Logger::init()?;
    let config = tls::config().await?;
    let mut server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::new(ContentEncoding::Auto))
            .configure(routes::root)
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
