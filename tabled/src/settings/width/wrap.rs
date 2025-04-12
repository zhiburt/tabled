//! This module contains [`Wrap`] structure, used to decrease width of a [`Table`]s or a cell on a [`Table`] by wrapping it's content
//! to a new line.
//!
//! [`Table`]: crate::Table

use crate::{
    grid::{
        config::SpannedConfig,
        config::{ColoredConfig, Entity},
        dimension::CompleteDimensionVecRecords,
        records::{EmptyRecords, ExactRecords, IntoRecords, PeekableRecords, Records, RecordsMut},
        util::string::{get_char_width, get_text_width},
    },
    settings::{
        measurement::Measurement,
        peaker::{Peaker, PriorityNone},
        width::Width,
        CellOption, TableOption,
    },
};

#[cfg(not(feature = "ansi"))]
use crate::grid::util::string::get_string_width;

use super::util::{get_table_widths, get_table_widths_with_total};

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
/// use tabled::{Table, settings::{object::Segment, width::Width, Modify}};
///
/// let table = Table::new(&["Hello World!"])
///     .with(Modify::new(Segment::all()).with(Width::wrap(3)));
/// ```
///
/// [`Padding`]: crate::settings::Padding
#[derive(Debug, Clone)]
pub struct Wrap<W = usize, P = PriorityNone> {
    width: W,
    // TODO: change treatment of space -- if we moving the word we can ignore space I think
    keep_words: bool,
    priority: P,
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
            priority: PriorityNone::new(),
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
    /// [`Padding`]: crate::settings::Padding
    /// [`PriorityMax`]: crate::settings::peaker::PriorityMax
    /// [`PriorityMin`]: crate::settings::peaker::PriorityMin
    pub fn priority<PP: Peaker>(self, priority: PP) -> Wrap<W, PP> {
        Wrap {
            width: self.width,
            keep_words: self.keep_words,
            priority,
        }
    }

    /// Set the keep words option.
    ///
    /// If a wrapping point will be in a word, [`Wrap`] will
    /// preserve a word (if possible) and wrap the string before it.
    pub fn keep_words(mut self, on: bool) -> Self {
        self.keep_words = on;
        self
    }
}

impl Wrap<(), ()> {
    /// Wrap a given string
    pub fn wrap(text: &str, width: usize, keeping_words: bool) -> String {
        wrap_text(text, width, keeping_words)
    }
}

impl<W, P, R> TableOption<R, ColoredConfig, CompleteDimensionVecRecords<'_>> for Wrap<W, P>
where
    W: Measurement<Width>,
    P: Peaker,
    R: Records + ExactRecords + PeekableRecords + RecordsMut<String>,
    for<'a> &'a R: Records,
    for<'a> <<&'a R as Records>::Iter as IntoRecords>::Cell: AsRef<str>,
{
    fn change(
        self,
        records: &mut R,
        cfg: &mut ColoredConfig,
        dims: &mut CompleteDimensionVecRecords<'_>,
    ) {
        if records.count_rows() == 0 || records.count_columns() == 0 {
            return;
        }

        let width = self.width.measure(&*records, cfg);
        let (widths, total) = get_table_widths_with_total(&*records, cfg);
        if width >= total {
            return;
        }

        let priority = self.priority;
        let keep_words = self.keep_words;
        let widths = wrap_total_width(records, cfg, widths, total, width, keep_words, priority);

        dims.set_widths(widths);
    }
}

impl<W, R> CellOption<R, ColoredConfig> for Wrap<W>
where
    W: Measurement<Width>,
    R: Records + ExactRecords + PeekableRecords + RecordsMut<String>,
    for<'a> &'a R: Records,
    for<'a> <<&'a R as Records>::Iter as IntoRecords>::Cell: AsRef<str>,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, entity: Entity) {
        let width = self.width.measure(&*records, cfg);

        let count_rows = records.count_rows();
        let count_columns = records.count_columns();

        for pos in entity.iter(count_rows, count_columns) {
            if !pos.is_covered((count_rows, count_columns).into()) {
                continue;
            }

            let text = records.get_text(pos);
            let cell_width = get_text_width(text);
            if cell_width <= width {
                continue;
            }

            let wrapped = wrap_text(text, width, self.keep_words);
            records.set(pos, wrapped);
        }
    }
}

fn wrap_total_width<R, P>(
    records: &mut R,
    cfg: &mut ColoredConfig,
    mut widths: Vec<usize>,
    total_width: usize,
    width: usize,
    keep_words: bool,
    priority: P,
) -> Vec<usize>
where
    R: Records + ExactRecords + PeekableRecords + RecordsMut<String>,
    P: Peaker,
    for<'a> &'a R: Records,
    for<'a> <<&'a R as Records>::Iter as IntoRecords>::Cell: AsRef<str>,
{
    let shape = (records.count_rows(), records.count_columns());
    let min_widths = get_table_widths(EmptyRecords::from(shape), cfg);

    decrease_widths(&mut widths, &min_widths, total_width, width, priority);

    let points = get_decrease_cell_list(cfg, &widths, &min_widths, shape);

    for ((row, col), width) in points {
        let mut wrap = Wrap::new(width);
        wrap.keep_words = keep_words;
        <Wrap as CellOption<_, _>>::change(wrap, records, cfg, (row, col).into());
    }

    widths
}

