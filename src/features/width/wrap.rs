//! This module contains [`Wrap`] structure, used to decrease width of a [`Table`]s or a cell on a [`Table`] by wrapping it's content
//! to a new line.

use std::marker::PhantomData;

use papergrid::{
    records::{empty::EmptyRecords, Records, RecordsMut},
    util::string_width_multiline,
    width::CfgWidthFunction,
    Entity,
};

use crate::{
    measurment::Measurment,
    peaker::{Peaker, PriorityNone},
    CellOption, Table, TableOption, Width,
};

use super::{
    get_table_widths, get_table_widths_with_total,
    truncate::{decrease_widths, get_decrease_cell_list},
};

/// Wrap wraps a string to a new line in case it exceeds the provided max boundary.
/// Otherwise keeps the content of a cell untouched.
///
/// The function is color aware if a `color` feature is on.
///
/// Be aware that it doesn't consider padding.
/// So if you want to set a exact width you might need to use [`Padding`] to set it to 0.
///
/// ## Example
///
/// ```
/// use tabled::{object::Segment, Width, Modify, Table};
///
/// let table = Table::new(&["Hello World!"])
///     .with(Modify::new(Segment::all()).with(Width::wrap(3)));
/// ```
///
/// [`Padding`]: crate::Padding
#[derive(Debug, Clone)]
pub struct Wrap<W = usize, P = PriorityNone> {
    width: W,
    keep_words: bool,
    _priority: PhantomData<P>,
}

impl<W> Wrap<W>
where
    W: Measurment<Width>,
{
    /// Creates a [`Wrap`] object
    pub fn new(width: W) -> Self {
        Self {
            width,
            keep_words: false,
            _priority: PhantomData::default(),
        }
    }
}

impl<W, P> Wrap<W, P> {
    /// Priority defines the logic by which a truncate will be applied when is done for the whole table.
    ///
    /// - [`PriorityNone`] which cuts the columns one after another.
    /// - [`PriorityMax`] cuts the biggest columns first.
    /// - [`PriorityMin`] cuts the lowest columns first.
    ///
    /// Be aware that it doesn't consider padding.
    /// So if you want to set a exact width you might need to use [`Padding`] to set it to 0.
    ///
    /// [`Padding`]: crate::Padding
    /// [`PriorityMax`]: crate::peaker::PriorityMax
    /// [`PriorityMin`]: crate::peaker::PriorityMin
    pub fn priority<PP>(self) -> Wrap<W, PP> {
        Wrap {
            width: self.width,
            keep_words: self.keep_words,
            _priority: PhantomData::default(),
        }
    }

    /// Set the keep words option.
    ///
    /// If a wrapping point will be in a word, [`Wrap`] will
    /// preserve a word (if possible) and wrap the string before it.
    pub fn keep_words(mut self) -> Self {
        self.keep_words = true;
        self
    }
}

impl<W, P, R> CellOption<R> for Wrap<W, P>
where
    W: Measurment<Width>,
    R: Records + RecordsMut<String>,
{
    fn change_cell(&mut self, table: &mut Table<R>, entity: Entity) {
        let width_ctrl = CfgWidthFunction::from_cfg(table.get_config());
        let width = self.width.measure(table.get_records(), table.get_config());

        let (count_rows, count_cols) = table.shape();
        for pos in entity.iter(count_rows, count_cols) {
            let records = table.get_records();
            let cell_width = records.get_width(pos, &width_ctrl);
            if cell_width <= width {
                continue;
            }

            let text = records.get_text(pos);
            // todo: Think about it.
            //       We could eliminate this allcation if we would be allowed to cut '\t' with unknown characters.
            //       Currently we don't do that.
            let text = papergrid::util::replace_tab(text, table.get_config().get_tab_width());
            let wrapped = wrap_text(&text, width, self.keep_words);

            debug_assert!(
                width >= string_width_multiline(&wrapped),
                "width={:?}\n\n content={:?}\n\n wrap={:?}\n",
                width,
                text,
                wrapped
            );

            let records = table.get_records_mut();
            records.set(pos, wrapped, &width_ctrl);
        }

        table.destroy_width_cache();
    }
}

impl<W, P, R> TableOption<R> for Wrap<W, P>
where
    W: Measurment<Width>,
    P: Peaker,
    R: Records + RecordsMut<String>,
{
    fn change(&mut self, table: &mut Table<R>) {
        if table.is_empty() {
            return;
        }

        let width = self.width.measure(table.get_records(), table.get_config());
        let (widths, total_width) =
            get_table_widths_with_total(table.get_records(), table.get_config());
        if width >= total_width {
            return;
        }

        let priority = P::create();
        let keep_words = self.keep_words;
        wrap_total_width(table, widths, total_width, width, keep_words, priority);
    }
}

fn wrap_total_width<R, P>(
    table: &mut Table<R>,
    mut widths: Vec<usize>,
    total_width: usize,
    width: usize,
    keep_words: bool,
    priority: P,
) where
    P: Peaker,
    R: Records + RecordsMut<String>,
{
    let (count_rows, count_cols) = table.shape();
    let cfg = table.get_config();
    let min_widths = get_table_widths(EmptyRecords::new(count_rows, count_cols), cfg);

    decrease_widths(&mut widths, &min_widths, total_width, width, priority);

    let points = get_decrease_cell_list(cfg, &widths, &min_widths, (count_rows, count_cols));

    let mut wrap = Wrap::new(0);
    wrap.keep_words = keep_words;
    for ((row, col), width) in points {
        wrap.width = width;
        wrap.change_cell(table, (row, col).into());
    }

    table.destroy_height_cache();
    table.destroy_width_cache();
    table.cache_width(widths);
}

#[cfg(not(feature = "color"))]
pub(crate) fn wrap_text(text: &str, width: usize, keep_words: bool) -> String {
    if width == 0 {
        return String::new();
    }

    if keep_words {
        split_keeping_words(text, width, "\n")
    } else {
        chunks(text, width).join("\n")
    }
}

#[cfg(feature = "color")]
pub(crate) fn wrap_text(text: &str, width: usize, keep_words: bool) -> String {
    if width == 0 {
        return String::new();
    }

    let (text, url): (String, Option<String>) = link_extraction::strip_osc(text);
    let (prefix, suffix) = build_link_prefix_suffix(url);

    if keep_words {
        split_keeping_words(&text, width, &prefix, &suffix)
    } else {
        chunks(&text, width, &prefix, &suffix).join("\n")
    }
}

