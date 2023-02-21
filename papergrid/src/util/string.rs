//! This module contains a different functions which are used by the [`Grid`].
//!
//! You should use it if you want to comply with how [`Grid`].
//!
//! [`Grid`]: crate::grid::spanned::Grid

/// Returns a string width with correction to tab width.
pub fn string_width_tab(text: &str, tab_width: usize) -> usize {
    #[cfg(not(feature = "color"))]
    {
        __string_width_tab(text, tab_width)
    }

    #[cfg(feature = "color")]
    {
        // we need to strip ansi because of terminal links
        // and they're can't be stripped by ansi_str.

        ansitok::parse_ansi(text)
            .filter(|e| e.kind() == ansitok::ElementKind::Text)
            .map(|e| &text[e.start()..e.end()])
            .map(|e| __string_width_tab(e, tab_width))
            .sum()
    }
}

fn __string_width_tab(text: &str, tab_width: usize) -> usize {
    text.chars().fold(0, |acc, c| {
        if c == '\t' {
            acc + tab_width
        } else {
            let w = unicode_width::UnicodeWidthChar::width(c).unwrap_or(0);
            acc + w
        }
    })
}

/// Returns a max per line string width with correction to tab width.
pub fn string_width_multiline_tab(text: &str, tab_width: usize) -> usize {
    #[cfg(not(feature = "color"))]
    {
        let (acc, max) = text.chars().fold((0, 0), |(acc, max), c| {
            if c == '\t' {
                (acc + tab_width, max)
            } else if c == '\n' {
                (0, std::cmp::max(acc, max))
            } else {
                let w = unicode_width::UnicodeWidthChar::width(c).unwrap_or(0);
                (acc + w, max)
            }
        });

        std::cmp::max(acc, max)
    }

    #[cfg(feature = "color")]
    {
        text.lines()
            .map(|line| string_width_tab(line, tab_width))
            .max()
            .unwrap_or(0)
    }
}

/// Returns a string width.
pub fn string_width(text: &str) -> usize {
    #[cfg(not(feature = "color"))]
    {
        unicode_width::UnicodeWidthStr::width(text)
    }

    #[cfg(feature = "color")]
    {
        // we need to strip ansi because of terminal links
        // and they're can't be stripped by ansi_str.

        ansitok::parse_ansi(text)
            .filter(|e| e.kind() == ansitok::ElementKind::Text)
            .map(|e| &text[e.start()..e.end()])
            .map(unicode_width::UnicodeWidthStr::width)
            .sum()
    }
}

/// Returns a max string width of a line.
pub fn string_width_multiline(text: &str) -> usize {
    #[cfg(not(feature = "color"))]
    {
        text.lines()
            .map(unicode_width::UnicodeWidthStr::width)
            .max()
            .unwrap_or(0)
    }

    #[cfg(feature = "color")]
    {
        text.lines().map(string_width).max().unwrap_or(0)
    }
}

/// Calculates a number of lines.
pub fn count_lines(s: &str) -> usize {
    if s.is_empty() {
        return 1;
    }

    bytecount::count(s.as_bytes(), b'\n') + 1
}

/// Returns a list of tabs (`\t`) in a string..
pub fn count_tabs(s: &str) -> usize {
    bytecount::count(s.as_bytes(), b'\t')
}

/// Splits the string by lines.
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub fn get_lines(text: &str) -> Lines<'_> {
    #[cfg(not(feature = "color"))]
    {
        // we call `split()` but not `lines()` in order to match colored implementation
        // specifically how we treat a trailing '\n' character.
        Lines {
            inner: text.split('\n'),
        }
    }

    #[cfg(feature = "color")]
    {
        Lines {
            inner: ansi_str::AnsiStr::ansi_split(text, "\n"),
        }
    }
}

/// Iterator over lines.
///
/// In comparison to `std::str::Lines`, it treats trailing '\n' as a new line.
#[allow(missing_debug_implementations)]
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub struct Lines<'a> {
    #[cfg(not(feature = "color"))]
    inner: std::str::Split<'a, char>,
    #[cfg(feature = "color")]
    inner: ansi_str::AnsiSplit<'a>,
}
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl<'a> Iterator for Lines<'a> {
    type Item = std::borrow::Cow<'a, str>;

    fn next(&mut self) -> Option<Self::Item> {
        #[cfg(not(feature = "color"))]
        {
            self.inner.next().map(std::borrow::Cow::Borrowed)
        }

        #[cfg(feature = "color")]
        {
            self.inner.next()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_width_emojie_test() {
        // ...emojis such as “joy”, which normally take up two columns when printed in a terminal
        // https://github.com/mgeisler/textwrap/pull/276
        assert_eq!(string_width("🎩"), 2);
        assert_eq!(string_width("Rust 💕"), 7);
        assert_eq!(string_width_multiline("Go 👍\nC 😎"), 5);
    }

    #[cfg(feature = "color")]
    #[test]
    fn colored_string_width_test() {
        use owo_colors::OwoColorize;
        assert_eq!(string_width(&"hello world".red().to_string()), 11);
        assert_eq!(
            string_width_multiline(&"hello\nworld".blue().to_string()),
            5
        );
        assert_eq!(string_width("\u{1b}[34m0\u{1b}[0m"), 1);
        assert_eq!(string_width(&"0".red().to_string()), 1);
    }

    #[test]
    fn count_lines_test() {
        assert_eq!(
            count_lines("\u{1b}[37mnow is the time for all good men\n\u{1b}[0m"),
            2
        );
        assert_eq!(count_lines("now is the time for all good men\n"), 2);
    }

    #[cfg(feature = "color")]
    #[test]
    fn string_width_multinline_for_link() {
        assert_eq!(
            string_width_multiline(
                "\u{1b}]8;;file:///home/nushell/asd.zip\u{1b}\\asd.zip\u{1b}]8;;\u{1b}\\"
            ),
            7
        );
    }

    #[cfg(feature = "color")]
    #[test]
    fn string_width_for_link() {
        assert_eq!(
            string_width("\u{1b}]8;;file:///home/nushell/asd.zip\u{1b}\\asd.zip\u{1b}]8;;\u{1b}\\"),
            7
        );
    }
}
