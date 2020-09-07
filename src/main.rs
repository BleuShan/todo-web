#![forbid(future_incompatible)]
#![warn(missing_debug_implementations, nonstandard_style, rust_2018_idioms)]
#![feature(format_args_capture)]

mod prelude;

use self::prelude::*;
use actix_files as fs;
use actix_web::{
    get,
    http::ContentEncoding,
    middleware,
    App,
    HttpServer,
};
use listenfd::ListenFd;
use rustls::{
    internal::pemfile::{
        certs,
        pkcs8_private_keys,
        rsa_private_keys,
    },
    Certificate,
    NoClientAuth,
    PrivateKey,
    ServerConfig,
};
use std::{
    fs::File,
    io::BufReader,
    path::Path,
};

fn load_tls_certs<PathRef>(path: PathRef) -> Result<Vec<Certificate>>
where
    PathRef: AsRef<Path>,
{
    let cert_file = &mut BufReader::new(File::open(path)?);
    certs(cert_file).map_err(|_| eyre!("Failed to load certificates"))
}

fn load_tls_key<PathRef>(path: PathRef) -> Result<PrivateKey>
where
    PathRef: AsRef<Path>,
{
    let rsa_keys = {
        let reader = &mut BufReader::new(File::open(path.as_ref())?);
        rsa_private_keys(reader).map_err(|_| eyre!("File contains invalid rsa keys"))?
    };

    let pkcs8_keys = {
        let reader = &mut BufReader::new(File::open(path.as_ref())?);
        pkcs8_private_keys(reader).map_err(|_| eyre!("File contains invalid pkcs8 keys"))?
    };

    if let Some(key) = pkcs8_keys.first() {
        Ok(key.clone())
    } else if let Some(key) = rsa_keys.first() {
        Ok(key.clone())
    } else {
        Err(eyre!("No key found"))
    }
}

#[get("/")]
async fn index() -> &'static str {
    "Hello World!"
}

#[actix_web::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let mut server = HttpServer::new(|| {
        let files = fs::Files::new("/", "./assets").index_file("index.html");
        App::new()
            .default_service(files)
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::new(ContentEncoding::Auto))
    });


    let mut config = ServerConfig::new(NoClientAuth::new());
    config.set_single_cert(
        load_tls_certs("localhost.pem")?,
        load_tls_key("localhost-key.pem")?,
    )?;

    let mut listenfd = ListenFd::from_env();
    server = if let Some(listener) = listenfd.take_tcp_listener(0)? {
        server.listen_rustls(listener, config)?
    } else {
        server.bind_rustls("localhost:3000", config)?
    };

    server.run().await.map_err(|error| Error::from(error))
}
