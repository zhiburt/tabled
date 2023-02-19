use std::fmt::Result;
use std::fmt::{self, Error};
use std::io;

pub(super) struct UTF8Writer<W>(W);

impl<W> UTF8Writer<W> {
    pub(crate) fn new(writer: W) -> Self {
        Self(writer)
    }
}

impl<W> fmt::Write for UTF8Writer<W>
where
    W: io::Write,
{
    fn write_str(&mut self, s: &str) -> Result {
        let mut buf = s.as_bytes();
        loop {
            let n = self.0.write(buf).map_err(|_| Error::default())?;
            if n == buf.len() {
                break;
            }

            buf = &buf[n..];
        }

        Ok(())
    }
}
