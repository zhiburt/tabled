//! This module contains a colored representation of a char which we call [`Symbol`].

use papergrid::util::string::string_width;

use crate::color::Color;

/// Symbol represents a character of a border.
///
/// It's only needed when used with `color` feature flag.
///
/// ```rust,no_run
/// # use owo_colors::OwoColorize;
/// # use tabled::{style::{Symbol, BorderColored}, object::Rows, TableIteratorExt, Modify};
/// #
/// # let data: Vec<&'static str> = Vec::new();
/// #
/// let colored_char = "#".red().to_string();
/// let table = data.table()
///     .with(Modify::new(Rows::single(0)).with(BorderColored::filled(Symbol::ansi(colored_char).unwrap())));
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "color")))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Symbol {
    c: char,
    color: Option<Color>,
}

impl Symbol {
    /// Creates a new [Symbol] which represents a colored char.
    pub const fn new(c: char, color: Option<Color>) -> Self {
        Self { c, color }
    }

    /// Creates a new [`Symbol`] from the String.
    /// The string must contain 1 UTF-8 character and any list of Ansi sequences.
    ///
    /// If it contains more then 1 character `None` will be returned.
    #[cfg(feature = "color")]
    pub fn ansi(text: impl AsRef<str>) -> Option<Self> {
        let text = text.as_ref();

        let mut chars = text.chars();
        let first_char = chars.next();
        let no_other_chars = chars.next().is_none();
        match first_char {
            Some(c) if no_other_chars => return Some(Self::new(c, None)),
            _ => (),
        }

        if string_width(text) != 1 {
            return None;
        }

        let (c, start, end) = get_ansi_secuences(text)?;
        if start.is_empty() && end.is_empty() {
            return Some(Self::new(c, None));
        }

        Some(Self::new(c, Some(Color::new(start, end))))
    }

    /// A function which create a [`Symbol`] from [`char`].
    pub const fn from_char(c: char) -> Self {
        Self::new(c, None)
    }

    /// A function which returns a used [`char`].
    pub const fn c(&self) -> char {
        self.c
    }

    /// A function which returns the used color.
    pub fn color(&self) -> Option<&Color> {
        self.color.as_ref()
    }
}

impl Default for Symbol {
    fn default() -> Self {
        Self::from_char(char::default())
    }
}

impl From<char> for Symbol {
    fn from(c: char) -> Self {
        Self::from_char(c)
    }
}

#[cfg(feature = "color")]
fn get_ansi_secuences(s: &str) -> Option<(char, String, String)> {
    let mut original = ansi_str::get_blocks(s);
    let block = original.next()?;

    let c = block.text().chars().next()?;

    let style = block.style();
    let start = style.start().to_string();
    let end = style.end().to_string();

    Some((c, start, end))
}
