#![forbid(future_incompatible)]
#![warn(missing_debug_implementations, nonstandard_style, rust_2018_idioms)]
#![feature(format_args_capture)]

mod api;
mod assets;
mod configuration;
mod prelude;
mod tls;

use self::{
    configuration::Configuration,
    prelude::*,
};
use actix_web::{
    http::ContentEncoding,
    middleware,
    App,
    HttpServer,
};
use listenfd::ListenFd;
use todo_web_shared::Logger;

#[actix_web::main]
#[instrument]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let _logger = Logger::new()
        .with_default_env_filter()?
        .with_default_error_layer()?
        .with_default_output()?
        .install()?;
    let config = Configuration::load();

    let mut server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::new(ContentEncoding::Auto))
            .configure(api::routes)
    });

    let mut listenfd = ListenFd::from_env();
    server = match config.tls().load_server_config().await {
        Ok(configuration) => {
            if let Some(listener) = listenfd.take_tcp_listener(0)? {
                server.listen_rustls(listener, configuration)?
            } else {
                server.bind_rustls(config.socket(), configuration)?
            }
        }
        Err(error) => {
            error!("{}", error);
            if let Some(listener) = listenfd.take_tcp_listener(0)? {
                server.listen(listener)?
            } else {
                server.bind(config.socket())?
            }
        }
    };

    server.run().await?;
    Ok(())
}
