use std::{
    borrow::Cow,
    fmt::{self, Display, Formatter},
};

use super::Color;

/// The structure represents a ANSI color by suffix and prefix.
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct AnsiColor<'a> {
    prefix: Cow<'a, str>,
    suffix: Cow<'a, str>,
}

impl<'a> AnsiColor<'a> {
    /// Constructs a new instance with suffix and prefix.
    ///
    /// They are not checked so you should make sure you provide correct ANSI.
    /// Otherwise you may want to use [`TryFrom`].
    ///
    /// [`TryFrom`]: std::convert::TryFrom
    pub const fn new(prefix: Cow<'a, str>, suffix: Cow<'a, str>) -> Self {
        Self { prefix, suffix }
    }
}

impl AnsiColor<'_> {
    /// Gets a reference to a prefix.
    pub fn get_prefix(&self) -> &str {
        &self.prefix
    }

    /// Gets a reference to a suffix.
    pub fn get_suffix(&self) -> &str {
        &self.suffix
    }
}

#[cfg(feature = "color")]
impl std::convert::TryFrom<&str> for AnsiColor<'static> {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        parse_ansi_color(value).ok_or(())
    }
}

#[cfg(feature = "color")]
impl std::convert::TryFrom<String> for AnsiColor<'static> {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl Color for AnsiColor<'_> {
    fn fmt_prefix(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.prefix.fmt(f)
    }

    fn fmt_suffix(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.suffix.fmt(f)
    }
}

#[cfg(feature = "color")]
fn parse_ansi_color(s: &str) -> Option<AnsiColor<'static>> {
    let mut blocks = ansi_str::get_blocks(s);
    let block = blocks.next()?;

    let start = block.start().to_string();
    let end = block.end().to_string();

    Some(AnsiColor::new(start.into(), end.into()))
}
