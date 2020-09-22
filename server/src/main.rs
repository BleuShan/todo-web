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
use tokio::signal;

#[tokio::main]
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

    let mut listener = tcp::bind_listener(config.socket().as_tuple()).await?;
    loop {
        let next = listener.next();
        select! {
            maybe_result = next => {
                match maybe_result {
                    Some(Ok(stream)) => {
                        handler.handle_connection(stream)
                    }
                    Some(Err(error)) => {
                        error!("{}", error);
                    }
                    None  => {
                        info!("TcpListener terminated. Exiting.");
                        break;
                    }
                }
            }
            Ok(()) = signal::ctrl_c() => {
                info!("Exit signal received. Exiting.");
                break;
            }
        }
    }

    Ok(())
}
