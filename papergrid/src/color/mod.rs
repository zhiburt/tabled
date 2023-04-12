//! A module which contains [`Color`] trait and its implementation [`AnsiColor`].

#[cfg(feature = "std")]
mod ansi_color;
mod static_color;

#[cfg(feature = "std")]
pub use ansi_color::AnsiColor;

pub use static_color::StaticColor;

use core::fmt::{self, Write};

#[allow(unreachable_pub)]
/// A trait which prints an ANSI prefix and suffix.
pub trait Color {
    /// Print ANSI prefix.
    fn fmt_prefix<W: Write>(&self, f: &mut W) -> fmt::Result;

    /// Print ANSI suffix.
    fn fmt_suffix<W: Write>(&self, f: &mut W) -> fmt::Result {
        f.write_str("\u{1b}[0m")
    }

    /// Print colored text.
    ///
    /// It may not handle `\n` (new lines).
    fn colorize<W: Write>(&self, f: &mut W, text: &str) -> fmt::Result {
        self.fmt_prefix(f)?;
        f.write_str(text)?;
        self.fmt_suffix(f)?;
        Ok(())
    }
}

impl<C> Color for &C
where
    C: Color,
{
    fn fmt_prefix<W: Write>(&self, f: &mut W) -> fmt::Result {
        C::fmt_prefix(self, f)
    }

    fn fmt_suffix<W: Write>(&self, f: &mut W) -> fmt::Result {
        C::fmt_suffix(self, f)
    }

    fn colorize<W: Write>(&self, f: &mut W, text: &str) -> fmt::Result {
        C::colorize(self, f, text)
    }
}
