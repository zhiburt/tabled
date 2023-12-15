use core::fmt::{self, Write};

use super::ANSIFmt;

/// The structure represents a ANSI color by suffix and prefix.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct ANSIStr<'a> {
    prefix: &'a str,
    suffix: &'a str,
}

impl<'a> ANSIStr<'a> {
    /// Constructs a new instance with suffix and prefix.
    ///
    /// They are not checked so you should make sure you provide correct ANSI.
    /// Otherwise you may want to use [`TryFrom`].
    ///
    /// [`TryFrom`]: std::convert::TryFrom
    pub const fn new(prefix: &'a str, suffix: &'a str) -> Self {
        Self { prefix, suffix }
    }

    /// Verifies if anything was actually set.
    pub const fn is_empty(&self) -> bool {
        self.prefix.is_empty() && self.suffix.is_empty()
    }

    /// Gets a reference to a prefix.
    pub fn get_prefix(&self) -> &'a str {
        self.prefix
    }

    /// Gets a reference to a suffix.
    pub fn get_suffix(&self) -> &'a str {
        self.suffix
    }
}

impl ANSIFmt for ANSIStr<'_> {
    fn fmt_ansi_prefix<W: Write>(&self, f: &mut W) -> fmt::Result {
        f.write_str(self.prefix)
    }

    fn fmt_ansi_suffix<W: Write>(&self, f: &mut W) -> fmt::Result {
        f.write_str(self.suffix)
    }
}
