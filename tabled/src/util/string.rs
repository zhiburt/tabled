use std::borrow::Cow;

/// The function cuts the string to a specific width.
/// Preserving colors with `ansi` feature on.
pub(crate) fn split_str(s: &str, width: usize) -> (Cow<'_, str>, Cow<'_, str>) {
    #[cfg(feature = "ansi")]
    {
        const REPLACEMENT: char = '\u{FFFD}';

        let stripped = ansi_str::AnsiStr::ansi_strip(s);
        let (length, cutwidth, csize) = split_at_width(&stripped, width);
        let (mut lhs, mut rhs) = ansi_str::AnsiStr::ansi_split_at(s, length);

        if csize > 0 {
            let mut buf = lhs.into_owned();
            let count_unknowns = width - cutwidth;
            buf.extend(std::iter::repeat(REPLACEMENT).take(count_unknowns));
            lhs = Cow::Owned(buf);
            rhs = Cow::Owned(ansi_str::AnsiStr::ansi_cut(rhs.as_ref(), csize..).into_owned());
        }

        (lhs, rhs)
    }

    #[cfg(not(feature = "ansi"))]
    {
        const REPLACEMENT: char = '\u{FFFD}';

        let (length, cutwidth, csize) = split_at_width(s, width);
        let (lhs, rhs) = s.split_at(length);

        if csize == 0 {
            return (Cow::Borrowed(lhs), Cow::Borrowed(rhs));
        }

        let count_unknowns = width - cutwidth;
        let mut buf = lhs.to_owned();
        buf.extend(std::iter::repeat(REPLACEMENT).take(count_unknowns));

        (Cow::Owned(buf), Cow::Borrowed(&rhs[csize..]))
    }
}

/// The function cuts the string to a specific width.
/// Preserving colors with `ansi` feature on.
pub(crate) fn cut_str(s: &str, width: usize) -> Cow<'_, str> {
    #[cfg(feature = "ansi")]
    {
        const REPLACEMENT: char = '\u{FFFD}';

        let stripped = ansi_str::AnsiStr::ansi_strip(s);
        let (length, cutwidth, csize) = split_at_width(&stripped, width);
        let mut buf = ansi_str::AnsiStr::ansi_cut(s, ..length);
        if csize != 0 {
            let mut b = buf.into_owned();
            let count_unknowns = width - cutwidth;
            b.extend(std::iter::repeat(REPLACEMENT).take(count_unknowns));
            buf = Cow::Owned(b);
        }

        buf
    }

    #[cfg(not(feature = "ansi"))]
    {
        cut_str2(s, width)
    }
}

/// The function cuts the string to a specific width.
/// While not preserving ansi sequences.
pub(crate) fn cut_str2(text: &str, width: usize) -> Cow<'_, str> {
    const REPLACEMENT: char = '\u{FFFD}';

    let (length, cutwidth, csize) = split_at_width(text, width);
    if csize == 0 {
        let buf = &text[..length];
        return Cow::Borrowed(buf);
    }

    let buf = &text[..length];
    let mut buf = buf.to_owned();
    let count_unknowns = width - cutwidth;
    buf.extend(std::iter::repeat(REPLACEMENT).take(count_unknowns));

    Cow::Owned(buf)
}

/// The function splits a string in the position and
/// returns a exact number of bytes before the position and in case of a split in an unicode grapheme
/// a width of a character which was tried to be split in.
pub(crate) fn split_at_width(s: &str, at_width: usize) -> (usize, usize, usize) {
    let mut length = 0;
    let mut width = 0;
    for c in s.chars() {
        if width == at_width {
            break;
        };

        let c_width = unicode_width::UnicodeWidthChar::width(c).unwrap_or_default();
        let c_length = c.len_utf8();

        // We cut the chars which takes more then 1 symbol to display,
        // in order to archive the necessary width.
        if width + c_width > at_width {
            return (length, width, c_length);
        }

        width += c_width;
        length += c_length;
    }

    (length, width, 0)
}