pub(crate) fn wrap_text(text: &str, width: usize, keep_words: bool) -> String {
    if width == 0 {
        return String::new();
    }

    #[cfg(not(feature = "ansi"))]
    {
        if keep_words {
            wrap_text_keeping_words(text, width)
        } else {
            wrap_text_basic(text, width)
        }
    }

    #[cfg(feature = "ansi")]
    {
        use crate::util::string::{build_link, strip_osc};

        let (text, url) = strip_osc(text);
        let (prefix, suffix) = build_link(url);

        if keep_words {
            wrap_text_keeping_words(&text, width, &prefix, &suffix)
        } else {
            wrap_text_basic(&text, width, &prefix, &suffix)
        }
    }
}

#[cfg(not(feature = "ansi"))]
fn wrap_text_basic(s: &str, width: usize) -> String {
    const REPLACEMENT: char = '\u{FFFD}';

    if width == 0 {
        return String::new();
    }

    let mut buf = String::new();
    let mut current_width = 0;
    for c in s.chars() {
        if c == '\n' {
            buf.push('\n');
            current_width = 0;
            continue;
        }

        if current_width == width {
            buf.push('\n');
            current_width = 0;
        }

        let char_width = std::cmp::max(1, get_char_width(c));
        let has_line_space = current_width + char_width <= width;
        if !has_line_space {
            let is_char_small = char_width <= width;
            if !is_char_small {
                let count_unknowns = width - current_width;
                buf.extend(std::iter::repeat_n(REPLACEMENT, count_unknowns));
                current_width += count_unknowns;
            } else {
                buf.push('\n');
                buf.push(c);
                current_width = char_width;
            }
        } else {
            buf.push(c);
            current_width += char_width;
        }
    }

    buf
}

#[cfg(feature = "ansi")]
fn wrap_text_basic(text: &str, width: usize, line_prefix: &str, line_suffix: &str) -> String {
    use std::fmt::Write;

    const REPLACEMENT: char = '\u{FFFD}';

    if width == 0 || text.is_empty() {
        return String::new();
    }

    let mut buf = String::with_capacity(width);
    let mut line_width = 0;

    buf.push_str(line_prefix);

    for block in ansi_str::get_blocks(text) {
        let style = block.style();
        let text = block.text();
        if text.is_empty() {
            continue;
        }

        let available = width - line_width;
        if available == 0 {
            buf.push('\n');
            line_width = 0;
        }

        let _ = write!(&mut buf, "{}", style.start());

        // let text_width = get_text_width(text);
        // if text_width + line_width <= width {
        //     buf.push_str(text);
        //     let _ = write!(&mut buf, "{}", style.end());
        //     buf.push_str(line_suffix);
        //     continue;
        // }

        for c in text.chars() {
            if c == '\n' {
                let _ = write!(&mut buf, "{}", style.end());
                buf.push_str(line_suffix);
                buf.push('\n');
                line_width = 0;
                buf.push_str(line_prefix);
                let _ = write!(&mut buf, "{}", style.start());
                continue;
            }

            let char_width = std::cmp::max(1, get_char_width(c));
            let line_has_space = line_width + char_width <= width;
            if line_has_space {
                buf.push(c);
                line_width += char_width;
                continue;
            }

            let is_char_small = char_width <= width;
            if is_char_small {
                let _ = write!(&mut buf, "{}", style.end());
                buf.push_str(line_suffix);
                buf.push('\n');
                line_width = 0;
                buf.push_str(line_prefix);
                let _ = write!(&mut buf, "{}", style.start());

                buf.push(c);
                line_width += char_width;
                continue;
            }

            if line_width == width {
                let _ = write!(&mut buf, "{}", style.end());
                buf.push_str(line_suffix);
                buf.push('\n');
                line_width = 0;
                buf.push_str(line_prefix);
                let _ = write!(&mut buf, "{}", style.start());
            }

            let count_unknowns = width - line_width;
            buf.extend(std::iter::repeat_n(REPLACEMENT, count_unknowns));
            line_width += count_unknowns;
        }

        if line_width > 0 {
            let _ = write!(&mut buf, "{}", style.end());
        }
    }

    if line_width > 0 {
        buf.push_str(line_suffix);
    }

    buf
}

