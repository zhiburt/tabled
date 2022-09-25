use std::fmt::{self, Display, Formatter};

#[cfg(feature = "color")]
mod ansi_color;

#[cfg(feature = "color")]
pub use ansi_color::AnsiColor;

#[allow(unreachable_pub)]
/// A trait which prints an ANSI prefix and suffix.
pub trait Color {
    /// Print ANSI prefix.
    fn fmt_prefix(&self, f: &mut Formatter<'_>) -> fmt::Result;

    /// Print ANSI suffix.
    fn fmt_suffix(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("\u{1b}[0m")
    }

    /// Print colored text.
    ///
    /// It may not handle `\n` (new lines).
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