/// Strip OSC codes from `s`. If `s` is a single OSC8 hyperlink, with no other text, then return
/// (s_with_all_hyperlinks_removed, Some(url)). If `s` does not meet this description, then return
/// (s_with_all_hyperlinks_removed, None). Any ANSI color sequences in `s` will be retained. See
/// <https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda>
///
/// The function is based on Dan Davison <https://github.com/dandavison> delta <https://github.com/dandavison/delta> ansi library.
#[cfg(feature = "ansi")]
pub(crate) fn strip_osc(text: &str) -> (String, Option<String>) {
    #[derive(Debug)]
    enum ExtractOsc8HyperlinkState {
        ExpectOsc8Url,
        ExpectFirstText,
        ExpectMoreTextOrTerminator,
        SeenOneHyperlink,
        WillNotReturnUrl,
    }

    use ExtractOsc8HyperlinkState::*;

    let mut url = None;
    let mut state = ExpectOsc8Url;
    let mut buf = String::with_capacity(text.len());

    for el in ansitok::parse_ansi(text) {
        match el.kind() {
            ansitok::ElementKind::Osc => match state {
                ExpectOsc8Url => {
                    url = Some(&text[el.start()..el.end()]);
                    state = ExpectFirstText;
                }
                ExpectMoreTextOrTerminator => state = SeenOneHyperlink,
                _ => state = WillNotReturnUrl,
            },
            ansitok::ElementKind::Sgr => buf.push_str(&text[el.start()..el.end()]),
            ansitok::ElementKind::Csi => buf.push_str(&text[el.start()..el.end()]),
            ansitok::ElementKind::Esc => {}
            ansitok::ElementKind::Text => {
                buf.push_str(&text[el.start()..el.end()]);
                match state {
                    ExpectFirstText => state = ExpectMoreTextOrTerminator,
                    ExpectMoreTextOrTerminator => {}
                    _ => state = WillNotReturnUrl,
                }
            }
        }
    }

    match state {
        WillNotReturnUrl => (buf, None),
        _ => {
            let url = url.and_then(|s| {
                s.strip_prefix("\x1b]8;;")
                    .and_then(|s| s.strip_suffix('\x1b'))
            });
            if let Some(url) = url {
                (buf, Some(url.to_string()))
            } else {
                (buf, None)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::grid::util::string::string_width;

    #[cfg(feature = "ansi")]
    use owo_colors::{colors::Yellow, OwoColorize};

    #[test]
    fn strip_test() {
        assert_eq!(cut_str("123456", 0), "");
        assert_eq!(cut_str("123456", 3), "123");
        assert_eq!(cut_str("123456", 10), "123456");

        assert_eq!(cut_str("a week ago", 4), "a we");

        assert_eq!(cut_str("😳😳😳😳😳", 0), "");
        assert_eq!(cut_str("😳😳😳😳😳", 3), "😳�");
        assert_eq!(cut_str("😳😳😳😳😳", 4), "😳😳");
        assert_eq!(cut_str("😳😳😳😳😳", 20), "😳😳😳😳😳");

        assert_eq!(cut_str("🏳️🏳️", 0), "");
        assert_eq!(cut_str("🏳️🏳️", 1), "🏳");
        assert_eq!(cut_str("🏳️🏳️", 2), "🏳\u{fe0f}🏳");
        assert_eq!(string_width("🏳️🏳️"), string_width("🏳\u{fe0f}🏳"));

        assert_eq!(cut_str("🎓", 1), "�");
        assert_eq!(cut_str("🎓", 2), "🎓");

        assert_eq!(cut_str("🥿", 1), "�");
        assert_eq!(cut_str("🥿", 2), "🥿");

        assert_eq!(cut_str("🩰", 1), "�");
        assert_eq!(cut_str("🩰", 2), "🩰");

        assert_eq!(cut_str("👍🏿", 1), "�");
        assert_eq!(cut_str("👍🏿", 2), "👍");
        assert_eq!(cut_str("👍🏿", 3), "👍�");
        assert_eq!(cut_str("👍🏿", 4), "👍🏿");

        assert_eq!(cut_str("🇻🇬", 1), "🇻");
        assert_eq!(cut_str("🇻🇬", 2), "🇻🇬");
        assert_eq!(cut_str("🇻🇬", 3), "🇻🇬");
        assert_eq!(cut_str("🇻🇬", 4), "🇻🇬");
    }

    #[cfg(feature = "ansi")]
    #[test]
    fn strip_color_test() {
        let numbers = "123456".red().on_bright_black().to_string();

        assert_eq!(cut_str(&numbers, 0), "\u{1b}[31;100m\u{1b}[39m\u{1b}[49m");
        assert_eq!(
            cut_str(&numbers, 3),
            "\u{1b}[31;100m123\u{1b}[39m\u{1b}[49m"
        );
        assert_eq!(cut_str(&numbers, 10), "\u{1b}[31;100m123456\u{1b}[0m");

        let emojies = "😳😳😳😳😳".red().on_bright_black().to_string();

        assert_eq!(cut_str(&emojies, 0), "\u{1b}[31;100m\u{1b}[39m\u{1b}[49m");
        assert_eq!(
            cut_str(&emojies, 3),
            "\u{1b}[31;100m😳\u{1b}[39m\u{1b}[49m�"
        );
        assert_eq!(
            cut_str(&emojies, 4),
            "\u{1b}[31;100m😳😳\u{1b}[39m\u{1b}[49m"
        );
        assert_eq!(cut_str(&emojies, 20), "\u{1b}[31;100m😳😳😳😳😳\u{1b}[0m");

        let emojies = "🏳️🏳️".red().on_bright_black().to_string();

        assert_eq!(cut_str(&emojies, 0), "\u{1b}[31;100m\u{1b}[39m\u{1b}[49m");
        assert_eq!(cut_str(&emojies, 1), "\u{1b}[31;100m🏳\u{1b}[39m\u{1b}[49m");
        assert_eq!(
            cut_str(&emojies, 2),
            "\u{1b}[31;100m🏳\u{fe0f}🏳\u{1b}[39m\u{1b}[49m"
        );
        assert_eq!(
            string_width(&emojies),
            string_width("\u{1b}[31;100m🏳\u{fe0f}🏳\u{1b}[39m\u{1b}[49m")
        );
    }

    #[test]
    #[cfg(feature = "ansi")]
    fn test_color_strip() {
        let s = "Collored string"
            .fg::<Yellow>()
            .on_truecolor(12, 200, 100)
            .blink()
            .to_string();
        assert_eq!(
            cut_str(&s, 1),
            "\u{1b}[5m\u{1b}[48;2;12;200;100m\u{1b}[33mC\u{1b}[25m\u{1b}[39m\u{1b}[49m"
        )
    }

    #[test]
    #[cfg(feature = "ansi")]
    fn test_srip_osc() {
        assert_eq!(
            strip_osc("just a string here"),
            (String::from("just a string here"), None)
        );
        assert_eq!(
            strip_osc("/etc/rc.conf"),
            (String::from("/etc/rc.conf"), None)
        );
        assert_eq!(
            strip_osc(
                "https://gitlab.com/finestructure/swiftpackageindex-builder/-/pipelines/1054655982"
            ),
            (String::from("https://gitlab.com/finestructure/swiftpackageindex-builder/-/pipelines/1054655982"), None)
        );

        assert_eq!(
            strip_osc(&build_link_prefix_suffix("just a string here")),
            (String::default(), Some(String::from("just a string here")))
        );
        assert_eq!(
            strip_osc(&build_link_prefix_suffix("/etc/rc.conf")),
            (String::default(), Some(String::from("/etc/rc.conf")))
        );
        assert_eq!(
            strip_osc(
                &build_link_prefix_suffix("https://gitlab.com/finestructure/swiftpackageindex-builder/-/pipelines/1054655982")
            ),
            (String::default(), Some(String::from("https://gitlab.com/finestructure/swiftpackageindex-builder/-/pipelines/1054655982")))
        );

        #[cfg(feature = "ansi")]
        fn build_link_prefix_suffix(url: &str) -> String {
            // https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda
            let osc8 = "\x1b]8;;";
            let st = "\x1b\\";
            format!("{osc8}{url}{st}")
        }
    }
}
