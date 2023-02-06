//! A module which contains a [`WidthFunc`] trait and it's implementation [`CfgWidthFunc`]

use util::{string_width, string_width_multiline, string_width_multiline_tab, string_width_tab};

/// A width function.
pub trait WidthFunc {
    /// Calculates a width of a string.
    fn width(&self, text: &str) -> usize;
    /// Calculates a width of a multiline string.
    fn width_multiline(&self, text: &str) -> usize;
}

impl<W> WidthFunc for &W
where
    W: WidthFunc,
{
    fn width(&self, text: &str) -> usize {
        W::width(self, text)
    }

    fn width_multiline(&self, text: &str) -> usize {
        W::width_multiline(self, text)
    }
}

/// A [`WidthFunc`] implementation which is used by [`Grid`].
///
/// [`Grid`]: crate::Grid
#[derive(Debug, Default, Clone)]
pub struct TabWidthFunc {
    tab_width: usize,
}

impl TabWidthFunc {
    /// Creates a [`CfgWidthFunc`] with a tab size.
    pub fn new(tab_size: usize) -> Self {
        Self {
            tab_width: tab_size,
        }
    }
}

impl WidthFunc for TabWidthFunc {
    fn width(&self, text: &str) -> usize {
        string_width_tab(text, self.tab_width)
    }

    fn width_multiline(&self, text: &str) -> usize {
        string_width_multiline_tab(text, self.tab_width)
    }
}

/// A [`WidthFunc`] implementation which is used by [`Grid`].
///
/// [`Grid`]: crate::Grid
#[derive(Debug, Default, Clone)]
pub struct BasicWidthFunc;

impl BasicWidthFunc {
    pub fn new() -> Self {
        BasicWidthFunc
    }
}

impl WidthFunc for BasicWidthFunc {
    fn width(&self, text: &str) -> usize {
        string_width(text)
    }

    fn width_multiline(&self, text: &str) -> usize {
        string_width_multiline(text)
    }
}

mod util {
    /// Returns a string width.
    #[cfg(not(feature = "color"))]
    pub fn string_width(text: &str) -> usize {
        unicode_width::UnicodeWidthStr::width(text)
    }

    /// Returns a string width.
    #[cfg(feature = "color")]
    pub fn string_width(text: &str) -> usize {
        // we need to strip ansi because of terminal links
        // and they're can't be stripped by ansi_str.

        ansitok::parse_ansi(text)
            .filter(|e| e.kind() == ansitok::ElementKind::Text)
            .map(|e| &text[e.start()..e.end()])
            .map(unicode_width::UnicodeWidthStr::width)
            .sum()
    }

    /// Returns a max string width of a line.
    #[cfg(not(feature = "color"))]
    pub fn string_width_multiline(text: &str) -> usize {
        text.lines()
            .map(unicode_width::UnicodeWidthStr::width)
            .max()
            .unwrap_or(0)
    }

    /// Returns a max string width of a line.
    #[cfg(feature = "color")]
    pub fn string_width_multiline(text: &str) -> usize {
        text.lines().map(string_width).max().unwrap_or(0)
    }

    /// Returns a string width with correction to tab width.
    pub fn string_width_tab(text: &str, tab_width: usize) -> usize {
        let width = string_width(text);
        let count_tabs = count_tabs(text);

        width + count_tabs * tab_width
    }

    /// Returns a max per line string width with correction to tab width.
    pub fn string_width_multiline_tab(text: &str, tab_width: usize) -> usize {
        text.lines()
            .map(|line| string_width_tab(line, tab_width))
            .max()
            .unwrap_or(0)
    }

    /// Returns a list of tabs (`\t`) in a string..
    pub fn count_tabs(s: &str) -> usize {
        bytecount::count(s.as_bytes(), b'\t')
    }
}
