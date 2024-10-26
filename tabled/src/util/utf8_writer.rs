use std::fmt;
use std::io;

pub(crate) struct UTF8Writer<W>(W);

impl<W> UTF8Writer<W> {
    pub(crate) fn new(writer: W) -> Self {
        Self(writer)
    }
}

impl<W> fmt::Write for UTF8Writer<W>
where
    W: io::Write,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut buf = s.as_bytes();
        loop {
            let n = self.0.write(buf).map_err(|_| fmt::Error)?;
            if n == buf.len() {
                break;
            }

            buf = &buf[n..];
        }

        Ok(())
    }
}
