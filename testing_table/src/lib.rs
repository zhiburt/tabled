mod macros;

pub fn is_lines_equal(s: &str, width: usize) -> bool {
    string_width_multiline(s) == width
}

fn string_width_multiline(text: &str) -> usize {
    #[cfg(not(feature = "ansi"))]
    {
        text.lines()
            .map(unicode_width::UnicodeWidthStr::width)
            .max()
            .unwrap_or(0)
    }

    #[cfg(feature = "ansi")]
    {
        text.lines().map(string_width).max().unwrap_or(0)
    }
}

#[allow(dead_code)]
fn string_width(text: &str) -> usize {
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
