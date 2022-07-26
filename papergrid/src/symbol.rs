use crate::util::string_width;
use crate::Color;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Symbol {
    c: char,
    color: Option<Color>,
}

impl Symbol {
    pub const fn new(c: char, color: Option<Color>) -> Self {
        Self { c, color }
    }

    /// Creates a new [`Symbol`] from the String.
    /// The string must contain 1 UTF-8 character and any list of Ansi sequences.
    ///
    /// If it contains more then 1 character `None` will be returned.
    pub fn ansi(s: impl AsRef<str>) -> Option<Self> {
        let s = s.as_ref();

        let mut chars = s.chars();
        let c = chars.next();
        let no_other_chars = chars.next().is_none();
        drop(chars);
        match c {
            Some(c) if no_other_chars => return Some(Self::new(c, None)),
            _ => (),
        }

        if string_width(s) != 1 {
            return None;
        }

        let (c, start, end) = get_ansi_secuences(s)?;
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

    /// A function which returns a used [`char`].
    pub fn color(self) -> Option<Color> {
        self.color
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

fn get_ansi_secuences(s: &str) -> Option<(char, String, String)> {
    let mut original = ansi_str::get_blocks(s);
    let block = original.next()?;

    let c = block.text().chars().next()?;

    let start = block.start().to_string();
    let end = block.end().to_string();

    Some((c, start, end))
}