#[cfg(not(feature = "ansi"))]
fn wrap_text_keeping_words(text: &str, width: usize) -> String {
    const REPLACEMENT: char = '\u{FFFD}';

    if width == 0 || text.is_empty() {
        return String::new();
    }

    let mut buf = String::with_capacity(width);
    let mut line_width = 0;

    for word in text.split(' ') {
        // restore space char
        if line_width > 0 {
            let line_has_space = line_width < width;
            if line_has_space {
                buf.push(' ');
                line_width += 1;
            }
        }

        let word_width = get_string_width(word);

        let line_has_space = line_width + word_width <= width;
        if line_has_space {
            buf.push_str(word);
            line_width += word_width;
            continue;
        }

        let is_small_word = word_width <= width;
        if is_small_word {
            buf.push('\n');
            line_width = 0;

            buf.push_str(word);
            line_width += word_width;
            continue;
        }

        for c in word.chars() {
            // take 1 char by 1 and just push it

            let char_width = std::cmp::max(1, get_char_width(c));
            let line_has_space = line_width + char_width <= width;
            if !line_has_space {
                let is_char_small = char_width <= width;
                if !is_char_small {
                    if line_width == width {
                        buf.push('\n');
                        line_width = 0;
                    }

                    // NOTE:
                    // Practically it only can happen if we wrap some late UTF8 symbol.
                    // For example:
                    // Emojie with width 2 but and wrap width 1
                    let available = width - line_width;
                    buf.extend(std::iter::repeat_n(REPLACEMENT, available));
                    line_width = width;
                    continue;
                }

                buf.push('\n');
                line_width = 0;
            }

            buf.push(c);
            line_width += char_width;
        }
    }

    buf
}

#[cfg(feature = "ansi")]
fn wrap_text_keeping_words(text: &str, width: usize, prefix: &str, suffix: &str) -> String {
    if text.is_empty() || width == 0 {
        return String::new();
    }

    parsing::split_text(text, width, prefix, suffix)
}

#[cfg(feature = "ansi")]
mod parsing {
    use super::get_char_width;
    use ansi_str::{get_blocks, AnsiBlock, AnsiBlockIter, Style};
    use std::fmt::Write;

