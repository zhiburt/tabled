//! A module which contains [`ANSIFmt`] trait and its implementation [`ANSIStr`]
#[cfg_attr(feature = "std", doc = "and [`ANSIBuf`].")]
#[cfg(feature = "std")]
mod ansi_buf;
mod ansi_str;

#[cfg(feature = "std")]
pub use ansi_buf::ANSIBuf;

pub use self::ansi_str::ANSIStr;

use core::fmt::{self, Write};

/// A trait which prints an ANSI prefix and suffix.
pub trait ANSIFmt {
    /// Print ANSI prefix.
    fn fmt_ansi_prefix<W: Write>(&self, f: &mut W) -> fmt::Result;

    /// Print ANSI suffix.
    fn fmt_ansi_suffix<W: Write>(&self, f: &mut W) -> fmt::Result {
        f.write_str("\u{1b}[0m")
    }
}

impl<C> ANSIFmt for &C
where
    C: ANSIFmt,
{
    fn fmt_ansi_prefix<W: Write>(&self, f: &mut W) -> fmt::Result {
        C::fmt_ansi_prefix(self, f)
    }

    fn fmt_ansi_suffix<W: Write>(&self, f: &mut W) -> fmt::Result {
        C::fmt_ansi_suffix(self, f)
    }
}