#[cfg(feature = "color")]
fn build_link_prefix_suffix(url: Option<String>) -> (String, String) {
    let (prefix, suffix) = if let Some(url) = url {
        // https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda
        let osc8 = "\x1b]8;;";
        let st = "\x1b\\";
        (format!("{osc8}{url}{st}"), format!("{osc8}{st}"))
    } else {
        ("".to_string(), "".to_string())
    };
    (prefix, suffix)
}

#[cfg(not(feature = "color"))]
fn chunks(s: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return Vec::new();
    }

    const REPLACEMENT: char = '\u{FFFD}';

    let mut buf = String::with_capacity(width);
    let mut list = Vec::new();
    let mut i = 0;
    for c in s.chars() {
        let c_width = unicode_width::UnicodeWidthChar::width(c).unwrap_or(0);
        if i + c_width > width {
            let count_unknowns = width - i;
            buf.extend(std::iter::repeat(REPLACEMENT).take(count_unknowns));
            i += count_unknowns;
        } else {
            buf.push(c);
            i += c_width;
        }

        if i == width {
            list.push(buf);
            buf = String::with_capacity(width);
            i = 0;
        }
    }

    if !buf.is_empty() {
        list.push(buf);
    }

    list
}

#[cfg(feature = "color")]
fn chunks(s: &str, width: usize, prefix: &str, suffix: &str) -> Vec<String> {
    use std::fmt::Write;

    if width == 0 {
        return Vec::new();
    }

    let mut list = Vec::new();
    let mut line = String::with_capacity(width);
    let mut line_width = 0;

    for b in ansi_str::get_blocks(s) {
        if b.text().is_empty() {
            continue;
        }

        line.push_str(prefix);
        let _ = write!(&mut line, "{}", b.start());

        let mut part = b.text();

        while !part.is_empty() {
            let available_space = width - line_width;

            let part_width = unicode_width::UnicodeWidthStr::width(part);
            if part_width <= available_space {
                line.push_str(part);
                line_width += part_width;

                if available_space == 0 {
                    let _ = write!(&mut line, "{}", b.end());
                    line.push_str(suffix);
                    list.push(line);
                    line = String::with_capacity(width);
                    line.push_str(prefix);
                    line_width = 0;
                    let _ = write!(&mut line, "{}", b.start());
                }

                break;
            }

            let (lhs, rhs, (unknowns, split_char)) = split_string_at(part, available_space);

            part = &rhs[split_char..];

            line.push_str(lhs);
            line_width += unicode_width::UnicodeWidthStr::width(lhs);

            const REPLACEMENT: char = '\u{FFFD}';
            line.extend(std::iter::repeat(REPLACEMENT).take(unknowns));
            line_width += unknowns;

            if line_width == width {
                let _ = write!(&mut line, "{}", b.end());
                line.push_str(suffix);
                list.push(line);
                line = String::with_capacity(width);
                line.push_str(prefix);
                line_width = 0;
                let _ = write!(&mut line, "{}", b.start());
            }
        }

        if line_width > 0 {
            let _ = write!(&mut line, "{}", b.end());
        }
    }

    if line_width > 0 {
        line.push_str(suffix);
        list.push(line);
    }

    list
}

#[cfg(not(feature = "color"))]
fn split_keeping_words(s: &str, width: usize, sep: &str) -> String {
    const REPLACEMENT: char = '\u{FFFD}';

    let mut lines = Vec::new();
    let mut line = String::with_capacity(width);
    let mut line_width = 0;

    let mut is_first_word = true;

    for word in s.split(' ') {
        if !is_first_word {
            let line_has_space = line_width < width;
            if line_has_space {
                line.push(' ');
                line_width += 1;
                is_first_word = false;
            }
        }

        if is_first_word {
            is_first_word = false;
        }

        let word_width = unicode_width::UnicodeWidthStr::width(word);

        let line_has_space = line_width + word_width <= width;
        if line_has_space {
            line.push_str(word);
            line_width += word_width;
            continue;
        }

        if word_width <= width {
            // the word can be fit to 'width' so we put it on new line

            line.extend(std::iter::repeat(' ').take(width - line_width));
            lines.push(line);

            line = String::with_capacity(width);
            line_width = 0;

            line.push_str(word);
            line_width += word_width;
            is_first_word = false;
        } else {
            // the word is too long any way so we split it

            let mut word_part = word;
            while !word_part.is_empty() {
                let available_space = width - line_width;
                let (lhs, rhs, (unknowns, split_char)) =
                    split_string_at(word_part, available_space);

                word_part = &rhs[split_char..];
                line_width += unicode_width::UnicodeWidthStr::width(lhs) + unknowns;

                line.push_str(lhs);
                line.extend(std::iter::repeat(REPLACEMENT).take(unknowns));

                if line_width == width {
                    lines.push(line);
                    line = String::with_capacity(width);
                    line_width = 0;
                    is_first_word = true;
                }
            }
        }
    }

    if line_width > 0 {
        line.extend(std::iter::repeat(' ').take(width - line_width));
        lines.push(line);
    }

    lines.join(sep)
}

