mod macros;

pub fn is_width_eq(s: &str, width: usize) -> bool {
    get_string_width(s) == width
}

fn get_string_width(text: &str) -> usize {
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

#[allow(dead_code)]
fn get_line_width(text: &str) -> usize {
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
