use super::prelude::*;
use crate::prelude::*;
use bytes::{
    Buf,
    Bytes,
};
use rust_embed::RustEmbed;
use std::{
    borrow::Cow,
    cmp,
    io::{
        self,
        prelude::*,
    },
};

#[derive(Debug, RustEmbed)]
#[folder = "templates"]
pub(super) struct TemplateSourceFiles;

#[derive(Debug)]
#[repr(transparent)]
pub struct TemplateSourceFile(Bytes);

impl TemplateSourceFile {
    pub fn open<PathRef>(filename: PathRef) -> TemplateFileResult<Self>
    where
        PathRef: AsRef<str>,
    {
        match TemplateSourceFiles::get(filename.as_ref()) {
            Some(bytes) => Ok(bytes.into()),
            None => Err(TemplateFileError::IOError(
                io::ErrorKind::NotFound.into(),
                filename.as_ref().to_string(),
            )),
        }
    }
}

impl From<Cow<'static, [u8]>> for TemplateSourceFile {
    fn from(data: Cow<'static, [u8]>) -> Self {
        let bytes = match data {
            Cow::Borrowed(source) => Bytes::from(source),
            Cow::Owned(source) => Bytes::from(source),
        };
        Self(bytes)
    }
}

impl Read for TemplateSourceFile {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let len = cmp::min(self.0.remaining(), buf.len());
        self.0.copy_to_slice(&mut buf[0..len]);
        Ok(len)
    }
}

impl BufRead for TemplateSourceFile {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        Ok(self.0.bytes())
    }

    fn consume(&mut self, count: usize) {
        self.0.advance(count)
    }
}

pub trait TemplateData
where
    Self: SendSync + Serialize,
{
    const TEMPLATE_NAME: &'static str;
}
