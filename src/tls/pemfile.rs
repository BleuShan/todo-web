use crate::prelude::*;
use async_std::io::BufRead;
use base64;
use rustls::{
    Certificate,
    PrivateKey,
};

/// Extract and decode all PEM sections from `rd`, which begin with `start_mark`
/// and end with `end_mark`.  Apply the functor `f` to each decoded buffer,
/// and return a Vec of `f`'s return values.
async fn extract<'a, A, F, R>(
    mut rd: Pin<Box<R>>,
    start_mark: &str,
    end_mark: &str,
    f: F,
) -> Result<Vec<A>>
where
    F: Fn(Vec<u8>) -> A,
    R: BufRead + 'a,
{
    let mut ders = Vec::new();
    let mut b64buf = String::new();
    let mut take_base64 = false;

    let mut raw_line = Vec::<u8>::new();
    loop {
        raw_line.clear();
        let len = rd.read_until(b'\n', &mut raw_line).await?;

        if len == 0 {
            return Ok(ders);
        }
        let line = String::from_utf8_lossy(&raw_line);

        if line.starts_with(start_mark) {
            take_base64 = true;
            continue;
        }

        if line.starts_with(end_mark) {
            take_base64 = false;
            let der = base64::decode(&b64buf)?;
            ders.push(f(der));
            b64buf = String::new();
            continue;
        }

        if take_base64 {
            b64buf.push_str(line.trim());
        }
    }
}

/// Extract all the certificates from rd, and return a vec of `key::Certificate`s
/// containing the der-format contents.
pub async fn certs<'a, R>(rd: Pin<Box<R>>) -> Result<Vec<Certificate>>
where
    R: BufRead + 'a,
{
    extract(
        rd,
        "-----BEGIN CERTIFICATE-----",
        "-----END CERTIFICATE-----",
        |v| Certificate(v),
    )
    .await
}

/// Extract all RSA private keys from rd, and return a vec of `key::PrivateKey`s
/// containing the der-format contents.
pub async fn rsa_private_keys<'a, R>(rd: Pin<Box<R>>) -> Result<Vec<PrivateKey>>
where
    R: BufRead + 'a,
{
    extract(
        rd,
        "-----BEGIN RSA PRIVATE KEY-----",
        "-----END RSA PRIVATE KEY-----",
        |v| PrivateKey(v),
    )
    .await
}

/// Extract all PKCS8-encoded private keys from rd, and return a vec of
/// `key::PrivateKey`s containing the der-format contents.
pub async fn pkcs8_private_keys<'a, R>(rd: Pin<Box<R>>) -> Result<Vec<PrivateKey>>
where
    R: BufRead + 'a,
{
    extract(
        rd,
        "-----BEGIN PRIVATE KEY-----",
        "-----END PRIVATE KEY-----",
        |v| PrivateKey(v),
    )
    .await
}
