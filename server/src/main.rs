#![forbid(future_incompatible)]
#![warn(missing_debug_implementations, nonstandard_style, rust_2018_idioms)]
#![feature(
    format_args_capture,
    never_type,
    trait_alias,
    box_patterns,
    box_syntax,
    type_alias_impl_trait
)]

mod app;
mod assets;
mod configuration;
mod http;
mod persistence;
mod prelude;
mod routes;
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
use app::AppData;
use async_std::io;
use listenfd::ListenFd;
use todo_web_shared::Logger;

#[instrument]
#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let _logger = Logger::new()
        .with_default_env_filter()?
        .with_default_output()?
        .with_default_error_layer()?
        .install()?;
    let config = Configuration::load();
    let app_data = AppData::load(&config).await?;

    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::new(ContentEncoding::Auto))
            .configure(routes::pages)
            .configure(routes::files)
    });

    info!("Starting http server.");
    let mut listenfd = ListenFd::from_env();
    server = match config.tls().load_server_config().await {
        Ok(configuration) => {
            if let Some(listener) = listenfd.take_tcp_listener(0)? {
                server.listen_rustls(listener, configuration)?
            } else {
                server.bind_rustls(config.socket(), configuration)?
            }
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            warn!("No TLS configuration loaded https will be unavailable.");
            if let Some(listener) = listenfd.take_tcp_listener(0)? {
                server.listen(listener)?
            } else {
                server.bind(config.socket())?
            }
        }
        Err(e) => return Err(e.into()),
    };

    server.run().await?;
    Ok(())
}
