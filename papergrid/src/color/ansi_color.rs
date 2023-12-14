use std::fmt::{self, Write};

use super::{ANSIFmt, Color};

/// The structure represents a ANSI color by suffix and prefix.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColorBuf {
    prefix: String,
    suffix: String,
}

impl ColorBuf {
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
    pub fn as_ref(&self) -> Color<'_> {
        Color::new(&self.prefix, &self.suffix)
    }
}

impl ANSIFmt for ColorBuf {
    fn fmt_prefix<W: Write>(&self, f: &mut W) -> fmt::Result {
        f.write_str(&self.prefix)
    }

    fn fmt_suffix<W: Write>(&self, f: &mut W) -> fmt::Result {
        f.write_str(&self.suffix)
    }
}

#[cfg(feature = "color")]
impl std::convert::TryFrom<&str> for ColorBuf<'static> {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        parse_ansi_color(value).ok_or(())
    }
}

#[cfg(feature = "color")]
impl std::convert::TryFrom<String> for ColorBuf<'static> {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

#[cfg(feature = "color")]
fn parse_ansi_color(s: &str) -> Option<ColorBuf<'static>> {
    let mut blocks = ansi_str::get_blocks(s);
    let block = blocks.next()?;
    let style = block.style();

    let start = style.start().to_string();
    let end = style.end().to_string();

    Some(ColorBuf::new(start.into(), end.into()))
}

impl From<Color<'_>> for ColorBuf {
    fn from(value: Color<'_>) -> Self {
        Self::new(value.get_prefix(), value.get_suffix())
    }
}
