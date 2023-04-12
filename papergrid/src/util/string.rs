//! This module contains a different functions which are used by the [`Grid`].
//!
//! You should use it if you want to comply with how [`Grid`].
//!
//! [`Grid`]: crate::grid::iterable::Grid

/// Returns string width and count lines of a string. It's a combination of [`string_width_multiline`] and [`count_lines`].
#[cfg(feature = "std")]
pub fn string_dimension(text: &str) -> (usize, usize) {
    #[cfg(not(feature = "color"))]
    {
        let (lines, acc, max) = text.chars().fold((1, 0, 0), |(lines, acc, max), c| {
            if c == '\n' {
                (lines + 1, 0, acc.max(max))
            } else {
                let w = unicode_width::UnicodeWidthChar::width(c).unwrap_or(0);
                (lines, acc + w, max)
            }
        });

        (lines, acc.max(max))
    }

    #[cfg(feature = "color")]
    {
        get_lines(text)
            .map(|line| string_width(&line))
            .fold((0, 0), |(i, acc), width| (i + 1, acc.max(width)))
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
pub struct Lines<'a> {
    #[cfg(not(feature = "color"))]
    inner: std::str::Split<'a, char>,
    #[cfg(feature = "color")]
    inner: ansi_str::AnsiSplit<'a>,
}
#[cfg(feature = "std")]
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

#[cfg(feature = "std")]
/// Replaces tabs in a string with a given width of spaces.
pub fn replace_tab(text: &str, n: usize) -> std::borrow::Cow<'_, str> {
    if !text.contains('\t') {
        return std::borrow::Cow::Borrowed(text);
    }

    // it's a general case which probably must be faster?
    let replaced = if n == 4 {
        text.replace('\t', "    ")
    } else {
        let mut text = text.to_owned();
        replace_tab_range(&mut text, n);
        text
    };

    std::borrow::Cow::Owned(replaced)
}

#[cfg(feature = "std")]
fn replace_tab_range(cell: &mut String, n: usize) -> &str {
    let mut skip = 0;
    while let &Some(pos) = &cell[skip..].find('\t') {
        let pos = skip + pos;

        let is_escaped = pos > 0 && cell.get(pos - 1..pos) == Some("\\");
        if is_escaped {
            skip = pos + 1;
        } else if n == 0 {
            cell.remove(pos);
            skip = pos;
        } else {
            // I'am not sure which version is faster a loop of 'replace'
            // or allacation of a string for replacement;
            cell.replace_range(pos..=pos, &" ".repeat(n));
            skip = pos + 1;
        }

        if cell.is_empty() || skip >= cell.len() {
            break;
        }
    }

    cell
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

    #[cfg(feature = "std")]
    #[test]
    fn string_dimension_test() {
        assert_eq!(
            string_dimension("\u{1b}[37mnow is the time for all good men\n\u{1b}[0m"),
            {
                #[cfg(feature = "color")]
                {
                    (2, 32)
                }
                #[cfg(not(feature = "color"))]
                {
                    (2, 36)
                }
            }
        );
        assert_eq!(
            string_dimension("now is the time for all good men\n"),
            (2, 32)
        );
        assert_eq!(string_dimension("asd"), (1, 3));
        assert_eq!(string_dimension(""), (1, 0));
    }

    #[cfg(feature = "std")]
    #[test]
    fn replace_tab_test() {
        assert_eq!(replace_tab("123\t\tabc\t", 3), "123      abc   ");

        assert_eq!(replace_tab("\t", 0), "");
        assert_eq!(replace_tab("\t", 3), "   ");
        assert_eq!(replace_tab("123\tabc", 3), "123   abc");
        assert_eq!(replace_tab("123\tabc\tzxc", 0), "123abczxc");

        assert_eq!(replace_tab("\\t", 0), "\\t");
        assert_eq!(replace_tab("\\t", 4), "\\t");
        assert_eq!(replace_tab("123\\tabc", 0), "123\\tabc");
        assert_eq!(replace_tab("123\\tabc", 4), "123\\tabc");
    }
}
