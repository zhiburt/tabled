use std::borrow::Cow;

use crate::grid::util::string::get_char_width;

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
            buf.extend(std::iter::repeat_n(REPLACEMENT, count_unknowns));
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
        buf.extend(std::iter::repeat_n(REPLACEMENT, count_unknowns));

        (Cow::Owned(buf), Cow::Borrowed(&rhs[csize..]))
    }
}

/// The function cuts the string to a specific width.
/// Preserving colors with `ansi` feature on.
pub(crate) fn cut_str(s: &str, width: usize) -> Cow<'_, str> {
    #[cfg(feature = "ansi")]
    {
        cut_str_colored(s, width)
    }

    #[cfg(not(feature = "ansi"))]
    {
        cut_str_basic(s, width)
    }
}

#[cfg(not(feature = "ansi"))]
fn cut_str_basic(text: &str, width: usize) -> Cow<'_, str> {
    const REPLACEMENT: char = '\u{FFFD}';

    let (length, cutwidth, csize) = split_at_width(text, width);
    if csize == 0 {
        let buf = &text[..length];
        return Cow::Borrowed(buf);
    }

    let buf = &text[..length];
    let mut buf = buf.to_owned();
    let count_unknowns = width - cutwidth;
    buf.extend(std::iter::repeat_n(REPLACEMENT, count_unknowns));

    Cow::Owned(buf)
}

// TODO: Cow::Borrow if no ansi is used.
#[cfg(feature = "ansi")]
fn cut_str_colored(text: &str, width: usize) -> Cow<'_, str> {
    use crate::util::string::{build_link, strip_osc};

    const REPLACEMENT: char = '\u{FFFD}';

    let (text, url) = strip_osc(text);
    let (prefix, suffix) = build_link(url);

    let stripped = ansi_str::AnsiStr::ansi_strip(&text);
    let (length, cutwidth, csize) = split_at_width(&stripped, width);
    let mut buf = ansi_str::AnsiStr::ansi_cut(&text, ..length);

    if csize != 0 {
        let mut b = buf.into_owned();
        let count_unknowns = width - cutwidth;
        b.extend(std::iter::repeat_n(REPLACEMENT, count_unknowns));
        buf = Cow::Owned(b);
    }

    if !prefix.is_empty() && !suffix.is_empty() {
        let mut b = String::with_capacity(buf.len() + prefix.len() + suffix.len());
        for (i, part) in ansi_str::AnsiStr::ansi_split(buf.as_ref(), "\n").enumerate() {
            if i > 0 {
                b.push('\n');
            }

            b.push_str(&prefix);
            b.push_str(&part);
            b.push_str(&suffix);
        }

        buf = Cow::Owned(b);
    }

    Cow::Owned(buf.into_owned())
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

        if c == '\n' {
            width = 0;
            length += 1;
            continue;
        }

        let c_width = std::cmp::max(1, get_char_width(c));
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

#[cfg(feature = "ansi")]
pub(crate) fn build_link(url: Option<String>) -> (String, String) {
    let url = match url {
        Some(url) => url,
        None => return (String::new(), String::new()),
    };

    // https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda
    let osc8 = "\x1b]8;;";
    let st = "\x1b\\";

    let prefix = format!("{osc8}{url}{st}");
    let suffix = format!("{osc8}{st}");

    (prefix, suffix)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::grid::util::string::get_line_width;

    #[test]
    fn strip_test() {
        assert_eq!(cut_str("123456", 0), "");
        assert_eq!(cut_str("123456", 3), "123");
        assert_eq!(cut_str("123456", 10), "123456");

        assert_eq!(cut_str("a week ago", 4), "a we");

        assert_eq!(cut_str("123\n456789", 0), "");
        assert_eq!(cut_str("123\n456789", 2), "12");
        assert_eq!(cut_str("123\n456789", 3), "123");
        assert_eq!(cut_str("123\n456789", 4), "123\n4567");
        assert_eq!(cut_str("123\n45\n67\n89", 4), "123\n45\n67\n89");
        assert_eq!(cut_str("123\n45\n67\n89\n", 4), "123\n45\n67\n89\n");

        assert_eq!(cut_str("😳😳😳😳😳", 0), "");
        assert_eq!(cut_str("😳😳😳😳😳", 3), "😳�");
        assert_eq!(cut_str("😳😳😳😳😳", 4), "😳😳");
        assert_eq!(cut_str("😳😳😳😳😳", 20), "😳😳😳😳😳");

        assert_eq!(cut_str("🏳️🏳️", 0), "");
        assert_eq!(cut_str("🏳️🏳️", 1), "🏳");
        assert_eq!(cut_str("🏳️🏳️", 2), "🏳\u{fe0f}");
        assert_eq!(get_line_width("🏳️🏳️"), get_line_width("🏳\u{fe0f}🏳\u{fe0f}"));

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
        let numbers = "\u{1b}[31;100m123456\u{1b}[39m\u{1b}[49m";

        assert_eq!(cut_str(numbers, 0), "\u{1b}[31;100m\u{1b}[39m\u{1b}[49m");
        assert_eq!(cut_str(numbers, 3), "\u{1b}[31;100m123\u{1b}[39m\u{1b}[49m");
        assert_eq!(
            cut_str(numbers, 10),
            "\u{1b}[31;100m123456\u{1b}[39m\u{1b}[49m"
        );

        let emojies = "\u{1b}[31;100m😳😳😳😳😳\u{1b}[39m\u{1b}[49m";

        assert_eq!(cut_str(emojies, 0), "\u{1b}[31;100m\u{1b}[39m\u{1b}[49m");
        assert_eq!(cut_str(emojies, 3), "\u{1b}[31;100m😳\u{1b}[39m\u{1b}[49m�");
        assert_eq!(
            cut_str(emojies, 4),
            "\u{1b}[31;100m😳😳\u{1b}[39m\u{1b}[49m"
        );
        assert_eq!(
            cut_str(emojies, 20),
            "\u{1b}[31;100m😳😳😳😳😳\u{1b}[39m\u{1b}[49m"
        );

        let emojies = "\u{1b}[31;100m🏳️🏳️\u{1b}[39m\u{1b}[49m";

        assert_eq!(cut_str(emojies, 0), "\u{1b}[31;100m\u{1b}[39m\u{1b}[49m");
        assert_eq!(cut_str(emojies, 1), "\u{1b}[31;100m🏳\u{1b}[39m\u{1b}[49m");
        assert_eq!(
            cut_str(emojies, 2),
            "\u{1b}[31;100m🏳\u{fe0f}\u{1b}[39m\u{1b}[49m"
        );
        assert_eq!(
            get_line_width(emojies),
            get_line_width("\u{1b}[31;100m🏳\u{fe0f}🏳\u{fe0f}\u{1b}[39m\u{1b}[49m")
        );
    }

    #[test]
    #[cfg(feature = "ansi")]
    fn test_color_strip() {
        let s = "\u{1b}[5;33;48;2;12;200;100mCollored string\u{1b}[0m";
        assert_eq!(
            cut_str(s, 1),
            "\u{1b}[5;33;48;2;12;200;100mC\u{1b}[25m\u{1b}[39m\u{1b}[49m"
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
