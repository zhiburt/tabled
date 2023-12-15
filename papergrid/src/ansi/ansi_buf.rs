use std::fmt::{self, Write};

use super::{ANSIFmt, ANSIStr};

/// The structure represents a ANSI color by suffix and prefix.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ANSIBuf {
    prefix: String,
    suffix: String,
}

impl ANSIBuf {
    /// Constructs a new instance with suffix and prefix.
    ///
    /// They are not checked so you should make sure you provide correct ANSI.
    /// Otherwise you may want to use [`TryFrom`].
    ///
    /// [`TryFrom`]: std::convert::TryFrom
    pub fn new<P, S>(prefix: P, suffix: S) -> Self
    where
        P: Into<String>,
        S: Into<String>,
    {
        let prefix = prefix.into();
        let suffix = suffix.into();

        Self { prefix, suffix }
    }

    /// Checks whether the color is not actually set.
    pub fn is_empty(&self) -> bool {
        self.prefix.is_empty() && self.suffix.is_empty()
    }

    /// Gets a reference to a prefix.
    pub fn get_prefix(&self) -> &str {
        &self.prefix
    }

    /// Gets a reference to a suffix.
    pub fn get_suffix(&self) -> &str {
        &self.suffix
    }

    /// Gets a reference as a color.
    pub fn as_ref(&self) -> ANSIStr<'_> {
        ANSIStr::new(&self.prefix, &self.suffix)
    }
}

impl ANSIFmt for ANSIBuf {
    fn fmt_ansi_prefix<W: Write>(&self, f: &mut W) -> fmt::Result {
        f.write_str(&self.prefix)
    }

    fn fmt_ansi_suffix<W: Write>(&self, f: &mut W) -> fmt::Result {
        f.write_str(&self.suffix)
    }
}

#[cfg(feature = "ansi")]
impl std::convert::TryFrom<&str> for ANSIBuf {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        parse_ansi_color(value).ok_or(())
    }
}

#[cfg(feature = "ansi")]
impl std::convert::TryFrom<String> for ANSIBuf {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

#[cfg(feature = "ansi")]
fn parse_ansi_color(s: &str) -> Option<ANSIBuf> {
    let mut blocks = ansi_str::get_blocks(s);
    let block = blocks.next()?;
    let style = block.style();

    let start = style.start().to_string();
    let end = style.end().to_string();

    Some(ANSIBuf::new(start, end))
}

impl From<ANSIStr<'_>> for ANSIBuf {
    fn from(value: ANSIStr<'_>) -> Self {
        Self::new(value.get_prefix(), value.get_suffix())
    }
}
