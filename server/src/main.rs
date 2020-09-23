#![warn(missing_debug_implementations, nonstandard_style, rust_2018_idioms)]
#![feature(
    format_args_capture,
    never_type,
    trait_alias,
    box_patterns,
    box_syntax,
    type_alias_impl_trait,
    try_blocks
)]
#![type_length_limit = "3314704"]

mod assets;
mod configuration;
mod net;
mod persistence;
mod prelude;

use self::{
    configuration::Configuration,
    net::{
        connection::ConnectionHandler,
        tcp,
    },
    prelude::*,
};
use todo_web_shared::Logger;

#[async_std::main]
#[instrument(err)]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let _logger = Logger::new()
        .with_default_env_filter()?
        .with_default_output()?
        .with_default_error_layer()?
        .install()?;
    info!("Starting server.");

    let config = Configuration::current();
    let handler = match config.tls().load_server_config().await {
        Ok(tls) => ConnectionHandler::new().with_tls(tls),
        Err(_) => {
            warn!("TLS disabled.");
            ConnectionHandler::new()
        }
    }
    .build();

    let listener = tcp::bind_listener(config.socket().as_tuple()).await?;

    let mut incoming = listener.incoming();
    while let Some(result) = incoming.next().await {
        match result {
            Ok(stream) => handler.handle_connection(stream),
            Err(error) => {
                error!("{}", error);
                break;
            }
        }
    }

    Ok(())
}