#[cfg(feature = "color")]
fn split_keeping_words(text: &str, width: usize, prefix: &str, suffix: &str) -> String {
    use std::fmt::Write;

    use ansi_str::AnsiBlock;

    if text.is_empty() || width == 0 {
        return String::new();
    }

    let mut buf = String::new();
    let mut line_width = 0;
    let mut word_begin_pos = 0;
    let mut word_length = 0;
    let mut is_empty_buf = true;

    let split = |buf: &mut String, block: &AnsiBlock<'_>| {
        let _ = write!(buf, "{}", block.end());
        buf.push_str(suffix);
        buf.push('\n');
        buf.push_str(prefix);
        let _ = write!(buf, "{}", block.start());
    };

    // go char by char and split string afterwords

    buf.push_str(prefix);

    for block in ansi_str::get_blocks(text) {
        if block.text().is_empty() {
            continue;
        }

        let _ = write!(buf, "{}", block.start());

        for c in block.text().chars() {
            let c_width = unicode_width::UnicodeWidthChar::width(c).unwrap_or(0);
            let is_enough_space = line_width + c_width <= width;

            let is_space = c == ' ';
            if is_space {
                word_length = 0;
                word_begin_pos = 0;

                if !is_enough_space {
                    split(&mut buf, &block);
                    line_width = 0;
                }

                buf.push(c);
                line_width += 1;

                if is_empty_buf {
                    is_empty_buf = false;
                }
                continue;
            }

            let is_first_c = word_length == 0;
            if is_first_c {
                word_begin_pos = buf.len();
            }

            if is_enough_space {
                buf.push(c);
                word_length += c_width;
                line_width += c_width;

                if is_empty_buf {
                    is_empty_buf = false;
                }
            } else {
                // we can't say if the word is really fits in at this time because we may not have the whole word,
                // but it's good enough.
                let partial_word_width = word_length + c_width;
                let is_word_small = partial_word_width <= width;
                if is_word_small {
                    // move it to other line

                    if !is_empty_buf {
                        // we don't fill the rest of the prev line here

                        let sep = format!("{}{}\n{}{}", block.end(), suffix, prefix, block.start());
                        buf.insert_str(word_begin_pos, &sep);
                    }

                    buf.push(c);
                    line_width = partial_word_width;
                    word_length += c_width;

                    if is_empty_buf {
                        is_empty_buf = false;
                    }
                } else {
                    // it's not small so we can't do anything about it.

                    if !is_empty_buf {
                        split(&mut buf, &block);
                    }

                    let is_big_char = c_width > width;
                    if is_big_char {
                        const REPLACEMENT: char = '\u{FFFD}';
                        buf.extend(std::iter::repeat(REPLACEMENT).take(width));
                        line_width = width;
                        word_length = width;
                    } else {
                        buf.push(c);
                        line_width = c_width;
                        word_length += c_width;
                    }

                    if is_empty_buf {
                        is_empty_buf = false;
                    }
                }
            }
        }

        let _ = write!(buf, "{}", block.end());
    }

    if line_width > 0 {
        buf.push_str(suffix);
    }

    // fill the remainings in a last line if it has any.
    if line_width < width {
        let rest = width - line_width;
        buf.extend(std::iter::repeat(' ').take(rest));
    }

    buf
}

fn split_string_at(text: &str, at: usize) -> (&str, &str, (usize, usize)) {
    use papergrid::util::split_at_pos;

    let (length, count_unknowns, split_char_size) = split_at_pos(text, at);
    let (lhs, rhs) = text.split_at(length);

    (lhs, rhs, (count_unknowns, split_char_size))
}

#[cfg(feature = "color")]
mod link_extraction {
    //! The module is based on Dan Davison <https://github.com/dandavison> delta <https://github.com/dandavison/delta> ansi library.

    use core::str::Bytes;
    use vte::Params;

    /// Strip OSC codes from `s`. If `s` is a single OSC8 hyperlink, with no other text, then return
    /// (s_with_all_hyperlinks_removed, Some(url)). If `s` does not meet this description, then return
    /// (s_with_all_hyperlinks_removed, None). Any ANSI color sequences in `s` will be retained. See
    /// https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda
    pub(super) fn strip_osc(s: &str) -> (String, Option<String>) {
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
        let mut text = String::with_capacity(s.len());

        for el in AnsiElementIterator::new(s) {
            match el {
                Element::Osc(i, j) => match state {
                    ExpectOsc8Url => {
                        url = Some(&s[i..j]);
                        state = ExpectFirstText;
                    }
                    ExpectMoreTextOrTerminator => state = SeenOneHyperlink,
                    _ => state = WillNotReturnUrl,
                },
                Element::Sgr(i, j) => text.push_str(&s[i..j]),
                Element::Csi(i, j) => text.push_str(&s[i..j]),
                Element::Esc(_, _) => {}
                Element::Text(i, j) => {
                    text.push_str(&s[i..j]);
                    match state {
                        ExpectFirstText => state = ExpectMoreTextOrTerminator,
                        ExpectMoreTextOrTerminator => {}
                        _ => state = WillNotReturnUrl,
                    }
                }
            }
        }

        match state {
            WillNotReturnUrl => (text, None),
            _ => {
                let url = url.and_then(|s| {
                    s.strip_prefix("\x1b]8;;")
                        .and_then(|s| s.strip_suffix('\x1b'))
                });
                if let Some(url) = url {
                    (text, Some(url.to_string()))
                } else {
                    (text, None)
                }
            }
        }
    }

    struct AnsiElementIterator<'a> {
        // The input bytes
        bytes: Bytes<'a>,

        // The state machine
        machine: vte::Parser,

        // Becomes non-None when the parser finishes parsing an ANSI sequence.
        // This is never Element::Text.
        element: Option<Element>,

        // Number of text bytes seen since the last element was emitted.
        text_length: usize,

        // Byte offset of start of current element.
        start: usize,

