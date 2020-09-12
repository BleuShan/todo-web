use crate::prelude::*;
use rust_embed::RustEmbed;
use std::io::{
    self,
    prelude::*,
};

#[derive(Debug, RustEmbed)]
#[folder = "templates"]
pub(super) struct TemplateSourceFiles;

#[derive(Debug)]
#[repr(transparent)]
pub struct TemplateSourceFile {
    data: Vec<u8>,
}

impl TemplateSourceFile {
    pub fn open<PathRef>(filename: PathRef) -> io::Result<Self>
    where
        PathRef: AsRef<str>,
    {
        match TemplateSourceFiles::get(filename.as_ref()) {
            Some(bytes) => Ok(bytes.into()),
            None => Err(io::ErrorKind::NotFound.into()),
        }
    }
}

impl<SourceBytes> From<SourceBytes> for TemplateSourceFile
where
    Vec<u8>: From<SourceBytes>,
{
    fn from(source: SourceBytes) -> Self {
        let data = Vec::from(source);
        Self { data }
    }
}

impl Read for TemplateSourceFile {
    fn read(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
        if self.data.is_empty() {
            return Ok(0);
        }
        let written = buf.write(&self.data)?;
        let mid = self.data.len() - written;
        self.data.rotate_left(written);
        drop(self.data.split_off(mid));

        Ok(written)
    }
}

pub trait TemplateData
where
    Self: SendSync + Serialize,
{
    const TEMPLATE_NAME: &'static str;
}
