use crate::prelude::*;
use actix_web::{
    error::ParseError,
    http::header::Header,
    FromRequest,
    HttpRequest,
};
use async_std::sync::Arc;
use future::Ready;

#[derive(Debug)]
#[repr(transparent)]
pub struct RequestHeader<Inner>(Arc<Option<Inner>>)
where
    Inner: Header;

impl<Inner> RequestHeader<Inner>
where
    Inner: Header,
{
    #[inline]
    pub fn get(&self) -> Option<&Inner> {
        (*self.0).as_ref()
    }

    pub fn map_or_else<U, D, F>(&self, default: D, f: F) -> U
    where
        D: FnOnce() -> U,
        F: FnOnce(&Inner) -> U,
    {
        self.get().map_or_else(default, f)
    }
}

impl<Inner> Clone for RequestHeader<Inner>
where
    Inner: Header,
{
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<Inner> From<Inner> for RequestHeader<Inner>
where
    Inner: Header,
{
    fn from(value: Inner) -> Self {
        Self(Arc::from(Some(value)))
    }
}

impl<Inner> FromRequest for RequestHeader<Inner>
where
    Inner: Header,
{
    type Error = ParseError;

    type Future = Ready<Result<Self, ParseError>>;

    type Config = ();

    fn from_request(request: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        if request.headers().get(Inner::name()).is_none() {
            return future::ready(Ok(Self(Arc::from(None))));
        }

        future::ready(Inner::parse(request).map(|header| Self::from(header)))
    }
}
