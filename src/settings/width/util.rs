use std::borrow::Cow;

use papergrid::{grid_projection::GridProjection, records::Records, ExactDimension, GridConfig};

pub fn get_table_widths<R: Records>(records: R, cfg: &GridConfig) -> Vec<usize> {
    ExactDimension::width(records, cfg)
}

pub fn get_table_widths_with_total<R: Records>(
    records: R,
    cfg: &GridConfig,
) -> (Vec<usize>, usize) {
    let widths = ExactDimension::width(records, cfg);
    let total_width = get_table_total_width(&widths, cfg);
    (widths, total_width)
}

fn get_table_total_width(list: &[usize], cfg: &GridConfig) -> usize {
    let margin = cfg.get_margin();
    let gp = GridProjection::new(cfg).count_columns(list.len());

    list.iter().sum::<usize>() + gp.count_vertical() + margin.left.size + margin.right.size
}

/// Replaces tabs in a string with a given width of spaces.
pub fn replace_tab(text: &str, n: usize) -> Cow<'_, str> {
    if !text.contains('\t') {
        return Cow::Borrowed(text);
    }

    // it's a general case which probably must be faster?
    let replaced = if n == 4 {
        text.replace('\t', "    ")
    } else {
        let mut text = text.to_owned();
        replace_tab_range(&mut text, n);
        text
    };

    Cow::Owned(replaced)
}

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

/// The function cuts the string to a specific width.
///
/// BE AWARE: width is expected to be in bytes.
pub fn cut_str(s: &str, width: usize) -> Cow<'_, str> {
    #[cfg(feature = "color")]
    {
        const REPLACEMENT: char = '\u{FFFD}';

        let stripped = ansi_str::AnsiStr::ansi_strip(s);
        let (length, count_unknowns, _) = split_at_pos(&stripped, width);

        let mut buf = ansi_str::AnsiStr::ansi_cut(s, ..length);
        if count_unknowns > 0 {
            let mut b = buf.into_owned();
            b.extend(std::iter::repeat(REPLACEMENT).take(count_unknowns));
            buf = Cow::Owned(b);
        }

        buf
    }
    #[cfg(not(feature = "color"))]
    {
        cut_str_basic(s, width)
    }
}

/// The function cuts the string to a specific width.
///
/// BE AWARE: width is expected to be in bytes.
pub fn cut_str_basic(s: &str, width: usize) -> Cow<'_, str> {
    const REPLACEMENT: char = '\u{FFFD}';

    let (length, count_unknowns, _) = split_at_pos(s, width);
    let buf = &s[..length];
    if count_unknowns == 0 {
        return Cow::Borrowed(buf);
    }

    let mut buf = buf.to_owned();
    buf.extend(std::iter::repeat(REPLACEMENT).take(count_unknowns));

    Cow::Owned(buf)
}

/// The function splits a string in the position and
/// returns a exact number of bytes before the position and in case of a split in an unicode grapheme
/// a width of a character which was tried to be splited in.
///
/// BE AWARE: pos is expected to be in bytes.
pub fn split_at_pos(s: &str, pos: usize) -> (usize, usize, usize) {
    let mut length = 0;
    let mut i = 0;
    for c in s.chars() {
        if i == pos {
            break;
        };

        let c_width = unicode_width::UnicodeWidthChar::width(c).unwrap_or(0);

        // We cut the chars which takes more then 1 symbol to display,
        // in order to archive the necessary width.
        if i + c_width > pos {
            let count = pos - i;
            return (length, count, c.len_utf8());
        }

        i += c_width;
        length += c.len_utf8();
    }

    (length, 0, 0)
}

/// Strip OSC codes from `s`. If `s` is a single OSC8 hyperlink, with no other text, then return
/// (s_with_all_hyperlinks_removed, Some(url)). If `s` does not meet this description, then return
/// (s_with_all_hyperlinks_removed, None). Any ANSI color sequences in `s` will be retained. See
/// <https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda>
///
/// The function is based on Dan Davison <https://github.com/dandavison> delta <https://github.com/dandavison/delta> ansi library.
#[cfg(feature = "color")]
pub fn strip_osc(text: &str) -> (String, Option<String>) {
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

    use papergrid::util::string::string_width;

    #[cfg(feature = "color")]
    use owo_colors::{colors::Yellow, OwoColorize};

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

    #[cfg(feature = "color")]
    #[test]
    fn strip_color_test() {
        use owo_colors::OwoColorize;

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
    #[cfg(feature = "color")]
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
}
