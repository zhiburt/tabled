use std::fmt::{self, Display, Formatter};

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

    fn colorize<T: Display>(&self, f: &mut Formatter<'_>, text: T) -> fmt::Result {
        C::colorize(self, f, text)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct AnsiColor {
    prefix: String,
    suffix: String,
}

impl AnsiColor {
    pub fn new(prefix: String, suffix: String) -> Self {
        Self { prefix, suffix }
    }

    pub fn get_prefix(&self) -> &str {
        &self.prefix
    }

    pub fn get_suffix(&self) -> &str {
        &self.suffix
    }
}

impl std::convert::TryFrom<&str> for AnsiColor {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        parse_ansi_color(value).ok_or(())
    }
}

impl std::convert::TryFrom<String> for AnsiColor {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl Color for AnsiColor {
    fn fmt_prefix(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.prefix.fmt(f)
    }

    fn fmt_suffix(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.suffix.fmt(f)
    }
}

fn parse_ansi_color(s: &str) -> Option<AnsiColor> {
    let mut blocks = ansi_str::get_blocks(s);
    let block = blocks.next()?;

    let start = block.start().to_string();
    let end = block.end().to_string();

    Some(AnsiColor::new(start, end))
}
