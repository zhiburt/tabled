//! This module contains [`Wrap`] structure, used to decrease width of a [`Table`]s or a cell on a [`Table`] by wrapping it's content
//! to a new line.

use std::marker::PhantomData;

use papergrid::util::string::{string_width_multiline, string_width_multiline_tab};

use crate::{
    grid::{
        config::{Entity, GridConfig},
        grid_projection::GridProjection,
    },
    measurement::Measurement,
    peaker::{Peaker, PriorityNone},
    records::ExactRecords,
    records::{EmptyRecords, Records, RecordsMut},
    table::general::TableDimension,
    width::util::replace_tab,
    CellOption, Table, TableOption, Width,
};

use super::util::{get_table_widths, get_table_widths_with_total, split_at_pos};

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

impl<W> Wrap<W> {
    /// Creates a [`Wrap`] object
    pub fn new(width: W) -> Self
    where
        W: Measurement<Width>,
    {
        Wrap {
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

impl<W, P, R> TableOption<R, TableDimension<'static>> for Wrap<W, P>
where
    W: Measurement<Width>,
    P: Peaker,
    R: Records + ExactRecords + RecordsMut<Text = String>,
    for<'a> &'a R: Records,
{
    fn change(
        &mut self,
        records: &mut R,
        cfg: &mut GridConfig,
        dims: &mut TableDimension<'static>,
    ) {
        if (&*records).count_rows() == 0 || (&*records).count_columns() == 0 {
            return;
        }

        let width = self.width.measure(&*records, cfg);
        let (widths, total) = get_table_widths_with_total(&*records, cfg);
        if width >= total {
            return;
        }

        let priority = P::create();
        let keep_words = self.keep_words;
        let widths = wrap_total_width(records, cfg, widths, total, width, keep_words, priority);

        dims.set_widths(widths);
    }
}

impl<W, R> CellOption<R> for Wrap<W>
where
    W: Measurement<Width>,
    R: Records + ExactRecords + RecordsMut<Text = String>,
    for<'a> &'a R: Records,
{
    fn change(&mut self, records: &mut R, cfg: &mut GridConfig, entity: Entity) {
        let width = self.width.measure(&*records, cfg);

        let count_rows = records.count_rows();
        let count_columns = records.count_columns();

        for pos in entity.iter(count_rows, count_columns) {
            let text = records.get_cell(pos).as_ref();

            let cell_width = string_width_multiline_tab(text, cfg.get_tab_width());
            if cell_width <= width {
                continue;
            }

            // todo: Think about it.
            //       We could eliminate this allocation if we would be allowed to cut '\t' with unknown characters.
            //       Currently we don't do that.
            let text = replace_tab(text, cfg.get_tab_width());
            let wrapped = wrap_text(&text, width, self.keep_words);

            records.set(pos, wrapped);
        }
    }
}

fn wrap_total_width<R, P>(
    records: &mut R,
    cfg: &mut GridConfig,
    mut widths: Vec<usize>,
    total_width: usize,
    width: usize,
    keep_words: bool,
    priority: P,
) -> Vec<usize>
where
    R: Records + ExactRecords + RecordsMut<Text = String>,
    P: Peaker,
    for<'a> &'a R: Records,
{
    let shape = (records.count_rows(), records.count_columns());
    let min_widths = get_table_widths(EmptyRecords::from(shape), cfg);

    decrease_widths(&mut widths, &min_widths, total_width, width, priority);

    let points = get_decrease_cell_list(cfg, &widths, &min_widths, shape);

    let mut wrap = Wrap::new(0);
    wrap.keep_words = keep_words;
    for ((row, col), width) in points {
        wrap.width = width;
        <Wrap as CellOption<R>>::change(&mut wrap, records, cfg, (row, col).into());
    }

    widths
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
    use super::util::strip_osc;

    if width == 0 {
        return String::new();
    }

    let (text, url): (String, Option<String>) = strip_osc(text);
    let (prefix, suffix) = build_link_prefix_suffix(url);

    if keep_words {
        split_keeping_words(&text, width, &prefix, &suffix)
    } else {
        chunks(&text, width, &prefix, &suffix).join("\n")
    }
}

#[cfg(feature = "color")]
fn build_link_prefix_suffix(url: Option<String>) -> (String, String) {
    match url {
        Some(url) => {
            // https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda
            let osc8 = "\x1b]8;;";
            let st = "\x1b\\";

            (format!("{}{}{}", osc8, url, st), format!("{}{}", osc8, st))
        }
        None => ("".to_string(), "".to_string()),
    }
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

        let style = b.style();

        line.push_str(prefix);
        let _ = write!(&mut line, "{}", style.start());

        let mut part = b.text();

        while !part.is_empty() {
            let available_space = width - line_width;

            let part_width = unicode_width::UnicodeWidthStr::width(part);
            if part_width <= available_space {
                line.push_str(part);
                line_width += part_width;

                if available_space == 0 {
                    let _ = write!(&mut line, "{}", style.end());
                    line.push_str(suffix);
                    list.push(line);
                    line = String::with_capacity(width);
                    line.push_str(prefix);
                    line_width = 0;
                    let _ = write!(&mut line, "{}", style.start());
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
                let _ = write!(&mut line, "{}", style.end());
                line.push_str(suffix);
                list.push(line);
                line = String::with_capacity(width);
                line.push_str(prefix);
                line_width = 0;
                let _ = write!(&mut line, "{}", style.start());
            }
        }

        if line_width > 0 {
            let _ = write!(&mut line, "{}", style.end());
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

    use ansi_str::{AnsiBlock, Style};

    if text.is_empty() || width == 0 {
        return String::new();
    }

    let mut buf = String::new();
    let mut line_width = 0;
    let mut word_begin_pos = 0;
    let mut word_length = 0;
    let mut is_empty_buf = true;

    let split = |buf: &mut String, style: &Style| {
        let _ = write!(buf, "{}", style.end());
        buf.push_str(suffix);
        buf.push('\n');
        buf.push_str(prefix);
        let _ = write!(buf, "{}", style.start());
    };

    // go char by char and split string afterwords

    buf.push_str(prefix);

    for block in ansi_str::get_blocks(text) {
        if block.text().is_empty() {
            continue;
        }

        let style = block.style();

        let _ = write!(buf, "{}", style.start());

        for c in block.text().chars() {
            let c_width = unicode_width::UnicodeWidthChar::width(c).unwrap_or(0);
            let is_enough_space = line_width + c_width <= width;

            let is_space = c == ' ';
            if is_space {
                word_length = 0;
                word_begin_pos = 0;

                if !is_enough_space {
                    split(&mut buf, &style);
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

                        let sep = format!("{}{}\n{}{}", style.end(), suffix, prefix, style.start());
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
                        split(&mut buf, &style);
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

        let _ = write!(buf, "{}", style.end());
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
    let (length, count_unknowns, split_char_size) = split_at_pos(text, at);
    let (lhs, rhs) = text.split_at(length);

    (lhs, rhs, (count_unknowns, split_char_size))
}

fn decrease_widths<F>(
    widths: &mut [usize],
    min_widths: &[usize],
    total_width: usize,
    mut width: usize,
    mut peeaker: F,
) where
    F: Peaker,
{
    let mut empty_list = 0;
    for col in 0..widths.len() {
        if widths[col] == 0 || widths[col] <= min_widths[col] {
            empty_list += 1;
        }
    }

    while width != total_width {
        if empty_list == widths.len() {
            break;
        }

        let col = match peeaker.peak(min_widths, widths) {
            Some(col) => col,
            None => break,
        };

        if widths[col] == 0 || widths[col] <= min_widths[col] {
            continue;
        }

        widths[col] -= 1;

        if widths[col] == 0 || widths[col] <= min_widths[col] {
            empty_list += 1;
        }

        width += 1;
    }
}

fn get_decrease_cell_list(
    cfg: &GridConfig,
    widths: &[usize],
    min_widths: &[usize],
    shape: (usize, usize),
) -> Vec<((usize, usize), usize)> {
    let gp = GridProjection::with_shape(cfg, shape);

    let mut points = Vec::new();
    (0..shape.1).for_each(|col| {
        (0..shape.0)
            .filter(|&row| gp.is_cell_visible((row, col)))
            .for_each(|row| {
                let (width, width_min) = match cfg.get_span_column((row, col)) {
                    Some(span) => {
                        let width = (col..col + span).map(|i| widths[i]).sum::<usize>();
                        let min_width = (col..col + span).map(|i| min_widths[i]).sum::<usize>();
                        let count_borders = count_borders(cfg, col, col + span, shape.1);
                        (width + count_borders, min_width + count_borders)
                    }
                    None => (widths[col], min_widths[col]),
                };

                if width >= width_min {
                    let padding = cfg.get_padding((row, col).into());
                    let width = width.saturating_sub(padding.left.size + padding.right.size);

                    points.push(((row, col), width));
                }
            });
    });

    points
}

fn count_borders(cfg: &GridConfig, start: usize, end: usize, count_columns: usize) -> usize {
    let gp = GridProjection::new(cfg).count_columns(count_columns);

    (start..end).skip(1).filter(|&i| gp.has_vertical(i)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

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
