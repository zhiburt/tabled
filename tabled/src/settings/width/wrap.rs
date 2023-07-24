//! This module contains [`Wrap`] structure, used to decrease width of a [`Table`]s or a cell on a [`Table`] by wrapping it's content
//! to a new line.
//!
//! [`Table`]: crate::Table

use std::marker::PhantomData;

use crate::{
    grid::config::ColoredConfig,
    grid::dimension::CompleteDimensionVecRecords,
    grid::records::{EmptyRecords, ExactRecords, PeekableRecords, Records, RecordsMut},
    grid::{config::Entity, config::SpannedConfig, util::string::string_width_multiline},
    settings::{
        measurement::Measurement,
        peaker::{Peaker, PriorityNone},
        width::Width,
        CellOption, TableOption,
    },
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
            _priority: PhantomData,
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
    pub fn priority<PP>(self) -> Wrap<W, PP> {
        Wrap {
            width: self.width,
            keep_words: self.keep_words,
            _priority: PhantomData,
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

impl Wrap<(), ()> {
    /// Wrap a given string
    pub fn wrap_text(text: &str, width: usize, keeping_words: bool) -> String {
        wrap_text(text, width, keeping_words)
    }
}

impl<W, P, R> TableOption<R, CompleteDimensionVecRecords<'static>, ColoredConfig> for Wrap<W, P>
where
    W: Measurement<Width>,
    P: Peaker,
    R: Records + ExactRecords + PeekableRecords + RecordsMut<String>,
    for<'a> &'a R: Records,
{
    fn change(
        self,
        records: &mut R,
        cfg: &mut ColoredConfig,
        dims: &mut CompleteDimensionVecRecords<'static>,
    ) {
        if records.count_rows() == 0 || records.count_columns() == 0 {
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

        let _ = dims.set_widths(widths);
    }
}

impl<W, R> CellOption<R, ColoredConfig> for Wrap<W>
where
    W: Measurement<Width>,
    R: Records + ExactRecords + PeekableRecords + RecordsMut<String>,
    for<'a> &'a R: Records,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, entity: Entity) {
        let width = self.width.measure(&*records, cfg);

        let count_rows = records.count_rows();
        let count_columns = records.count_columns();

        for pos in entity.iter(count_rows, count_columns) {
            let is_valid_pos = pos.0 < records.count_rows() && pos.1 < records.count_columns();
            if !is_valid_pos {
                continue;
            }

            let text = records.get_text(pos);
            let cell_width = string_width_multiline(text);
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

            (format!("{osc8}{url}{st}"), format!("{osc8}{st}"))
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
        let c_width = unicode_width::UnicodeWidthChar::width(c).unwrap_or_default();
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
        let text_style = b.style();
        let mut text_slice = b.text();
        if text_slice.is_empty() {
            continue;
        }

        let available_space = width - line_width;
        if available_space == 0 {
            list.push(line);
            line = String::with_capacity(width);
            line_width = 0;
        }

        line.push_str(prefix);
        let _ = write!(&mut line, "{}", text_style.start());

        while !text_slice.is_empty() {
            let available_space = width - line_width;

            let part_width = unicode_width::UnicodeWidthStr::width(text_slice);
            if part_width <= available_space {
                line.push_str(text_slice);
                line_width += part_width;

                if available_space == 0 {
                    let _ = write!(&mut line, "{}", text_style.end());
                    line.push_str(suffix);
                    list.push(line);
                    line = String::with_capacity(width);
                    line.push_str(prefix);
                    line_width = 0;
                    let _ = write!(&mut line, "{}", text_style.start());
                }

                break;
            }

            let (lhs, rhs, (unknowns, split_char)) = split_string_at(text_slice, available_space);

            text_slice = &rhs[split_char..];

            line.push_str(lhs);
            line_width += unicode_width::UnicodeWidthStr::width(lhs);

            const REPLACEMENT: char = '\u{FFFD}';
            line.extend(std::iter::repeat(REPLACEMENT).take(unknowns));
            line_width += unknowns;

            if line_width == width {
                let _ = write!(&mut line, "{}", text_style.end());
                line.push_str(suffix);
                list.push(line);
                line = String::with_capacity(width);
                line.push_str(prefix);
                line_width = 0;
                let _ = write!(&mut line, "{}", text_style.start());
            }
        }

        if line_width > 0 {
            let _ = write!(&mut line, "{}", text_style.end());
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
                is_first_word = false;

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
    if text.is_empty() || width == 0 {
        return String::new();
    }

    let stripped_text = ansi_str::AnsiStr::ansi_strip(text);
    let mut word_width = 0;
    let mut word_chars = 0;
    let mut blocks = parsing::Blocks::new(ansi_str::get_blocks(text));
    let mut buf = parsing::MultilineBuffer::new(width);
    buf.set_prefix(prefix);
    buf.set_suffix(suffix);

    for c in stripped_text.chars() {
        match c {
            ' ' => {
                parsing::handle_word(&mut buf, &mut blocks, word_chars, word_width, 1);
                word_chars = 0;
                word_width = 0;
            }
            '\n' => {
                parsing::handle_word(&mut buf, &mut blocks, word_chars, word_width, 1);
                word_chars = 0;
                word_width = 0;
            }
            _ => {
                word_width += unicode_width::UnicodeWidthChar::width(c).unwrap_or(0);
                word_chars += 1;
            }
        }
    }

    if word_chars > 0 {
        parsing::handle_word(&mut buf, &mut blocks, word_chars, word_width, 0);
        buf.finish_line(&blocks);
    }

    buf.into_string()
}

#[cfg(feature = "color")]
mod parsing {
    use ansi_str::{AnsiBlock, AnsiBlockIter, Style};
    use std::fmt::Write;

    pub(super) struct Blocks<'a> {
        iter: AnsiBlockIter<'a>,
        current: Option<RelativeBlock<'a>>,
    }

    impl<'a> Blocks<'a> {
        pub(super) fn new(iter: AnsiBlockIter<'a>) -> Self {
            Self {
                iter,
                current: None,
            }
        }

        pub(super) fn next_block(&mut self) -> Option<RelativeBlock<'a>> {
            self.current
                .take()
                .or_else(|| self.iter.next().map(RelativeBlock::new))
        }
    }

    pub(super) struct RelativeBlock<'a> {
        block: AnsiBlock<'a>,
        pos: usize,
    }

    impl<'a> RelativeBlock<'a> {
        pub(super) fn new(block: AnsiBlock<'a>) -> Self {
            Self { block, pos: 0 }
        }

        pub(super) fn get_text(&self) -> &str {
            &self.block.text()[self.pos..]
        }

        pub(super) fn get_origin(&self) -> &str {
            self.block.text()
        }

        pub(super) fn get_style(&self) -> &Style {
            self.block.style()
        }
    }

    pub(super) struct MultilineBuffer<'a> {
        buf: String,
        width_last: usize,
        width: usize,
        prefix: &'a str,
        suffix: &'a str,
    }

    impl<'a> MultilineBuffer<'a> {
        pub(super) fn new(width: usize) -> Self {
            Self {
                buf: String::new(),
                width_last: 0,
                prefix: "",
                suffix: "",
                width,
            }
        }

        pub(super) fn into_string(self) -> String {
            self.buf
        }

        pub(super) fn set_suffix(&mut self, suffix: &'a str) {
            self.suffix = suffix;
        }

        pub(super) fn set_prefix(&mut self, prefix: &'a str) {
            self.prefix = prefix;
        }

        pub(super) fn max_width(&self) -> usize {
            self.width
        }

        pub(super) fn available_width(&self) -> usize {
            self.width - self.width_last
        }

        pub(super) fn fill(&mut self, c: char) -> usize {
            debug_assert_eq!(unicode_width::UnicodeWidthChar::width(c), Some(1));

            let rest_width = self.available_width();
            for _ in 0..rest_width {
                self.buf.push(c);
            }

            rest_width
        }

        pub(super) fn set_next_line(&mut self, blocks: &Blocks<'_>) {
            if let Some(block) = &blocks.current {
                let _ = self
                    .buf
                    .write_fmt(format_args!("{}", block.get_style().end()));
            }

            self.buf.push_str(self.suffix);

            let _ = self.fill(' ');
            self.buf.push('\n');
            self.width_last = 0;

            self.buf.push_str(self.prefix);

            if let Some(block) = &blocks.current {
                let _ = self
                    .buf
                    .write_fmt(format_args!("{}", block.get_style().start()));
            }
        }

        pub(super) fn finish_line(&mut self, blocks: &Blocks<'_>) {
            if let Some(block) = &blocks.current {
                let _ = self
                    .buf
                    .write_fmt(format_args!("{}", block.get_style().end()));
            }

            self.buf.push_str(self.suffix);

            let _ = self.fill(' ');
            self.width_last = 0;
        }

        pub(super) fn read_chars(&mut self, block: &RelativeBlock<'_>, n: usize) -> (usize, usize) {
            let mut count_chars = 0;
            let mut count_bytes = 0;
            for c in block.get_text().chars() {
                if count_chars == n {
                    break;
                }

                count_chars += 1;
                count_bytes += c.len_utf8();

                let cwidth = unicode_width::UnicodeWidthChar::width(c).unwrap_or(0);

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

        pub(super) fn read_chars_unchecked(
            &mut self,
            block: &RelativeBlock<'_>,
            n: usize,
        ) -> (usize, usize) {
            let mut count_chars = 0;
            let mut count_bytes = 0;
            for c in block.get_text().chars() {
                if count_chars == n {
                    break;
                }

                count_chars += 1;
                count_bytes += c.len_utf8();

                let cwidth = unicode_width::UnicodeWidthChar::width(c).unwrap_or(0);
                self.width_last += cwidth;

                self.buf.push(c);
            }

            debug_assert!(self.width_last <= self.width);

            (count_chars, count_bytes)
        }
    }

    pub(super) fn read_chars(buf: &mut MultilineBuffer<'_>, blocks: &mut Blocks<'_>, n: usize) {
        let mut n = n;
        while n > 0 {
            let is_new_block = blocks.current.is_none();
            let mut block = blocks.next_block().expect("Must never happen");
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
                blocks.current = Some(block);
            }

            n -= read_count;
        }
    }

    pub(super) fn read_chars_unchecked(
        buf: &mut MultilineBuffer<'_>,
        blocks: &mut Blocks<'_>,
        n: usize,
    ) {
        let mut n = n;
        while n > 0 {
            let is_new_block = blocks.current.is_none();
            let mut block = blocks.next_block().expect("Must never happen");

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
                blocks.current = Some(block);
            }

            n -= read_count;
        }
    }

    pub(super) fn handle_word(
        buf: &mut MultilineBuffer<'_>,
        blocks: &mut Blocks<'_>,
        word_chars: usize,
        word_width: usize,
        additional_read: usize,
    ) {
        if word_chars > 0 {
            let has_line_space = word_width <= buf.available_width();
            let is_word_too_big = word_width > buf.max_width();

            if is_word_too_big {
                read_chars(buf, blocks, word_chars + additional_read);
            } else if has_line_space {
                read_chars_unchecked(buf, blocks, word_chars);
                if additional_read > 0 {
                    read_chars(buf, blocks, additional_read);
                }
            } else {
                buf.set_next_line(&*blocks);
                read_chars_unchecked(buf, blocks, word_chars);
                if additional_read > 0 {
                    read_chars(buf, blocks, additional_read);
                }
            }

            return;
        }

        let has_current_line_space = additional_read <= buf.available_width();
        if has_current_line_space {
            read_chars_unchecked(buf, blocks, additional_read);
        } else {
            buf.set_next_line(&*blocks);
            read_chars_unchecked(buf, blocks, additional_read);
        }
    }
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
    cfg: &SpannedConfig,
    widths: &[usize],
    min_widths: &[usize],
    shape: (usize, usize),
) -> Vec<((usize, usize), usize)> {
    let mut points = Vec::new();
    (0..shape.1).for_each(|col| {
        (0..shape.0)
            .filter(|&row| cfg.is_cell_visible((row, col)))
            .for_each(|row| {
                let (width, width_min) = match cfg.get_column_span((row, col)) {
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
        #[allow(clippy::redundant_closure)]
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

        assert_eq!(split_keeping_words(text, 2), "\u{1b}[36mJa\u{1b}[39m\n\u{1b}[36mpa\u{1b}[39m\n\u{1b}[36mne\u{1b}[39m\n\u{1b}[36mse\u{1b}[39m\n\u{1b}[36m “\u{1b}[39m\n\u{1b}[36mva\u{1b}[39m\n\u{1b}[36mca\u{1b}[39m\n\u{1b}[36mnc\u{1b}[39m\n\u{1b}[36my”\u{1b}[39m\n\u{1b}[36m b\u{1b}[39m\n\u{1b}[36mut\u{1b}[39m\n\u{1b}[36mto\u{1b}[39m\n\u{1b}[36mn\u{1b}[39m ");
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

    #[cfg(feature = "color")]
    #[test]
    fn split_by_line_keeping_words_color_3_test() {
        let split = |text, width| split_keeping_words(text, width, "", "");
        assert_eq!(
            split(
                "\u{1b}[37m🚵🏻🚵🏻🚵🏻🚵🏻🚵🏻🚵🏻🚵🏻🚵🏻🚵🏻🚵🏻\u{1b}[0m",
                3,
            ),
            "\u{1b}[37m🚵�\u{1b}[39m\n\u{1b}[37m🚵�\u{1b}[39m\n\u{1b}[37m🚵�\u{1b}[39m\n\u{1b}[37m🚵�\u{1b}[39m\n\u{1b}[37m🚵�\u{1b}[39m\n\u{1b}[37m🚵�\u{1b}[39m\n\u{1b}[37m🚵�\u{1b}[39m\n\u{1b}[37m🚵�\u{1b}[39m\n\u{1b}[37m🚵�\u{1b}[39m\n\u{1b}[37m🚵�\u{1b}[39m",
        );
        assert_eq!(
            split("\u{1b}[37mthis is a long sentence\u{1b}[0m", 7),
            "\u{1b}[37mthis is\u{1b}[39m\n\u{1b}[37m a long\u{1b}[39m\n\u{1b}[37m senten\u{1b}[39m\n\u{1b}[37mce\u{1b}[39m     "
        );
        assert_eq!(
            split("\u{1b}[37mHello World\u{1b}[0m", 7),
            "\u{1b}[37mHello \u{1b}[39m \n\u{1b}[37mWorld\u{1b}[39m  "
        );
        assert_eq!(
            split("\u{1b}[37mHello Wo\u{1b}[37mrld\u{1b}[0m", 7),
            "\u{1b}[37mHello \u{1b}[39m \n\u{1b}[37mWo\u{1b}[39m\u{1b}[37mrld\u{1b}[39m  "
        );
        assert_eq!(
            split("\u{1b}[37mHello Wo\u{1b}[37mrld\u{1b}[0m", 8),
            "\u{1b}[37mHello \u{1b}[39m  \n\u{1b}[37mWo\u{1b}[39m\u{1b}[37mrld\u{1b}[39m   "
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
                "\u{1b}[30mDebian\u{1b}[39m\u{1b}[31mDebian\u{1b}[39m\u{1b}[32mDebian\u{1b}[39m\u{1b}[33mDebian\u{1b}[39m\u{1b}[34mDebian\u{1b}[39m",
                "\u{1b}[35mDebian\u{1b}[39m\u{1b}[36mDebian\u{1b}[39m\u{1b}[37mDebian\u{1b}[39m\u{1b}[40mDebian\u{1b}[49m\u{1b}[41mDebian\u{1b}[49m",
                "\u{1b}[42mDebian\u{1b}[49m\u{1b}[43mDebian\u{1b}[49m\u{1b}[44mDebian\u{1b}[49m",
            ]
        );
    }

    #[cfg(feature = "color")]
    #[test]
    fn chunks_wrap_3() {
        let text = "\u{1b}[37mCreate bytes from the \u{1b}[0m\u{1b}[7;34marg\u{1b}[0m\u{1b}[37muments.\u{1b}[0m";

        assert_eq!(
            chunks(text, 22, "", ""),
            [
                "\u{1b}[37mCreate bytes from the \u{1b}[39m",
                "\u{1b}[7m\u{1b}[34marg\u{1b}[27m\u{1b}[39m\u{1b}[37muments.\u{1b}[39m"
            ]
        );
    }

    #[cfg(feature = "color")]
    #[test]
    fn chunks_wrap_3_keeping_words() {
        let text = "\u{1b}[37mCreate bytes from the \u{1b}[0m\u{1b}[7;34marg\u{1b}[0m\u{1b}[37muments.\u{1b}[0m";

        assert_eq!(
            split_keeping_words(text, 22, "", ""),
            "\u{1b}[37mCreate bytes from the \u{1b}[39m\n\u{1b}[7m\u{1b}[34marg\u{1b}[27m\u{1b}[39m\u{1b}[37muments.\u{1b}[39m            "
        );
    }

    #[cfg(feature = "color")]
    #[test]
    fn chunks_wrap_4() {
        let text = "\u{1b}[37mReturns the floor of a number (l\u{1b}[0m\u{1b}[41;37marg\u{1b}[0m\u{1b}[37mest integer less than or equal to that number).\u{1b}[0m";

        assert_eq!(
            chunks(text, 10, "", ""),
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
            ]
        );
    }

    #[cfg(feature = "color")]
    #[test]
    fn chunks_wrap_4_keeping_words() {
        let text = "\u{1b}[37mReturns the floor of a number (l\u{1b}[0m\u{1b}[41;37marg\u{1b}[0m\u{1b}[37mest integer less than or equal to that number).\u{1b}[0m";
        assert_eq!(
            split_keeping_words(text, 10, "", ""),
            concat!(
                "\u{1b}[37mReturns \u{1b}[39m  \n",
                "\u{1b}[37mthe floor \u{1b}[39m\n",
                "\u{1b}[37mof a \u{1b}[39m     \n",
                "\u{1b}[37mnumber \u{1b}[39m   \n",
                "\u{1b}[37m(l\u{1b}[39m\u{1b}[37m\u{1b}[41marg\u{1b}[39m\u{1b}[49m\u{1b}[37mest \u{1b}[39m \n",
                "\u{1b}[37minteger \u{1b}[39m  \n",
                "\u{1b}[37mless than \u{1b}[39m\n",
                "\u{1b}[37mor equal \u{1b}[39m \n",
                "\u{1b}[37mto that \u{1b}[39m  \n",
                "\u{1b}[37mnumber).\u{1b}[39m  ",
            )
        );
    }
}

//  \u{1b}[37mReturns \u{1b}[39m\n
//  \u{1b}[37mthe floor \u{1b}[39m\n
//  \u{1b}[37mof a \u{1b}[39m\n
//  \u{1b}[37mnumber \u{1b}[39m\u{1b}[49m\n
//  \u{1b}[37m\u{1b}[41m(l\u{1b}[39m\u{1b}[37m\u{1b}[41marg\u{1b}[39m\u{1b}[49m\u{1b}[37mest \u{1b}[39m\n
//  \u{1b}[37minteger \u{1b}[39m\n
//  \u{1b}[37mless than \u{1b}[39m\n
//  \u{1b}[37mor equal \u{1b}[39m\n
//  \u{1b}[37mto that \u{1b}[39m\n
//  \u{1b}[37mnumber).\u{1b}[39m  "

//
//

//  \u{1b}[37mReturns \u{1b}[39m\n
//  \u{1b}[37mthe floor \u{1b}[39m\n
//  \u{1b}[37mof a \u{1b}[39m\n
//  \u{1b}[37mnumber \u{1b}[39m\u{1b}[49m\n
//  \u{1b}[37m\u{1b}[41m(l\u{1b}[39m\u{1b}[37m\u{1b}[41marg\u{1b}[39m\u{1b}[49m\u{1b}[37mest \u{1b}[39m\n
//  \u{1b}[37minteger \u{1b}[39m\n
//  \u{1b}[37mless than \u{1b}[39m\n
//  \u{1b}[37mor equal \u{1b}[39m\n
//  \u{1b}[37mto that \u{1b}[39m\n
//  \u{1b}[37mnumber).\u{1b}[39m  "

// "\u{1b}[37mReturns\u{1b}[37m \u{1b}[39m\n
// \u{1b}[37mthe\u{1b}[37m floor\u{1b}[37m \u{1b}[39m\n
// \u{1b}[37mof\u{1b}[37m a\u{1b}[37m \u{1b}[39m\n
// \u{1b}[37mnumber\u{1b}[37m \u{1b}[39m\u{1b}[49m\n
// \u{1b}[37m\u{1b}[41m(l\u{1b}[39m\u{1b}[37m\u{1b}[41marg\u{1b}[39m\u{1b}[49m\u{1b}[37mest\u{1b}[37m \u{1b}[39m\n
// \u{1b}[37minteger\u{1b}[37m \u{1b}[39m\n
// \u{1b}[37mless\u{1b}[37m than\u{1b}[37m \u{1b}[39m\n
// \u{1b}[37mor\u{1b}[37m equal\u{1b}[37m \u{1b}[39m\n
// \u{1b}[37mto\u{1b}[37m that\u{1b}[37m \u{1b}[39m\n
// \u{1b}[37mnumber).\u{1b}[39m  "
