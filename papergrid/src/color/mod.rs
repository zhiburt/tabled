use std::fmt::{self, Display, Formatter};

#[cfg(feature = "color")]
pub mod ansi_color;

pub trait Color {
    fn fmt_prefix(&self, f: &mut Formatter<'_>) -> fmt::Result;

    fn fmt_suffix(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("\u{1b}[0m")
    }

    fn colorize<T>(&self, f: &mut Formatter<'_>, text: T) -> fmt::Result
    where
        T: Display,
    {
        self.fmt_prefix(f)?;
        text.fmt(f)?;
        self.fmt_suffix(f)?;
        Ok(())
    }
}

impl<C> Color for &C
where
    C: Color,
{
    fn fmt_prefix(&self, f: &mut Formatter<'_>) -> fmt::Result {
        C::fmt_prefix(self, f)
    }

    fn fmt_suffix(&self, f: &mut Formatter<'_>) -> fmt::Result {
        C::fmt_suffix(self, f)
    }

    fn colorize<T>(&self, f: &mut Formatter<'_>, text: T) -> fmt::Result
    where
        T: Display,
    {
        C::colorize(self, f, text)
    }
}