        // Byte offset of most rightward byte processed so far
        pos: usize,
    }

    #[derive(Default)]
    struct Performer {
        // Becomes non-None when the parser finishes parsing an ANSI sequence.
        // This is never Element::Text.
        element: Option<Element>,

        // Number of text bytes seen since the last element was emitted.
        text_length: usize,
    }

    #[derive(Debug)]
    enum Element {
        // TODO: capture SGR Params. Delta captures these as an ansi_term::Style struct.
        // https://github.com/dandavison/delta/blob/1193d54d5c90ab3a45048de3fd1e95c7c2580014/src/ansi/iterator.rs#L136-L137
        // However, the ansi_term crate is unmaintained.
        Sgr(usize, usize),
        Csi(usize, usize),
        Esc(usize, usize),
        Osc(usize, usize),
        Text(usize, usize),
    }

    impl Element {
        fn set_range(&mut self, start: usize, end: usize) {
            let (from, to) = match self {
                Element::Sgr(from, to) => (from, to),
                Element::Csi(from, to) => (from, to),
                Element::Esc(from, to) => (from, to),
                Element::Osc(from, to) => (from, to),
                Element::Text(from, to) => (from, to),
            };

            *from = start;
            *to = end;
        }
    }

    impl<'a> AnsiElementIterator<'a> {
        fn new(s: &'a str) -> Self {
            Self {
                machine: vte::Parser::new(),
                bytes: s.bytes(),
                element: None,
                text_length: 0,
                start: 0,
                pos: 0,
            }
        }

        fn advance_vte(&mut self, byte: u8) {
            let mut performer = Performer::default();
            self.machine.advance(&mut performer, byte);
            self.element = performer.element;
            self.text_length += performer.text_length;
            self.pos += 1;
        }
    }

    impl<'a> Iterator for AnsiElementIterator<'a> {
        type Item = Element;

        fn next(&mut self) -> Option<Element> {
            // If the last element emitted was text, then there may be a non-text element waiting
            // to be emitted. In that case we do not consume a new byte.
            while self.element.is_none() {
                match self.bytes.next() {
                    Some(b) => self.advance_vte(b),
                    None => break,
                }
            }

            if let Some(mut element) = self.element.take() {
                // There is a non-text element waiting to be emitted, but it may have preceding
                // text, which must be emitted first.
                if self.text_length > 0 {
                    let start = self.start;
                    self.start += self.text_length;
                    self.text_length = 0;
                    self.element = Some(element);
                    return Some(Element::Text(start, self.start));
                }

                let start = self.start;
                self.start = self.pos;
                element.set_range(start, self.pos);

                return Some(element);
            }

            if self.text_length > 0 {
                self.text_length = 0;
                return Some(Element::Text(self.start, self.pos));
            }

            None
        }
    }

    // Based on https://github.com/alacritty/vte/blob/v0.9.0/examples/parselog.rs
    impl vte::Perform for Performer {
        fn csi_dispatch(&mut self, params: &Params, intermediates: &[u8], ignore: bool, c: char) {
            if ignore || intermediates.len() > 1 {
                return;
            }

            let is_sgr = c == 'm' && intermediates.first().is_none();
            let element = if is_sgr {
                if params.is_empty() {
                    // Attr::Reset
                    // Probably doesn't need to be handled: https://github.com/dandavison/delta/pull/431#discussion_r536883568
                    None
                } else {
                    Some(Element::Sgr(0, 0))
                }
            } else {
                Some(Element::Csi(0, 0))
            };

            self.element = element;
        }

        fn print(&mut self, c: char) {
            self.text_length += c.len_utf8();
        }

        fn execute(&mut self, byte: u8) {
            // E.g. '\n'
            if byte < 128 {
                self.text_length += 1;
            }
        }

        fn hook(&mut self, _params: &Params, _intermediates: &[u8], _ignore: bool, _c: char) {}

        fn put(&mut self, _byte: u8) {}

        fn unhook(&mut self) {}

        fn osc_dispatch(&mut self, _params: &[&[u8]], _bell_terminated: bool) {
            self.element = Some(Element::Osc(0, 0));
        }

        fn esc_dispatch(&mut self, _intermediates: &[u8], _ignore: bool, _byte: u8) {
            self.element = Some(Element::Esc(0, 0));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "color")]
    #[test]
    fn test_color_strip() {
        use owo_colors::{colors::Yellow, OwoColorize};
        use papergrid::util::cut_str;

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
    fn split_test() {
        #[cfg(not(feature = "color"))]
        let split = |text, width| chunks(text, width).join("\n");

        #[cfg(feature = "color")]
        let split = |text, width| chunks(text, width, "", "").join("\n");

        assert_eq!(split("123456", 0), "");

        assert_eq!(split("123456", 1), "1\n2\n3\n4\n5\n6");
        assert_eq!(split("123456", 2), "12\n34\n56");
        assert_eq!(split("12345", 2), "12\n34\n5");
        assert_eq!(split("123456", 6), "123456");
        assert_eq!(split("123456", 10), "123456");

        assert_eq!(split("😳😳😳😳😳", 1), "�\n�\n�\n�\n�");
        assert_eq!(split("😳😳😳😳😳", 2), "😳\n😳\n😳\n😳\n😳");
        assert_eq!(split("😳😳😳😳😳", 3), "😳�\n😳�\n😳");
        assert_eq!(split("😳😳😳😳😳", 6), "😳😳😳\n😳😳");
        assert_eq!(split("😳😳😳😳😳", 20), "😳😳😳😳😳");

        assert_eq!(split("😳123😳", 1), "�\n1\n2\n3\n�");
        assert_eq!(split("😳12😳3", 1), "�\n1\n2\n�\n3");
    }

    #[test]
    fn chunks_test() {
        #[cfg(not(feature = "color"))]
        let chunks = |text, width| chunks(text, width);

        #[cfg(feature = "color")]
        let chunks = |text, width| chunks(text, width, "", "");

        assert_eq!(chunks("123456", 0), [""; 0]);

        assert_eq!(chunks("123456", 1), ["1", "2", "3", "4", "5", "6"]);
        assert_eq!(chunks("123456", 2), ["12", "34", "56"]);
        assert_eq!(chunks("12345", 2), ["12", "34", "5"]);

        assert_eq!(chunks("😳😳😳😳😳", 1), ["�", "�", "�", "�", "�"]);
        assert_eq!(chunks("😳😳😳😳😳", 2), ["😳", "😳", "😳", "😳", "😳"]);
        assert_eq!(chunks("😳😳😳😳😳", 3), ["😳�", "😳�", "😳"]);
    }

    #[cfg(not(feature = "color"))]
    #[test]
    fn split_by_line_keeping_words_test() {
        let split_keeping_words = |text, width| split_keeping_words(text, width, "\n");

        assert_eq!(split_keeping_words("123456", 1), "1\n2\n3\n4\n5\n6");
        assert_eq!(split_keeping_words("123456", 2), "12\n34\n56");
        assert_eq!(split_keeping_words("12345", 2), "12\n34\n5 ");

        assert_eq!(split_keeping_words("😳😳😳😳😳", 1), "�\n�\n�\n�\n�");

        assert_eq!(split_keeping_words("111 234 1", 4), "111 \n234 \n1   ");
    }

    #[cfg(feature = "color")]
    #[test]
    fn split_by_line_keeping_words_test() {
        #[cfg(feature = "color")]
        let split_keeping_words = |text, width| split_keeping_words(text, width, "", "");

        assert_eq!(split_keeping_words("123456", 1), "1\n2\n3\n4\n5\n6");
        assert_eq!(split_keeping_words("123456", 2), "12\n34\n56");
        assert_eq!(split_keeping_words("12345", 2), "12\n34\n5 ");

        assert_eq!(split_keeping_words("😳😳😳😳😳", 1), "�\n�\n�\n�\n�");

        assert_eq!(split_keeping_words("111 234 1", 4), "111 \n234 \n1   ");
    }

    #[cfg(feature = "color")]
    #[test]
    fn split_by_line_keeping_words_color_test() {
        #[cfg(feature = "color")]
        let split_keeping_words = |text, width| split_keeping_words(text, width, "", "");

        #[cfg(not(feature = "color"))]
        let split_keeping_words = |text, width| split_keeping_words(text, width, "\n");

        let text = "\u{1b}[36mJapanese “vacancy” button\u{1b}[0m";

        println!("{}", split_keeping_words(text, 2));
        println!("{}", split_keeping_words(text, 1));

        assert_eq!(split_keeping_words(text, 2), "\u{1b}[36mJa\u{1b}[39m\n\u{1b}[36mpa\u{1b}[39m\n\u{1b}[36mne\u{1b}[39m\n\u{1b}[36mse\u{1b}[39m\n\u{1b}[36m \u{1b}[39m\n\u{1b}[36m“v\u{1b}[39m\n\u{1b}[36mac\u{1b}[39m\n\u{1b}[36man\u{1b}[39m\n\u{1b}[36mcy\u{1b}[39m\n\u{1b}[36m” \u{1b}[39m\n\u{1b}[36mbu\u{1b}[39m\n\u{1b}[36mtt\u{1b}[39m\n\u{1b}[36mon\u{1b}[39m");
        assert_eq!(split_keeping_words(text, 1), "\u{1b}[36mJ\u{1b}[39m\n\u{1b}[36ma\u{1b}[39m\n\u{1b}[36mp\u{1b}[39m\n\u{1b}[36ma\u{1b}[39m\n\u{1b}[36mn\u{1b}[39m\n\u{1b}[36me\u{1b}[39m\n\u{1b}[36ms\u{1b}[39m\n\u{1b}[36me\u{1b}[39m\n\u{1b}[36m \u{1b}[39m\n\u{1b}[36m“\u{1b}[39m\n\u{1b}[36mv\u{1b}[39m\n\u{1b}[36ma\u{1b}[39m\n\u{1b}[36mc\u{1b}[39m\n\u{1b}[36ma\u{1b}[39m\n\u{1b}[36mn\u{1b}[39m\n\u{1b}[36mc\u{1b}[39m\n\u{1b}[36my\u{1b}[39m\n\u{1b}[36m”\u{1b}[39m\n\u{1b}[36m \u{1b}[39m\n\u{1b}[36mb\u{1b}[39m\n\u{1b}[36mu\u{1b}[39m\n\u{1b}[36mt\u{1b}[39m\n\u{1b}[36mt\u{1b}[39m\n\u{1b}[36mo\u{1b}[39m\n\u{1b}[36mn\u{1b}[39m");
    }

    #[cfg(feature = "color")]
    #[test]
    fn split_by_line_keeping_words_color_2_test() {
        use ansi_str::AnsiStr;

        #[cfg(feature = "color")]
        let split_keeping_words = |text, width| split_keeping_words(text, width, "", "");

        #[cfg(not(feature = "color"))]
        let split_keeping_words = |text, width| split_keeping_words(text, width, "\n");

        let text = "\u{1b}[37mTigre Ecuador   OMYA Andina     3824909999      Calcium carbonate       Colombia\u{1b}[0m";

        assert_eq!(
            split_keeping_words(text, 2)
                .ansi_split("\n")
                .collect::<Vec<_>>(),
            [
                "\u{1b}[37mTi\u{1b}[39m",
                "\u{1b}[37mgr\u{1b}[39m",
                "\u{1b}[37me \u{1b}[39m",
                "\u{1b}[37mEc\u{1b}[39m",
                "\u{1b}[37mua\u{1b}[39m",
                "\u{1b}[37mdo\u{1b}[39m",
                "\u{1b}[37mr \u{1b}[39m",
                "\u{1b}[37m  \u{1b}[39m",
                "\u{1b}[37mOM\u{1b}[39m",
                "\u{1b}[37mYA\u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37mAn\u{1b}[39m",
                "\u{1b}[37mdi\u{1b}[39m",
                "\u{1b}[37mna\u{1b}[39m",
                "\u{1b}[37m  \u{1b}[39m",
                "\u{1b}[37m  \u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37m38\u{1b}[39m",
                "\u{1b}[37m24\u{1b}[39m",
                "\u{1b}[37m90\u{1b}[39m",
                "\u{1b}[37m99\u{1b}[39m",
                "\u{1b}[37m99\u{1b}[39m",
                "\u{1b}[37m  \u{1b}[39m",
                "\u{1b}[37m  \u{1b}[39m",
                "\u{1b}[37m  \u{1b}[39m",
                "\u{1b}[37mCa\u{1b}[39m",
                "\u{1b}[37mlc\u{1b}[39m",
                "\u{1b}[37miu\u{1b}[39m",
                "\u{1b}[37mm \u{1b}[39m",
                "\u{1b}[37mca\u{1b}[39m",
                "\u{1b}[37mrb\u{1b}[39m",
                "\u{1b}[37mon\u{1b}[39m",
                "\u{1b}[37mat\u{1b}[39m",
                "\u{1b}[37me \u{1b}[39m",
                "\u{1b}[37m  \u{1b}[39m",
                "\u{1b}[37m  \u{1b}[39m",
                "\u{1b}[37m  \u{1b}[39m",
                "\u{1b}[37mCo\u{1b}[39m",
                "\u{1b}[37mlo\u{1b}[39m",
                "\u{1b}[37mmb\u{1b}[39m",
                "\u{1b}[37mia\u{1b}[39m"
            ]
        );

        assert_eq!(
            split_keeping_words(text, 1)
                .ansi_split("\n")
                .collect::<Vec<_>>(),
            [
                "\u{1b}[37mT\u{1b}[39m",
                "\u{1b}[37mi\u{1b}[39m",
                "\u{1b}[37mg\u{1b}[39m",
                "\u{1b}[37mr\u{1b}[39m",
                "\u{1b}[37me\u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37mE\u{1b}[39m",
                "\u{1b}[37mc\u{1b}[39m",
                "\u{1b}[37mu\u{1b}[39m",
                "\u{1b}[37ma\u{1b}[39m",
                "\u{1b}[37md\u{1b}[39m",
                "\u{1b}[37mo\u{1b}[39m",
                "\u{1b}[37mr\u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37mO\u{1b}[39m",
                "\u{1b}[37mM\u{1b}[39m",
                "\u{1b}[37mY\u{1b}[39m",
                "\u{1b}[37mA\u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37mA\u{1b}[39m",
                "\u{1b}[37mn\u{1b}[39m",
                "\u{1b}[37md\u{1b}[39m",
                "\u{1b}[37mi\u{1b}[39m",
                "\u{1b}[37mn\u{1b}[39m",
                "\u{1b}[37ma\u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37m3\u{1b}[39m",
                "\u{1b}[37m8\u{1b}[39m",
                "\u{1b}[37m2\u{1b}[39m",
                "\u{1b}[37m4\u{1b}[39m",
                "\u{1b}[37m9\u{1b}[39m",
                "\u{1b}[37m0\u{1b}[39m",
                "\u{1b}[37m9\u{1b}[39m",
                "\u{1b}[37m9\u{1b}[39m",
                "\u{1b}[37m9\u{1b}[39m",
                "\u{1b}[37m9\u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37mC\u{1b}[39m",
                "\u{1b}[37ma\u{1b}[39m",
                "\u{1b}[37ml\u{1b}[39m",
                "\u{1b}[37mc\u{1b}[39m",
                "\u{1b}[37mi\u{1b}[39m",
                "\u{1b}[37mu\u{1b}[39m",
                "\u{1b}[37mm\u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37mc\u{1b}[39m",
                "\u{1b}[37ma\u{1b}[39m",
                "\u{1b}[37mr\u{1b}[39m",
                "\u{1b}[37mb\u{1b}[39m",
                "\u{1b}[37mo\u{1b}[39m",
                "\u{1b}[37mn\u{1b}[39m",
                "\u{1b}[37ma\u{1b}[39m",
                "\u{1b}[37mt\u{1b}[39m",
                "\u{1b}[37me\u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37mC\u{1b}[39m",
                "\u{1b}[37mo\u{1b}[39m",
                "\u{1b}[37ml\u{1b}[39m",
                "\u{1b}[37mo\u{1b}[39m",
                "\u{1b}[37mm\u{1b}[39m",
                "\u{1b}[37mb\u{1b}[39m",
                "\u{1b}[37mi\u{1b}[39m",
                "\u{1b}[37ma\u{1b}[39m"
            ]
        )
    }

    #[cfg(feature = "color")]
    #[test]
    fn split_by_line_keeping_words_color_3_test() {
        let split_keeping_words = |text, width| split_keeping_words(text, width, "", "");

        println!(
            "{}",
            split_keeping_words("\u{1b}[37mthis is a long sentence\u{1b}[0m", 7)
        );

        println!(
            "{}",
            split_keeping_words(
                "\u{1b}[37m🚵🏻🚵🏻🚵🏻🚵🏻🚵🏻🚵🏻🚵🏻🚵🏻🚵🏻🚵🏻\u{1b}[0m",
                3,
            ),
        );

        assert_eq!(
            split_keeping_words(
                "\u{1b}[37m🚵🏻🚵🏻🚵🏻🚵🏻🚵🏻🚵🏻🚵🏻🚵🏻🚵🏻🚵🏻\u{1b}[0m",
                3,
            ),
            "\u{1b}[37m🚵\u{1b}[39m\n\u{1b}[37m🏻\u{1b}[39m\n\u{1b}[37m🚵\u{1b}[39m\n\u{1b}[37m🏻\u{1b}[39m\n\u{1b}[37m🚵\u{1b}[39m\n\u{1b}[37m🏻\u{1b}[39m\n\u{1b}[37m🚵\u{1b}[39m\n\u{1b}[37m🏻\u{1b}[39m\n\u{1b}[37m🚵\u{1b}[39m\n\u{1b}[37m🏻\u{1b}[39m\n\u{1b}[37m🚵\u{1b}[39m\n\u{1b}[37m🏻\u{1b}[39m\n\u{1b}[37m🚵\u{1b}[39m\n\u{1b}[37m🏻\u{1b}[39m\n\u{1b}[37m🚵\u{1b}[39m\n\u{1b}[37m🏻\u{1b}[39m\n\u{1b}[37m🚵\u{1b}[39m\n\u{1b}[37m🏻\u{1b}[39m\n\u{1b}[37m🚵\u{1b}[39m\n\u{1b}[37m🏻\u{1b}[39m ",
        );
        assert_eq!(
            split_keeping_words("\u{1b}[37mthis is a long sentence\u{1b}[0m", 7),
            "\u{1b}[37mthis is\u{1b}[39m\n\u{1b}[37m a long\u{1b}[39m\n\u{1b}[37m \u{1b}[39m\n\u{1b}[37msentenc\u{1b}[39m\n\u{1b}[37me\u{1b}[39m      "
        );
        assert_eq!(
            split_keeping_words("\u{1b}[37mHello World\u{1b}[0m", 7),
            "\u{1b}[37mHello \u{1b}[39m\n\u{1b}[37mWorld\u{1b}[39m  "
        );
        assert_eq!(
            split_keeping_words("\u{1b}[37mHello Wo\u{1b}[37mrld\u{1b}[0m", 7),
            "\u{1b}[37mHello \u{1b}[39m\n\u{1b}[37mWo\u{1b}[39m\u{1b}[37mrld\u{1b}[39m  "
        );
        assert_eq!(
            split_keeping_words("\u{1b}[37mHello Wo\u{1b}[37mrld\u{1b}[0m", 8),
            "\u{1b}[37mHello \u{1b}[39m\n\u{1b}[37mWo\u{1b}[39m\u{1b}[37mrld\u{1b}[39m   "
        );
    }

    #[cfg(not(feature = "color"))]
    #[test]
    fn split_keeping_words_4_test() {
        let split_keeping_words = |text, width| split_keeping_words(text, width, "\n");

        assert_eq!(split_keeping_words("12345678", 3,), "123\n456\n78 ");
        assert_eq!(split_keeping_words("12345678", 2,), "12\n34\n56\n78");
    }

    #[cfg(feature = "color")]
    #[test]
    fn split_keeping_words_4_test() {
        let split_keeping_words = |text, width| split_keeping_words(text, width, "", "");

        #[cfg(not(feature = "color"))]
        let split_keeping_words = |text, width| split_keeping_words(text, width, "\n");

        assert_eq!(split_keeping_words("12345678", 3,), "123\n456\n78 ");
        assert_eq!(split_keeping_words("12345678", 2,), "12\n34\n56\n78");
    }

    #[cfg(feature = "color")]
    #[test]
    fn chunks_test_with_prefix_and_suffix() {
        assert_eq!(chunks("123456", 0, "^", "$"), ["^$"; 0]);

        assert_eq!(
            chunks("123456", 1, "^", "$"),
            ["^1$", "^2$", "^3$", "^4$", "^5$", "^6$"]
        );
        assert_eq!(chunks("123456", 2, "^", "$"), ["^12$", "^34$", "^56$"]);
        assert_eq!(chunks("12345", 2, "^", "$"), ["^12$", "^34$", "^5$"]);

        assert_eq!(
            chunks("😳😳😳😳😳", 1, "^", "$"),
            ["^�$", "^�$", "^�$", "^�$", "^�$"]
        );
        assert_eq!(
            chunks("😳😳😳😳😳", 2, "^", "$"),
            ["^😳$", "^😳$", "^😳$", "^😳$", "^😳$"]
        );
        assert_eq!(
            chunks("😳😳😳😳😳", 3, "^", "$"),
            ["^😳�$", "^😳�$", "^😳$"]
        );
    }

    #[cfg(feature = "color")]
    #[test]
    fn split_by_line_keeping_words_test_with_prefix_and_suffix() {
        assert_eq!(
            split_keeping_words("123456", 1, "^", "$"),
            "^1$\n^2$\n^3$\n^4$\n^5$\n^6$"
        );
        assert_eq!(
            split_keeping_words("123456", 2, "^", "$"),
            "^12$\n^34$\n^56$"
        );
        assert_eq!(
            split_keeping_words("12345", 2, "^", "$"),
            "^12$\n^34$\n^5$ "
        );

        assert_eq!(
            split_keeping_words("😳😳😳😳😳", 1, "^", "$"),
            "^�$\n^�$\n^�$\n^�$\n^�$"
        );
    }

    #[cfg(feature = "color")]
    #[test]
    fn split_by_line_keeping_words_color_2_test_with_prefix_and_suffix() {
        use ansi_str::AnsiStr;

        let text = "\u{1b}[37mTigre Ecuador   OMYA Andina     3824909999      Calcium carbonate       Colombia\u{1b}[0m";

        assert_eq!(
            split_keeping_words(text, 2, "^", "$")
                .ansi_split("\n")
                .collect::<Vec<_>>(),
            [
                "^\u{1b}[37mTi\u{1b}[39m$",
                "^\u{1b}[37mgr\u{1b}[39m$",
                "^\u{1b}[37me \u{1b}[39m$",
                "^\u{1b}[37mEc\u{1b}[39m$",
                "^\u{1b}[37mua\u{1b}[39m$",
                "^\u{1b}[37mdo\u{1b}[39m$",
                "^\u{1b}[37mr \u{1b}[39m$",
                "^\u{1b}[37m  \u{1b}[39m$",
                "^\u{1b}[37mOM\u{1b}[39m$",
                "^\u{1b}[37mYA\u{1b}[39m$",
                "^\u{1b}[37m \u{1b}[39m$",
                "^\u{1b}[37mAn\u{1b}[39m$",
                "^\u{1b}[37mdi\u{1b}[39m$",
                "^\u{1b}[37mna\u{1b}[39m$",
                "^\u{1b}[37m  \u{1b}[39m$",
                "^\u{1b}[37m  \u{1b}[39m$",
                "^\u{1b}[37m \u{1b}[39m$",
                "^\u{1b}[37m38\u{1b}[39m$",
                "^\u{1b}[37m24\u{1b}[39m$",
                "^\u{1b}[37m90\u{1b}[39m$",
                "^\u{1b}[37m99\u{1b}[39m$",
                "^\u{1b}[37m99\u{1b}[39m$",
                "^\u{1b}[37m  \u{1b}[39m$",
                "^\u{1b}[37m  \u{1b}[39m$",
                "^\u{1b}[37m  \u{1b}[39m$",
                "^\u{1b}[37mCa\u{1b}[39m$",
                "^\u{1b}[37mlc\u{1b}[39m$",
                "^\u{1b}[37miu\u{1b}[39m$",
                "^\u{1b}[37mm \u{1b}[39m$",
                "^\u{1b}[37mca\u{1b}[39m$",
                "^\u{1b}[37mrb\u{1b}[39m$",
                "^\u{1b}[37mon\u{1b}[39m$",
                "^\u{1b}[37mat\u{1b}[39m$",
                "^\u{1b}[37me \u{1b}[39m$",
                "^\u{1b}[37m  \u{1b}[39m$",
                "^\u{1b}[37m  \u{1b}[39m$",
                "^\u{1b}[37m  \u{1b}[39m$",
                "^\u{1b}[37mCo\u{1b}[39m$",
                "^\u{1b}[37mlo\u{1b}[39m$",
                "^\u{1b}[37mmb\u{1b}[39m$",
                "^\u{1b}[37mia\u{1b}[39m$"
            ]
        );

        assert_eq!(
            split_keeping_words(text, 1, "^", "$")
                .ansi_split("\n")
                .collect::<Vec<_>>(),
            [
                "^\u{1b}[37mT\u{1b}[39m$",
                "^\u{1b}[37mi\u{1b}[39m$",
                "^\u{1b}[37mg\u{1b}[39m$",
                "^\u{1b}[37mr\u{1b}[39m$",
                "^\u{1b}[37me\u{1b}[39m$",
                "^\u{1b}[37m \u{1b}[39m$",
                "^\u{1b}[37mE\u{1b}[39m$",
                "^\u{1b}[37mc\u{1b}[39m$",
                "^\u{1b}[37mu\u{1b}[39m$",
                "^\u{1b}[37ma\u{1b}[39m$",
                "^\u{1b}[37md\u{1b}[39m$",
                "^\u{1b}[37mo\u{1b}[39m$",
                "^\u{1b}[37mr\u{1b}[39m$",
                "^\u{1b}[37m \u{1b}[39m$",
                "^\u{1b}[37m \u{1b}[39m$",
                "^\u{1b}[37m \u{1b}[39m$",
                "^\u{1b}[37mO\u{1b}[39m$",
                "^\u{1b}[37mM\u{1b}[39m$",
                "^\u{1b}[37mY\u{1b}[39m$",
                "^\u{1b}[37mA\u{1b}[39m$",
                "^\u{1b}[37m \u{1b}[39m$",
                "^\u{1b}[37mA\u{1b}[39m$",
                "^\u{1b}[37mn\u{1b}[39m$",
                "^\u{1b}[37md\u{1b}[39m$",
                "^\u{1b}[37mi\u{1b}[39m$",
                "^\u{1b}[37mn\u{1b}[39m$",
                "^\u{1b}[37ma\u{1b}[39m$",
                "^\u{1b}[37m \u{1b}[39m$",
                "^\u{1b}[37m \u{1b}[39m$",
                "^\u{1b}[37m \u{1b}[39m$",
                "^\u{1b}[37m \u{1b}[39m$",
                "^\u{1b}[37m \u{1b}[39m$",
                "^\u{1b}[37m3\u{1b}[39m$",
                "^\u{1b}[37m8\u{1b}[39m$",
                "^\u{1b}[37m2\u{1b}[39m$",
                "^\u{1b}[37m4\u{1b}[39m$",
                "^\u{1b}[37m9\u{1b}[39m$",
                "^\u{1b}[37m0\u{1b}[39m$",
                "^\u{1b}[37m9\u{1b}[39m$",
                "^\u{1b}[37m9\u{1b}[39m$",
                "^\u{1b}[37m9\u{1b}[39m$",
                "^\u{1b}[37m9\u{1b}[39m$",
                "^\u{1b}[37m \u{1b}[39m$",
                "^\u{1b}[37m \u{1b}[39m$",
                "^\u{1b}[37m \u{1b}[39m$",
                "^\u{1b}[37m \u{1b}[39m$",
                "^\u{1b}[37m \u{1b}[39m$",
                "^\u{1b}[37m \u{1b}[39m$",
                "^\u{1b}[37mC\u{1b}[39m$",
                "^\u{1b}[37ma\u{1b}[39m$",
                "^\u{1b}[37ml\u{1b}[39m$",
                "^\u{1b}[37mc\u{1b}[39m$",
                "^\u{1b}[37mi\u{1b}[39m$",
                "^\u{1b}[37mu\u{1b}[39m$",
                "^\u{1b}[37mm\u{1b}[39m$",
                "^\u{1b}[37m \u{1b}[39m$",
                "^\u{1b}[37mc\u{1b}[39m$",
                "^\u{1b}[37ma\u{1b}[39m$",
                "^\u{1b}[37mr\u{1b}[39m$",
                "^\u{1b}[37mb\u{1b}[39m$",
                "^\u{1b}[37mo\u{1b}[39m$",
                "^\u{1b}[37mn\u{1b}[39m$",
                "^\u{1b}[37ma\u{1b}[39m$",
                "^\u{1b}[37mt\u{1b}[39m$",
                "^\u{1b}[37me\u{1b}[39m$",
                "^\u{1b}[37m \u{1b}[39m$",
                "^\u{1b}[37m \u{1b}[39m$",
                "^\u{1b}[37m \u{1b}[39m$",
                "^\u{1b}[37m \u{1b}[39m$",
                "^\u{1b}[37m \u{1b}[39m$",
                "^\u{1b}[37m \u{1b}[39m$",
                "^\u{1b}[37m \u{1b}[39m$",
                "^\u{1b}[37mC\u{1b}[39m$",
                "^\u{1b}[37mo\u{1b}[39m$",
                "^\u{1b}[37ml\u{1b}[39m$",
                "^\u{1b}[37mo\u{1b}[39m$",
                "^\u{1b}[37mm\u{1b}[39m$",
                "^\u{1b}[37mb\u{1b}[39m$",
                "^\u{1b}[37mi\u{1b}[39m$",
                "^\u{1b}[37ma\u{1b}[39m$"
            ]
        )
    }

    #[cfg(feature = "color")]
    #[test]
    fn chunks_wrap_2() {
        let text = "\u{1b}[30mDebian\u{1b}[0m\u{1b}[31mDebian\u{1b}[0m\u{1b}[32mDebian\u{1b}[0m\u{1b}[33mDebian\u{1b}[0m\u{1b}[34mDebian\u{1b}[0m\u{1b}[35mDebian\u{1b}[0m\u{1b}[36mDebian\u{1b}[0m\u{1b}[37mDebian\u{1b}[0m\u{1b}[40mDebian\u{1b}[0m\u{1b}[41mDebian\u{1b}[0m\u{1b}[42mDebian\u{1b}[0m\u{1b}[43mDebian\u{1b}[0m\u{1b}[44mDebian\u{1b}[0m";

        assert_eq!(
            chunks(text, 30, "", ""),
            [
                "\u{1b}[30mDebian\u{1b}[39m\u{1b}[31mDebian\u{1b}[39m\u{1b}[32mDebian\u{1b}[39m\u{1b}[33mDebian\u{1b}[39m\u{1b}[34mDebian\u{1b}[39m\u{1b}[35m\u{1b}[39m", "\u{1b}[35mDebian\u{1b}[39m\u{1b}[36mDebian\u{1b}[39m\u{1b}[37mDebian\u{1b}[39m\u{1b}[40mDebian\u{1b}[49m\u{1b}[41mDebian\u{1b}[49m\u{1b}[42m\u{1b}[49m", "\u{1b}[42mDebian\u{1b}[49m\u{1b}[43mDebian\u{1b}[49m\u{1b}[44mDebian\u{1b}[49m"
            ]
        );
    }
}
