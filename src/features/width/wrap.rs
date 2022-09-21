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

pub(crate) fn wrap_text(text: &str, width: usize, keep_words: bool) -> String {
    if width == 0 {
        String::new()
    } else if keep_words {
        split_keeping_words(text, width, "\n")
    } else {
        split(text, width, "\n")
    }
}

fn split(s: &str, width: usize, sep: &str) -> String {
    if width == 0 {
        return String::new();
    }

    chunks(s, width).join(sep)
}

#[cfg(not(feature = "color"))]
fn chunks(s: &str, width: usize) -> Vec<String> {
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
fn chunks(s: &str, width: usize) -> Vec<String> {
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

        let _ = write!(&mut line, "{}", b.start());

        let mut part = b.text();
        while !part.is_empty() {
            let available_space = width - line_width;

            let part_width = unicode_width::UnicodeWidthStr::width(part);
            if part_width <= available_space {
                line.push_str(part);
                line_width += part_width;

                if line_width == available_space {
                    let _ = write!(&mut line, "{}", b.end());
                    list.push(line);
                    line = String::with_capacity(width);
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
                list.push(line);
                line = String::with_capacity(width);
                line_width = 0;
                let _ = write!(&mut line, "{}", b.start());
            }
        }

        if line_width > 0 {
            let _ = write!(&mut line, "{}", b.end());
        }
    }

    if line_width > 0 {
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
fn split_keeping_words(text: &str, width: usize, sep: &str) -> String {
    use ansi_str::AnsiStr;
    use std::borrow::Cow;

    if text.is_empty() || width == 0 {
        return String::new();
    }

    let mut buf = String::with_capacity(width);
    let mut line_width = 0;

    // looking up the words
    let mut st = Cow::Borrowed(text);
    while let Some(pos) = st.ansi_find(" ") {
        if pos > 0 {
            let word = st.ansi_cut(..pos);
            split_keeping_word(word, sep, width, &mut buf, &mut line_width);
        }

        // we get space char because it might be colored
        let space = st.ansi_cut(pos..pos + 1);

        let line_has_space = line_width < width;
        if !line_has_space {
            buf.push_str(sep);
            line_width = 0;
        }

        buf.push_str(&space);
        line_width += 1;

        st = Cow::Owned(st.ansi_cut(pos + 1..).into_owned());
    }

    if !st.is_empty() {
        split_keeping_word(st, sep, width, &mut buf, &mut line_width);
    }

    if line_width < width {
        buf.extend(std::iter::repeat(' ').take(width - line_width));
    }

    buf
}

#[cfg(feature = "color")]
fn split_keeping_word(
    word: std::borrow::Cow<'_, str>,
    sep: &str,
    width: usize,
    buf: &mut String,
    line_width: &mut usize,
) {
    let word_width = papergrid::util::string_width(word.as_ref());

    let line_has_space = *line_width + word_width <= width;
    if line_has_space {
        buf.push_str(&word);
        *line_width += word_width;
    } else if word_width <= width {
        // the word can be fit to 'width' so we put it on new line

        buf.extend(std::iter::repeat(' ').take(width - *line_width));
        buf.push_str(sep);

        buf.push_str(&word);
        *line_width = word_width;
    } else {
        // the word is too long any way so we split it

        let mut part = word;
        while !part.is_empty() {
            if *line_width == width {
                buf.push_str(sep);
                *line_width = 0;
            }

            let available_space = width - *line_width;

            let (lhs, mut rhs, (unknowns, split_char)) =
                split_string_at_colored(&part, available_space);

            rhs.drain(..split_char);
            part = std::borrow::Cow::Owned(rhs);

            buf.push_str(&lhs);
            const REPLACEMENT: char = '\u{FFFD}';
            buf.extend(std::iter::repeat(REPLACEMENT).take(unknowns));

            *line_width += papergrid::util::string_width(&lhs) + unknowns;
        }
    }
}

fn split_string_at(text: &str, at: usize) -> (&str, &str, (usize, usize)) {
    use papergrid::util::split_at_pos;

    let (length, count_unknowns, split_char_size) = split_at_pos(text, at);
    let (lhs, rhs) = text.split_at(length);

    (lhs, rhs, (count_unknowns, split_char_size))
}

#[cfg(feature = "color")]
fn split_string_at_colored(text: &str, at: usize) -> (String, String, (usize, usize)) {
    use papergrid::util::split_at_pos;

    let s = ansi_str::AnsiStr::ansi_strip(text);
    let (length, count_unknowns, split_char_size) = split_at_pos(&s, at);
    let (lhs, rhs) = ansi_str::AnsiStr::ansi_split_at(text, length);

    (
        lhs.into_owned(),
        rhs.into_owned(),
        (count_unknowns, split_char_size),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_test() {
        assert_eq!(split("123456", 0, "\n"), "");

        assert_eq!(split("123456", 1, "\n"), "1\n2\n3\n4\n5\n6");
        assert_eq!(split("123456", 2, "\n"), "12\n34\n56");
        assert_eq!(split("12345", 2, "\n"), "12\n34\n5");
        assert_eq!(split("123456", 6, "\n"), "123456");
        assert_eq!(split("123456", 10, "\n"), "123456");

        assert_eq!(split("😳😳😳😳😳", 1, "\n"), "�\n�\n�\n�\n�");
        assert_eq!(split("😳😳😳😳😳", 2, "\n"), "😳\n😳\n😳\n😳\n😳");
        assert_eq!(split("😳😳😳😳😳", 3, "\n"), "😳�\n😳�\n😳");
        assert_eq!(split("😳😳😳😳😳", 6, "\n"), "😳😳😳\n😳😳");
        assert_eq!(split("😳😳😳😳😳", 20, "\n"), "😳😳😳😳😳");

        assert_eq!(split("😳123😳", 1, "\n"), "�\n1\n2\n3\n�");
        assert_eq!(split("😳12😳3", 1, "\n"), "�\n1\n2\n�\n3");
    }

    #[cfg(feature = "color")]
    #[test]
    fn chunks_test() {
        assert_eq!(chunks("123456", 0), [""; 0]);

        assert_eq!(chunks("123456", 1), ["1", "2", "3", "4", "5", "6"]);
        assert_eq!(chunks("123456", 2), ["12", "34", "56"]);
        assert_eq!(chunks("12345", 2), ["12", "34", "5"]);

        assert_eq!(chunks("😳😳😳😳😳", 1), ["�", "�", "�", "�", "�"]);
        assert_eq!(chunks("😳😳😳😳😳", 2), ["😳", "😳", "😳", "😳", "😳"]);
        assert_eq!(chunks("😳😳😳😳😳", 3), ["😳�", "😳�", "😳"]);
    }

    #[test]
    fn split_by_line_keeping_words_test() {
        assert_eq!(split_keeping_words("123456", 1, "\n"), "1\n2\n3\n4\n5\n6");
        assert_eq!(split_keeping_words("123456", 2, "\n"), "12\n34\n56");
        assert_eq!(split_keeping_words("12345", 2, "\n"), "12\n34\n5 ");

        assert_eq!(split_keeping_words("😳😳😳😳😳", 1, "\n"), "�\n�\n�\n�\n�");

        assert_eq!(
            split_keeping_words("111 234 1", 4, "\n"),
            "111 \n234 \n1   "
        );
    }

    #[cfg(feature = "color")]
    #[test]
    fn split_by_line_keeping_words_color_test() {
        let text = "\u{1b}[37mJapanese “vacancy” button\u{1b}[0m";

        println!("{}", split_keeping_words(text, 2, "\n"));

        assert_eq!(split_keeping_words(text, 2, "\n"), "\u{1b}[37mJa\u{1b}[39m\n\u{1b}[37mpa\u{1b}[39m\n\u{1b}[37mne\u{1b}[39m\n\u{1b}[37mse\u{1b}[39m\n\u{1b}[37m \u{1b}[39m\u{1b}[37m“\u{1b}[39m\n\u{1b}[37mva\u{1b}[39m\n\u{1b}[37mca\u{1b}[39m\n\u{1b}[37mnc\u{1b}[39m\n\u{1b}[37my”\u{1b}[39m\n\u{1b}[37m \u{1b}[39m\u{1b}[37mb\u{1b}[39m\n\u{1b}[37mut\u{1b}[39m\n\u{1b}[37mto\u{1b}[39m\n\u{1b}[37mn\u{1b}[39m ");
        assert_eq!(split_keeping_words(text, 1, "\n"), "\u{1b}[37mJ\u{1b}[39m\n\u{1b}[37ma\u{1b}[39m\n\u{1b}[37mp\u{1b}[39m\n\u{1b}[37ma\u{1b}[39m\n\u{1b}[37mn\u{1b}[39m\n\u{1b}[37me\u{1b}[39m\n\u{1b}[37ms\u{1b}[39m\n\u{1b}[37me\u{1b}[39m\n\u{1b}[37m \u{1b}[39m\n\u{1b}[37m“\u{1b}[39m\n\u{1b}[37mv\u{1b}[39m\n\u{1b}[37ma\u{1b}[39m\n\u{1b}[37mc\u{1b}[39m\n\u{1b}[37ma\u{1b}[39m\n\u{1b}[37mn\u{1b}[39m\n\u{1b}[37mc\u{1b}[39m\n\u{1b}[37my\u{1b}[39m\n\u{1b}[37m”\u{1b}[39m\n\u{1b}[37m \u{1b}[39m\n\u{1b}[37mb\u{1b}[39m\n\u{1b}[37mu\u{1b}[39m\n\u{1b}[37mt\u{1b}[39m\n\u{1b}[37mt\u{1b}[39m\n\u{1b}[37mo\u{1b}[39m\n\u{1b}[37mn\u{1b}[39m");
    }

    #[cfg(feature = "color")]
    #[test]
    fn split_by_line_keeping_words_color_2_test() {
        use ansi_str::AnsiStr;

        let text = "\u{1b}[37mTigre Ecuador   OMYA Andina     3824909999      Calcium carbonate       Colombia\u{1b}[0m";

        assert_eq!(
            split_keeping_words(text, 2, "\n")
                .ansi_split("\n")
                .collect::<Vec<_>>(),
            [
                "\u{1b}[37mTi\u{1b}[39m",
                "\u{1b}[37mgr\u{1b}[39m",
                "\u{1b}[37me\u{1b}[39m\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37mEc\u{1b}[39m",
                "\u{1b}[37mua\u{1b}[39m",
                "\u{1b}[37mdo\u{1b}[39m",
                "\u{1b}[37mr\u{1b}[39m\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37mOM\u{1b}[39m",
                "\u{1b}[37mYA\u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m\u{1b}[37mA\u{1b}[39m",
                "\u{1b}[37mnd\u{1b}[39m",
                "\u{1b}[37min\u{1b}[39m",
                "\u{1b}[37ma\u{1b}[39m\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37m38\u{1b}[39m",
                "\u{1b}[37m24\u{1b}[39m",
                "\u{1b}[37m90\u{1b}[39m",
                "\u{1b}[37m99\u{1b}[39m",
                "\u{1b}[37m99\u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37mCa\u{1b}[39m",
                "\u{1b}[37mlc\u{1b}[39m",
                "\u{1b}[37miu\u{1b}[39m",
                "\u{1b}[37mm\u{1b}[39m\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37mca\u{1b}[39m",
                "\u{1b}[37mrb\u{1b}[39m",
                "\u{1b}[37mon\u{1b}[39m",
                "\u{1b}[37mat\u{1b}[39m",
                "\u{1b}[37me\u{1b}[39m\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37mCo\u{1b}[39m",
                "\u{1b}[37mlo\u{1b}[39m",
                "\u{1b}[37mmb\u{1b}[39m",
                "\u{1b}[37mia\u{1b}[39m"
            ]
        );

        assert_eq!(
            split_keeping_words(text, 1, "\n")
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
        println!(
            "{}",
            split_keeping_words("\u{1b}[37mthis is a long sentence\u{1b}[0m", 7, "\n")
        );

        assert_eq!(
            split_keeping_words(
                "\u{1b}[37m🚵🏻🚵🏻🚵🏻🚵🏻🚵🏻🚵🏻🚵🏻🚵🏻🚵🏻🚵🏻\u{1b}[0m",
                3,
                "\n"
            ),
            "\u{1b}[37m🚵\u{1b}[39m�\nm🏻\n🚵�\n🚵�\n🚵�\n🚵�\n🚵�\n🚵�\n🚵�\n🚵�\n🚵�"
        );
        assert_eq!(
            split_keeping_words("\u{1b}[37mthis is a long sentence\u{1b}[0m", 7, "\n"),
            "\u{1b}[37mthis\u{1b}[39m\u{1b}[37m \u{1b}[39m\u{1b}[37mis\u{1b}[39m\n\u{1b}[37m \u{1b}[39m\u{1b}[37ma\u{1b}[39m\u{1b}[37m \u{1b}[39m\u{1b}[37mlong\u{1b}[39m\n\u{1b}[37m \u{1b}[39m\u{1b}[37msenten\u{1b}[39m\n\u{1b}[37mce\u{1b}[39m     "
        );
        assert_eq!(
            split_keeping_words("\u{1b}[37mHello World\u{1b}[0m", 7, "\n"),
            "\u{1b}[37mHello\u{1b}[39m\u{1b}[37m \u{1b}[39m \n\u{1b}[37mWorld\u{1b}[0m  "
        );
        assert_eq!(
            split_keeping_words("\u{1b}[37mHello Wo\u{1b}[37mrld\u{1b}[0m", 7, "\n"),
            "\u{1b}[37mHello\u{1b}[39m\u{1b}[37m \u{1b}[39m \n\u{1b}[37mWo\u{1b}[37mrld\u{1b}[0m  "
        );
        assert_eq!(
            split_keeping_words("\u{1b}[37mHello Wo\u{1b}[37mrld\u{1b}[0m", 8, "\n"),
            "\u{1b}[37mHello\u{1b}[39m\u{1b}[37m \u{1b}[39m  \n\u{1b}[37mWo\u{1b}[37mrld\u{1b}[0m   "
        );
    }
}