    struct TextBlocks<'a> {
        iter: AnsiBlockIter<'a>,
        current: Option<RelativeBlock<'a>>,
    }

    impl<'a> TextBlocks<'a> {
        fn new(text: &'a str) -> Self {
            Self {
                iter: get_blocks(text),
                current: None,
            }
        }

        fn next(&mut self) -> Option<RelativeBlock<'a>> {
            self.current
                .take()
                .or_else(|| self.iter.next().map(RelativeBlock::new))
        }
    }

    struct RelativeBlock<'a> {
        block: AnsiBlock<'a>,
        pos: usize,
    }

    impl<'a> RelativeBlock<'a> {
        fn new(block: AnsiBlock<'a>) -> Self {
            Self { block, pos: 0 }
        }

        fn get_text(&self) -> &str {
            &self.block.text()[self.pos..]
        }

        fn get_origin(&self) -> &str {
            self.block.text()
        }

        fn get_style(&self) -> &Style {
            self.block.style()
        }
    }

    struct MultilineBuffer<'text, 'color> {
        buf: String,
        width_last: usize,
        width: usize,
        prefix: &'color str,
        suffix: &'color str,
        blocks: TextBlocks<'text>,
    }

    impl<'text, 'color> MultilineBuffer<'text, 'color> {
        fn new(text: &'text str, width: usize, prefix: &'color str, suffix: &'color str) -> Self {
            let blocks = TextBlocks::new(text);

            Self {
                buf: String::new(),
                width_last: 0,
                prefix,
                suffix,
                width,
                blocks,
            }
        }

        fn into_string(self) -> String {
            self.buf
        }

        fn max_width(&self) -> usize {
            self.width
        }

        fn available_width(&self) -> usize {
            self.width - self.width_last
        }

        fn fill(&mut self, c: char) -> usize {
            debug_assert_eq!(get_char_width(c), 1);

            let rest_width = self.available_width();
            for _ in 0..rest_width {
                self.buf.push(c);
            }

            rest_width
        }

        fn set_next_line(&mut self) {
            if let Some(block) = &self.blocks.current {
                let _ = self
                    .buf
                    .write_fmt(format_args!("{}", block.get_style().end()));
            }

            self.buf.push_str(self.suffix);

            self.buf.push('\n');
            self.width_last = 0;

            self.buf.push_str(self.prefix);

            if let Some(block) = &self.blocks.current {
                let _ = self
                    .buf
                    .write_fmt(format_args!("{}", block.get_style().start()));
            }
        }

        fn finish_line(&mut self) {
            if let Some(block) = &self.blocks.current {
                let _ = self
                    .buf
                    .write_fmt(format_args!("{}", block.get_style().end()));
            }

            self.buf.push_str(self.suffix);
            self.width_last = 0;
        }

        fn read_chars(&mut self, block: &RelativeBlock<'_>, n: usize) -> (usize, usize) {
            let mut count_chars = 0;
            let mut count_bytes = 0;
            for c in block.get_text().chars() {
                if count_chars == n {
                    break;
                }

                count_chars += 1;
                count_bytes += c.len_utf8();

                let cwidth = std::cmp::max(1, get_char_width(c));

                let available_space = self.width - self.width_last;
                if available_space == 0 {
                    let _ = self
                        .buf
                        .write_fmt(format_args!("{}", block.get_style().end()));
                    self.buf.push_str(self.suffix);
                    self.buf.push('\n');
                    self.buf.push_str(self.prefix);
                    let _ = self
                        .buf
                        .write_fmt(format_args!("{}", block.get_style().start()));
                    self.width_last = 0;
                }

                let is_enough_space = self.width_last + cwidth <= self.width;
                if !is_enough_space {
                    // thereatically a cwidth can be 2 but buf_width is 1
                    // but it handled here too;

                    const REPLACEMENT: char = '\u{FFFD}';
                    let _ = self.fill(REPLACEMENT);
                    self.width_last = self.width;
                } else {
                    self.buf.push(c);
                    self.width_last += cwidth;
                }
            }

            (count_chars, count_bytes)
        }

        fn read_chars_unchecked(&mut self, block: &RelativeBlock<'_>, n: usize) -> (usize, usize) {
            let mut count_chars = 0;
            let mut count_bytes = 0;
            for c in block.get_text().chars() {
                if count_chars == n {
                    break;
                }

                count_chars += 1;
                count_bytes += c.len_utf8();

                let cwidth = std::cmp::max(1, get_char_width(c));
                self.width_last += cwidth;

                self.buf.push(c);
            }

            debug_assert!(self.width_last <= self.width);

            (count_chars, count_bytes)
        }
    }

    fn read_chars(buf: &mut MultilineBuffer<'_, '_>, n: usize) {
        let mut n = n;
        while n > 0 {
            let is_new_block = buf.blocks.current.is_none();
            let mut block = buf.blocks.next().expect("Must never happen");

            if is_new_block {
                buf.buf.push_str(buf.prefix);
                let _ = buf
                    .buf
                    .write_fmt(format_args!("{}", block.get_style().start()));
            }

            let (read_count, read_bytes) = buf.read_chars(&block, n);

            if block.pos + read_bytes == block.get_origin().len() {
                let _ = buf
                    .buf
                    .write_fmt(format_args!("{}", block.get_style().end()));
            } else {
                block.pos += read_bytes;
                buf.blocks.current = Some(block);
            }

            n -= read_count;
        }
    }

    fn read_chars_unchecked(buf: &mut MultilineBuffer<'_, '_>, n: usize) {
        let mut n = n;
        while n > 0 {
            let is_new_block = buf.blocks.current.is_none();
            let mut block = buf.blocks.next().expect("Must never happen");

            if is_new_block {
                buf.buf.push_str(buf.prefix);
                let _ = buf
                    .buf
                    .write_fmt(format_args!("{}", block.get_style().start()));
            }

            let (read_count, read_bytes) = buf.read_chars_unchecked(&block, n);

            if block.pos + read_bytes == block.get_origin().len() {
                let _ = buf
                    .buf
                    .write_fmt(format_args!("{}", block.get_style().end()));
            } else {
                block.pos += read_bytes;
                buf.blocks.current = Some(block);
            }

            n -= read_count;
        }
    }

    fn handle_word(
        buf: &mut MultilineBuffer<'_, '_>,
        word_chars: usize,
        word_width: usize,
        additional_read: usize,
    ) {
        if word_chars > 0 {
            let has_line_space = word_width <= buf.available_width();
            let is_word_too_big = word_width > buf.max_width();

            if is_word_too_big {
                read_chars(buf, word_chars + additional_read);
            } else if has_line_space {
                read_chars_unchecked(buf, word_chars);
                if additional_read > 0 {
                    read_chars(buf, additional_read);
                }
            } else {
                buf.set_next_line();
                read_chars_unchecked(buf, word_chars);
                if additional_read > 0 {
                    read_chars(buf, additional_read);
                }
            }

            return;
        }

        let has_current_line_space = additional_read <= buf.available_width();
        if has_current_line_space {
            read_chars_unchecked(buf, additional_read);
        } else {
            buf.set_next_line();
            read_chars_unchecked(buf, additional_read);
        }
    }

    pub(super) fn split_text(text: &str, width: usize, prefix: &str, suffix: &str) -> String {
        let mut word_width = 0;
        let mut word_chars = 0;
        let mut buf = MultilineBuffer::new(text, width, prefix, suffix);

        let stripped_text = ansi_str::AnsiStr::ansi_strip(text);
        for c in stripped_text.chars() {
            match c {
                ' ' => {
                    handle_word(&mut buf, word_chars, word_width, 1);
                    word_chars = 0;
                    word_width = 0;
                }
                '\n' => {
                    handle_word(&mut buf, word_chars, word_width, 1);
                    word_chars = 0;
                    word_width = 0;
                }
                _ => {
                    word_width += std::cmp::max(1, get_char_width(c));
                    word_chars += 1;
                }
            }
        }

        if word_chars > 0 {
            handle_word(&mut buf, word_chars, word_width, 0);
            buf.finish_line();
        }

        buf.into_string()
    }
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
    cfg: &SpannedConfig,
    widths: &[usize],
    min_widths: &[usize],
    shape: (usize, usize),
) -> Vec<((usize, usize), usize)> {
    let mut points = Vec::new();
    (0..shape.1).for_each(|col| {
        (0..shape.0)
            .filter(|&row| cfg.is_cell_visible((row, col).into()))
            .for_each(|row| {
                let (width, width_min) = match cfg.get_column_span((row, col).into()) {
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

fn count_borders(cfg: &SpannedConfig, start: usize, end: usize, count_columns: usize) -> usize {
    (start..end)
        .skip(1)
        .filter(|&i| cfg.has_vertical(i, count_columns))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_test() {
        #[cfg(not(feature = "ansi"))]
        let split = |text, width| wrap_text_basic(text, width);

        #[cfg(feature = "ansi")]
        let split = |text, width| wrap_text_basic(text, width, "", "");

        assert_eq!(split("123456", 0), "");

        assert_eq!(split("123456", 1), "1\n2\n3\n4\n5\n6");
        assert_eq!(split("123456", 2), "12\n34\n56");
        assert_eq!(split("12345", 2), "12\n34\n5");
        assert_eq!(split("123456", 6), "123456");
        assert_eq!(split("123456", 10), "123456");

        assert_eq!(split("😳😳😳😳😳", 1), "�\n�\n�\n�\n�");
        assert_eq!(split("😳😳😳😳😳", 2), "😳\n😳\n😳\n😳\n😳");
        assert_eq!(split("😳😳😳😳😳", 3), "😳\n😳\n😳\n😳\n😳");
        assert_eq!(split("😳😳😳😳😳", 6), "😳😳😳\n😳😳");
        assert_eq!(split("😳😳😳😳😳", 20), "😳😳😳😳😳");

        assert_eq!(split("😳123😳", 1), "�\n1\n2\n3\n�");
        assert_eq!(split("😳12😳3", 1), "�\n1\n2\n�\n3");
    }

    #[test]
    fn chunks_test() {
        #[allow(clippy::redundant_closure)]
        #[cfg(not(feature = "ansi"))]
        let chunks = |text, width| wrap_text_basic(text, width);

        #[cfg(feature = "ansi")]
        let chunks = |text, width| wrap_text_basic(text, width, "", "");

        assert_eq!(chunks("123456", 0), "");

        assert_eq!(
            chunks("123456", 1),
            ["1", "2", "3", "4", "5", "6"].join("\n")
        );
        assert_eq!(chunks("123456", 2), ["12", "34", "56"].join("\n"));
        assert_eq!(chunks("12345", 2), ["12", "34", "5"].join("\n"));

        assert_eq!(
            chunks("😳😳😳😳😳", 1),
            ["�", "�", "�", "�", "�"].join("\n")
        );
        assert_eq!(
            chunks("😳😳😳😳😳", 2),
            ["😳", "😳", "😳", "😳", "😳"].join("\n")
        );
        assert_eq!(
            chunks("😳😳😳😳😳", 3),
            ["😳", "😳", "😳", "😳", "😳"].join("\n")
        );
    }

    #[cfg(not(feature = "ansi"))]
    #[test]
    fn split_by_line_keeping_words_test() {
        let split_keeping_words = |text, width| wrap_text_keeping_words(text, width);

        assert_eq!(split_keeping_words("123456", 1), "1\n2\n3\n4\n5\n6");
        assert_eq!(split_keeping_words("123456", 2), "12\n34\n56");
        assert_eq!(split_keeping_words("12345", 2), "12\n34\n5");

        assert_eq!(split_keeping_words("😳😳😳😳😳", 1), "�\n�\n�\n�\n�");

        assert_eq!(split_keeping_words("111 234 1", 4), "111 \n234 \n1");
    }

    #[cfg(feature = "ansi")]
    #[test]
    fn split_by_line_keeping_words_test() {
        #[cfg(feature = "ansi")]
        let split_keeping_words = |text, width| wrap_text_keeping_words(text, width, "", "");

        assert_eq!(split_keeping_words("123456", 1), "1\n2\n3\n4\n5\n6");
        assert_eq!(split_keeping_words("123456", 2), "12\n34\n56");
        assert_eq!(split_keeping_words("12345", 2), "12\n34\n5");

        assert_eq!(split_keeping_words("😳😳😳😳😳", 1), "�\n�\n�\n�\n�");

        assert_eq!(split_keeping_words("111 234 1", 4), "111 \n234 \n1");
    }

    #[cfg(feature = "ansi")]
    #[test]
    fn split_by_line_keeping_words_color_test() {
        #[cfg(feature = "ansi")]
        let split_keeping_words = |text, width| wrap_text_keeping_words(text, width, "", "");

        #[cfg(not(feature = "ansi"))]
        let split_keeping_words = |text, width| split_keeping_words(text, width, "\n");

        let text = "\u{1b}[36mJapanese “vacancy” button\u{1b}[0m";

        assert_eq!(split_keeping_words(text, 2), "\u{1b}[36mJa\u{1b}[39m\n\u{1b}[36mpa\u{1b}[39m\n\u{1b}[36mne\u{1b}[39m\n\u{1b}[36mse\u{1b}[39m\n\u{1b}[36m “\u{1b}[39m\n\u{1b}[36mva\u{1b}[39m\n\u{1b}[36mca\u{1b}[39m\n\u{1b}[36mnc\u{1b}[39m\n\u{1b}[36my”\u{1b}[39m\n\u{1b}[36m b\u{1b}[39m\n\u{1b}[36mut\u{1b}[39m\n\u{1b}[36mto\u{1b}[39m\n\u{1b}[36mn\u{1b}[39m");
        assert_eq!(split_keeping_words(text, 1), "\u{1b}[36mJ\u{1b}[39m\n\u{1b}[36ma\u{1b}[39m\n\u{1b}[36mp\u{1b}[39m\n\u{1b}[36ma\u{1b}[39m\n\u{1b}[36mn\u{1b}[39m\n\u{1b}[36me\u{1b}[39m\n\u{1b}[36ms\u{1b}[39m\n\u{1b}[36me\u{1b}[39m\n\u{1b}[36m \u{1b}[39m\n\u{1b}[36m“\u{1b}[39m\n\u{1b}[36mv\u{1b}[39m\n\u{1b}[36ma\u{1b}[39m\n\u{1b}[36mc\u{1b}[39m\n\u{1b}[36ma\u{1b}[39m\n\u{1b}[36mn\u{1b}[39m\n\u{1b}[36mc\u{1b}[39m\n\u{1b}[36my\u{1b}[39m\n\u{1b}[36m”\u{1b}[39m\n\u{1b}[36m \u{1b}[39m\n\u{1b}[36mb\u{1b}[39m\n\u{1b}[36mu\u{1b}[39m\n\u{1b}[36mt\u{1b}[39m\n\u{1b}[36mt\u{1b}[39m\n\u{1b}[36mo\u{1b}[39m\n\u{1b}[36mn\u{1b}[39m");
    }

    #[cfg(feature = "ansi")]
    #[test]
    fn split_by_line_keeping_words_color_2_test() {
        use ansi_str::AnsiStr;

        #[cfg(feature = "ansi")]
        let split_keeping_words = |text, width| wrap_text_keeping_words(text, width, "", "");

        #[cfg(not(feature = "ansi"))]
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
                "\u{1b}[37m A\u{1b}[39m",
                "\u{1b}[37mnd\u{1b}[39m",
                "\u{1b}[37min\u{1b}[39m",
                "\u{1b}[37ma \u{1b}[39m",
                "\u{1b}[37m  \u{1b}[39m",
                "\u{1b}[37m  \u{1b}[39m",
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

    #[cfg(feature = "ansi")]
    #[test]
    fn split_by_line_keeping_words_color_3_test() {
        let split = |text, width| wrap_text_keeping_words(text, width, "", "");
        assert_eq!(
            split(
                "\u{1b}[37m🚵🏻🚵🏻🚵🏻🚵🏻🚵🏻🚵🏻🚵🏻🚵🏻🚵🏻🚵🏻\u{1b}[0m",
                3,
            ),
            "\u{1b}[37m🚵�\u{1b}[39m\n\u{1b}[37m🚵�\u{1b}[39m\n\u{1b}[37m🚵�\u{1b}[39m\n\u{1b}[37m🚵�\u{1b}[39m\n\u{1b}[37m🚵�\u{1b}[39m\n\u{1b}[37m🚵�\u{1b}[39m\n\u{1b}[37m🚵�\u{1b}[39m\n\u{1b}[37m🚵�\u{1b}[39m\n\u{1b}[37m🚵�\u{1b}[39m\n\u{1b}[37m🚵�\u{1b}[39m",
        );
        assert_eq!(
            split("\u{1b}[37mthis is a long sentence\u{1b}[0m", 7),
            "\u{1b}[37mthis is\u{1b}[39m\n\u{1b}[37m a long\u{1b}[39m\n\u{1b}[37m senten\u{1b}[39m\n\u{1b}[37mce\u{1b}[39m"
        );
        assert_eq!(
            split("\u{1b}[37mHello World\u{1b}[0m", 7),
            "\u{1b}[37mHello \u{1b}[39m\n\u{1b}[37mWorld\u{1b}[39m"
        );
        assert_eq!(
            split("\u{1b}[37mHello Wo\u{1b}[37mrld\u{1b}[0m", 7),
            "\u{1b}[37mHello \u{1b}[39m\n\u{1b}[37mWo\u{1b}[39m\u{1b}[37mrld\u{1b}[39m"
        );
        assert_eq!(
            split("\u{1b}[37mHello Wo\u{1b}[37mrld\u{1b}[0m", 8),
            "\u{1b}[37mHello \u{1b}[39m\n\u{1b}[37mWo\u{1b}[39m\u{1b}[37mrld\u{1b}[39m"
        );
    }

    #[cfg(not(feature = "ansi"))]
    #[test]
    fn split_keeping_words_4_test() {
        let split_keeping_words = |text, width| wrap_text_keeping_words(text, width);

        assert_eq!(split_keeping_words("12345678", 3,), "123\n456\n78");
        assert_eq!(split_keeping_words("12345678", 2,), "12\n34\n56\n78");
    }

    #[cfg(feature = "ansi")]
    #[test]
    fn split_keeping_words_4_test() {
        let split_keeping_words = |text, width| wrap_text_keeping_words(text, width, "", "");

        #[cfg(not(feature = "ansi"))]
        let split_keeping_words = |text, width| split_keeping_words(text, width, "\n");

        assert_eq!(split_keeping_words("12345678", 3,), "123\n456\n78");
        assert_eq!(split_keeping_words("12345678", 2,), "12\n34\n56\n78");
    }

    #[cfg(feature = "ansi")]
    #[test]
    fn chunks_test_with_prefix_and_suffix() {
        assert_eq!(wrap_text_basic("123456", 0, "^", "$"), ["^$"; 0].join("\n"));

        assert_eq!(
            wrap_text_basic("123456", 1, "^", "$"),
            ["^1$", "^2$", "^3$", "^4$", "^5$", "^6$"].join("\n")
        );
        assert_eq!(
            wrap_text_basic("123456", 2, "^", "$"),
            ["^12$", "^34$", "^56$"].join("\n")
        );
        assert_eq!(
            wrap_text_basic("12345", 2, "^", "$"),
            ["^12$", "^34$", "^5$"].join("\n")
        );

        assert_eq!(
            wrap_text_basic("😳😳😳😳😳", 1, "^", "$"),
            ["^�$", "^�$", "^�$", "^�$", "^�$"].join("\n")
        );
        assert_eq!(
            wrap_text_basic("😳😳😳😳😳", 2, "^", "$"),
            ["^😳$", "^😳$", "^😳$", "^😳$", "^😳$"].join("\n")
        );
        assert_eq!(
            wrap_text_basic("😳😳😳😳😳", 3, "^", "$"),
            "^😳$\n^😳$\n^😳$\n^😳$\n^😳$"
        );
    }

    #[cfg(feature = "ansi")]
    #[test]
    fn split_by_line_keeping_words_test_with_prefix_and_suffix() {
        assert_eq!(
            wrap_text_keeping_words("123456", 1, "^", "$"),
            "^1$\n^2$\n^3$\n^4$\n^5$\n^6$"
        );
        assert_eq!(
            wrap_text_keeping_words("123456", 2, "^", "$"),
            "^12$\n^34$\n^56$"
        );
        assert_eq!(
            wrap_text_keeping_words("12345", 2, "^", "$"),
            "^12$\n^34$\n^5$"
        );

        assert_eq!(
            wrap_text_keeping_words("😳😳😳😳😳", 1, "^", "$"),
            "^�$\n^�$\n^�$\n^�$\n^�$"
        );
    }

    #[cfg(feature = "ansi")]
    #[test]
    fn split_by_line_keeping_words_color_2_test_with_prefix_and_suffix() {
        use ansi_str::AnsiStr;

        let text = "\u{1b}[37mTigre Ecuador   OMYA Andina     3824909999      Calcium carbonate       Colombia\u{1b}[0m";

        assert_eq!(
            wrap_text_keeping_words(text, 2, "^", "$")
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
                "^\u{1b}[37m A\u{1b}[39m$",
                "^\u{1b}[37mnd\u{1b}[39m$",
                "^\u{1b}[37min\u{1b}[39m$",
                "^\u{1b}[37ma \u{1b}[39m$",
                "^\u{1b}[37m  \u{1b}[39m$",
                "^\u{1b}[37m  \u{1b}[39m$",
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
            wrap_text_keeping_words(text, 1, "^", "$")
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

    #[cfg(feature = "ansi")]
    #[test]
    fn chunks_wrap_2() {
        let text = "\u{1b}[30mDebian\u{1b}[0m\u{1b}[31mDebian\u{1b}[0m\u{1b}[32mDebian\u{1b}[0m\u{1b}[33mDebian\u{1b}[0m\u{1b}[34mDebian\u{1b}[0m\u{1b}[35mDebian\u{1b}[0m\u{1b}[36mDebian\u{1b}[0m\u{1b}[37mDebian\u{1b}[0m\u{1b}[40mDebian\u{1b}[0m\u{1b}[41mDebian\u{1b}[0m\u{1b}[42mDebian\u{1b}[0m\u{1b}[43mDebian\u{1b}[0m\u{1b}[44mDebian\u{1b}[0m";
        assert_eq!(
            wrap_text_basic(text, 30, "", ""),
            [
                "\u{1b}[30mDebian\u{1b}[39m\u{1b}[31mDebian\u{1b}[39m\u{1b}[32mDebian\u{1b}[39m\u{1b}[33mDebian\u{1b}[39m\u{1b}[34mDebian\u{1b}[39m",
                "\u{1b}[35mDebian\u{1b}[39m\u{1b}[36mDebian\u{1b}[39m\u{1b}[37mDebian\u{1b}[39m\u{1b}[40mDebian\u{1b}[49m\u{1b}[41mDebian\u{1b}[49m",
                "\u{1b}[42mDebian\u{1b}[49m\u{1b}[43mDebian\u{1b}[49m\u{1b}[44mDebian\u{1b}[49m",
            ].join("\n")
        );
    }

    #[cfg(feature = "ansi")]
    #[test]
    fn chunks_wrap_3() {
        let text = "\u{1b}[37mCreate bytes from the \u{1b}[0m\u{1b}[7;34marg\u{1b}[0m\u{1b}[37muments.\u{1b}[0m";

        assert_eq!(
            wrap_text_basic(text, 22, "", ""),
            [
                "\u{1b}[37mCreate bytes from the \u{1b}[39m",
                "\u{1b}[7m\u{1b}[34marg\u{1b}[27m\u{1b}[39m\u{1b}[37muments.\u{1b}[39m"
            ]
            .join("\n")
        );
    }

    #[cfg(feature = "ansi")]
    #[test]
    fn chunks_wrap_3_keeping_words() {
        let text = "\u{1b}[37mCreate bytes from the \u{1b}[0m\u{1b}[7;34marg\u{1b}[0m\u{1b}[37muments.\u{1b}[0m";

        assert_eq!(
            wrap_text_keeping_words(text, 22, "", ""),
            "\u{1b}[37mCreate bytes from the \u{1b}[39m\n\u{1b}[7m\u{1b}[34marg\u{1b}[27m\u{1b}[39m\u{1b}[37muments.\u{1b}[39m"
        );
    }

    #[cfg(feature = "ansi")]
    #[test]
    fn chunks_wrap_4() {
        let text = "\u{1b}[37mReturns the floor of a number (l\u{1b}[0m\u{1b}[41;37marg\u{1b}[0m\u{1b}[37mest integer less than or equal to that number).\u{1b}[0m";

        assert_eq!(
            wrap_text_basic(text, 10, "", ""),
            [
                "\u{1b}[37mReturns th\u{1b}[39m",
                "\u{1b}[37me floor of\u{1b}[39m",
                "\u{1b}[37m a number \u{1b}[39m",
                "\u{1b}[37m(l\u{1b}[39m\u{1b}[37m\u{1b}[41marg\u{1b}[39m\u{1b}[49m\u{1b}[37mest i\u{1b}[39m",
                "\u{1b}[37mnteger les\u{1b}[39m",
                "\u{1b}[37ms than or \u{1b}[39m",
                "\u{1b}[37mequal to t\u{1b}[39m",
                "\u{1b}[37mhat number\u{1b}[39m",
                "\u{1b}[37m).\u{1b}[39m",
            ].join("\n")
        );
    }

    #[cfg(feature = "ansi")]
    #[test]
    fn chunks_wrap_4_keeping_words() {
        let text = "\u{1b}[37mReturns the floor of a number (l\u{1b}[0m\u{1b}[41;37marg\u{1b}[0m\u{1b}[37mest integer less than or equal to that number).\u{1b}[0m";
        assert_eq!(
            wrap_text_keeping_words(text, 10, "", ""),
            concat!(
                "\u{1b}[37mReturns \u{1b}[39m\n",
                "\u{1b}[37mthe floor \u{1b}[39m\n",
                "\u{1b}[37mof a \u{1b}[39m\n",
                "\u{1b}[37mnumber \u{1b}[39m\n",
                "\u{1b}[37m(l\u{1b}[39m\u{1b}[37m\u{1b}[41marg\u{1b}[39m\u{1b}[49m\u{1b}[37mest \u{1b}[39m\n",
                "\u{1b}[37minteger \u{1b}[39m\n",
                "\u{1b}[37mless than \u{1b}[39m\n",
                "\u{1b}[37mor equal \u{1b}[39m\n",
                "\u{1b}[37mto that \u{1b}[39m\n",
                "\u{1b}[37mnumber).\u{1b}[39m",
            )
        );
    }

    #[cfg(not(feature = "ansi"))]
    #[test]
    fn chunks_chinese_0() {
        let text = "(公司{ 名称:\"腾讯科技（深圳）有限公司\",成立时间:\"1998年11月\"}";

        assert_eq!(
            wrap_text_basic(text, 40),
            concat!(
                "(公司{ 名称:\"腾讯科技（深圳）有限公司\",\n",
                "成立时间:\"1998年11月\"}",
            ),
        );
    }

    #[cfg(not(feature = "ansi"))]
    #[test]
    fn chunks_keeping_chinese_0() {
        let text = "(公司{ 名称:\"腾讯科技（深圳）有限公司\",成立时间:\"1998年11月\"}";

        assert_eq!(
            wrap_text_keeping_words(text, 40),
            concat!(
                "(公司{ 名称:\"腾讯科技（深圳）有限公司\",\n",
                "成立时间:\"1998年11月\"}",
            ),
        );
    }
}
