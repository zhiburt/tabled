/// Returns a string width.
pub fn get_line_width(text: &str) -> usize {
    #[cfg(not(feature = "ansi"))]
    {
        unicode_width::UnicodeWidthStr::width(text)
    }

    #[cfg(feature = "ansi")]
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
pub fn get_text_width(text: &str) -> usize {
    #[cfg(not(feature = "ansi"))]
    {
        text.lines()
            .map(unicode_width::UnicodeWidthStr::width)
            .max()
            .unwrap_or(0)
    }

    #[cfg(feature = "ansi")]
    {
        text.lines().map(get_line_width).max().unwrap_or(0)
    }
}

/// Returns a char width.
pub fn get_char_width(c: char) -> usize {
    unicode_width::UnicodeWidthChar::width(c).unwrap_or_default()
}

/// Returns a string width (accouting all characters).
pub fn get_string_width(text: &str) -> usize {
    unicode_width::UnicodeWidthStr::width(text)
}
