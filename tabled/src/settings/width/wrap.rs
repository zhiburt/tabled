//! This module contains [`Wrap`] structure, used to decrease width of a [`Table`]s or a cell on a [`Table`] by wrapping it's content
//! to a new line.
//!
//! [`Table`]: crate::Table

use papergrid::dimension::{iterable::IterGridDimension, Estimate};

use crate::{
    grid::{
        config::{ColoredConfig, Entity, Position, SpannedConfig},
        dimension::CompleteDimension,
        records::{
            vec_records::Cell, EmptyRecords, ExactRecords, IntoRecords, PeekableRecords, Records,
            RecordsMut,
        },
        util::string::{get_char_width, get_string_width},
    },
    settings::{
        measurement::Measurement,
        peaker::{Peaker, PriorityNone},
        width::Width,
        CellOption, TableOption,
    },
};

use super::util::get_table_total_width;

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

impl<W, P, R> TableOption<R, ColoredConfig, CompleteDimension<'_>> for Wrap<W, P>
where
    W: Measurement<Width>,
    P: Peaker,
    R: Records + ExactRecords + PeekableRecords + RecordsMut<String>,
    for<'a> &'a R: Records,
    for<'a> <<&'a R as Records>::Iter as IntoRecords>::Cell: Cell + AsRef<str>,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, dims: &mut CompleteDimension<'_>) {
        if records.count_rows() == 0 || records.count_columns() == 0 {
            return;
        }

        let width = self.width.measure(&*records, cfg);

        dims.estimate(&*records, cfg);
        let widths = dims.get_widths().expect("must be found");

        let total = get_table_total_width(widths, cfg);
        if width >= total {
            return;
        }

        let w = Wrap {
            keep_words: self.keep_words,
            priority: self.priority,
            width,
        };
        let widths = wrap_total_width(records, cfg, widths, total, w);

        dims.set_widths(widths);
        dims.clear_height();
    }
}

impl<W, R, P> CellOption<R, ColoredConfig> for Wrap<W, P>
where
    W: Measurement<Width>,
    R: Records + ExactRecords + PeekableRecords + RecordsMut<String>,
    for<'a> &'a R: Records,
    for<'a> <<&'a R as Records>::Iter as IntoRecords>::Cell: AsRef<str>,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, entity: Entity) {
        let count_rows = records.count_rows();
        let count_columns = records.count_columns();
        let max_pos = Position::new(count_rows, count_columns);

        let width = self.width.measure(&*records, cfg);

        for pos in entity.iter(count_rows, count_columns) {
            if !max_pos.has_coverage(pos) {
                continue;
            }

            let cell_width = records.get_width(pos);
            if cell_width <= width {
                continue;
            }

            let text = records.get_text(pos);
            let wrapped = wrap_text(text, width, self.keep_words);
            records.set(pos, wrapped);
        }
    }
}

fn wrap_total_width<R, P>(
    records: &mut R,
    cfg: &mut ColoredConfig,
    widths: &[usize],
    total: usize,
    w: Wrap<usize, P>,
) -> Vec<usize>
where
    R: Records + ExactRecords + PeekableRecords + RecordsMut<String>,
    P: Peaker,
    for<'a> &'a R: Records,
    for<'a> <<&'a R as Records>::Iter as IntoRecords>::Cell: AsRef<str>,
{
    let count_rows = records.count_rows();
    let count_columns = records.count_columns();

    // TODO: Could be optimized by calculating width and min_width together
    //       I just don't like the boiler plate we will add :(
    //       But the benefit is clear.
    let min_widths = IterGridDimension::width(EmptyRecords::new(count_rows, count_columns), cfg);

    let mut widths = widths.to_vec();
    decrease_widths(&mut widths, &min_widths, total, w.width, w.priority);

    let points = get_decrease_cell_list(cfg, &widths, &min_widths, count_rows, count_columns);

    for (pos, width) in points {
        let text = records.get_text(pos);
        let wrapped = wrap_text(text, width, w.keep_words);
        records.set(pos, wrapped);
    }

    widths
}

fn wrap_text(text: &str, width: usize, keep_words: bool) -> String {
    if width == 0 {
        return String::new();
    }

    #[cfg(not(feature = "ansi"))]
    {
        if keep_words {
            wrap_text_keeping_words_noansi(text, width)
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

fn wrap_text_keeping_words_noansi(text: &str, width: usize) -> String {
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
    const REPLACEMENT: char = '\u{FFFD}';

    struct Blocks<'a> {
        iter: ansi_str::AnsiBlockIter<'a>,
        last_block: Option<BlockSlice<'a>>,
    }

    struct BlockSlice<'a> {
        block: ansi_str::AnsiBlock<'a>,
        pos: usize,
    }

    impl<'a> Blocks<'a> {
        fn new(text: &'a str) -> Self {
            Self {
                iter: ansi_str::get_blocks(text),
                last_block: None,
            }
        }

        fn read(&mut self, buf: &mut String, nbytes: usize) {
            use std::fmt::Write;

            debug_assert_ne!(nbytes, 0);

            let mut size: usize = nbytes;

            if let Some(mut b) = self.last_block.take() {
                let text = b.block.text();
                let slice = &text[b.pos..];
                let slice_size = slice.len();

                match slice_size.cmp(&size) {
                    std::cmp::Ordering::Less => {
                        buf.push_str(slice);
                        let _ = write!(buf, "{}", b.block.style().end());
                        size -= slice_size;
                    }
                    std::cmp::Ordering::Equal => {
                        buf.push_str(slice);
                        let _ = write!(buf, "{}", b.block.style().end());
                        return;
                    }
                    std::cmp::Ordering::Greater => {
                        let truncated = &slice[..size];
                        buf.push_str(truncated);
                        b.pos += size;
                        self.last_block = Some(b);
                        return;
                    }
                }
            }

            for block in self.iter.by_ref() {
                let text = block.text();
                if text.is_empty() {
                    continue;
                }

                let text_size = text.len();

                match text_size.cmp(&size) {
                    std::cmp::Ordering::Less => {
                        let _ = write!(buf, "{}", block.style().start());
                        buf.push_str(text);
                        let _ = write!(buf, "{}", block.style().end());
                        size -= text_size;
                    }
                    std::cmp::Ordering::Equal => {
                        let _ = write!(buf, "{}", block.style().start());
                        buf.push_str(text);
                        let _ = write!(buf, "{}", block.style().end());
                        return;
                    }
                    std::cmp::Ordering::Greater => {
                        let _ = write!(buf, "{}", block.style().start());
                        let slice = &text[..size];
                        buf.push_str(slice);
                        self.last_block = Some(BlockSlice { block, pos: size });

                        return;
                    }
                }
            }
        }

        fn read_char(&mut self, buf: &mut String) {
            use std::fmt::Write;

            if let Some(mut b) = self.last_block.take() {
                let text = b.block.text();
                let slice = &text[b.pos..];

                let mut chars = slice.chars();
                let ch = chars.next().expect("ok");
                let ch_size = ch.len_utf8();

                buf.push(ch);

                if chars.next().is_none() {
                    let _ = write!(buf, "{}", b.block.style().end());
                    return;
                } else {
                    debug_assert_ne!(ch_size, 0);

                    b.pos += ch_size;
                    self.last_block = Some(b);

                    return;
                }
            }

            for block in self.iter.by_ref() {
                let text = block.text();
                if text.is_empty() {
                    continue;
                }

                let mut chars = text.chars();
                let ch = chars.next().expect("ok");
                let ch_size = ch.len_utf8();

                let _ = write!(buf, "{}", block.style().start());
                buf.push(ch);

                if chars.next().is_none() {
                    let _ = write!(buf, "{}", block.style().end());
                    return;
                } else {
                    debug_assert_ne!(ch_size, 0);

                    self.last_block = Some(BlockSlice {
                        block,
                        pos: ch_size,
                    });
                    return;
                }
            }
        }

        fn skip(&mut self, buf: &mut String, nbytes: usize) {
            use std::fmt::Write;

            debug_assert_ne!(nbytes, 0);

            let mut size = nbytes;

            if let Some(mut b) = self.last_block.take() {
                let text = b.block.text();
                let slice = &text[b.pos..];
                let slice_size = slice.len();

                match slice_size.cmp(&size) {
                    std::cmp::Ordering::Less => {
                        let _ = write!(buf, "{}", b.block.style().end());
                        size -= slice_size;
                    }
                    std::cmp::Ordering::Equal => {
                        let _ = write!(buf, "{}", b.block.style().end());
                        return;
                    }
                    std::cmp::Ordering::Greater => {
                        b.pos += size;
                        self.last_block = Some(b);
                        return;
                    }
                }
            }

            for block in self.iter.by_ref() {
                let text = block.text();
                let text_size = text.len();

                if text.is_empty() {
                    continue;
                }

                match text_size.cmp(&size) {
                    std::cmp::Ordering::Less => {
                        size -= text_size;
                    }
                    std::cmp::Ordering::Equal => {
                        return;
                    }
                    std::cmp::Ordering::Greater => {
                        let _ = write!(buf, "{}", block.style().start());
                        self.last_block = Some(BlockSlice { block, pos: size });
                        return;
                    }
                }
            }
        }

        fn finish(&mut self, buf: &mut String) {
            use std::fmt::Write;

            if let Some(b) = &mut self.last_block {
                let _ = write!(buf, "{}", b.block.style().end());
            }
        }

        fn start(&mut self, buf: &mut String) {
            use std::fmt::Write;

            if let Some(b) = &mut self.last_block {
                let _ = write!(buf, "{}", b.block.style().start());
            }
        }
    }

    if width == 0 || text.is_empty() {
        return String::new();
    }

    let stripped = ansi_str::AnsiStr::ansi_strip(text);
    let is_simple_text = stripped.len() == text.len() && prefix.is_empty() && suffix.is_empty();
    if is_simple_text {
        return wrap_text_keeping_words_noansi(text, width);
    }

    let mut buf = String::with_capacity(width + prefix.len() + suffix.len());
    let mut line_width = 0;
    let mut blocks = Blocks::new(text);

    buf.push_str(prefix);

    for word in stripped.split(' ') {
        let word_width = get_string_width(word);
        let word_size = word.len();

        // restore space char if we can
        if line_width > 0 {
            let line_has_space = line_width < width;
            if line_has_space {
                blocks.read(&mut buf, 1);
                line_width += 1;
            } else {
                // special case where we want to keep ' ' space
                if width == 1 {
                    blocks.finish(&mut buf);
                    buf.push_str(suffix);
                    buf.push('\n');
                    buf.push_str(prefix);
                    blocks.start(&mut buf);
                    blocks.read(&mut buf, 1);
                    line_width = 1;
                } else {
                    blocks.skip(&mut buf, 1);
                }
            }
        } else if word_width == 0 {
            blocks.skip(&mut buf, 1);
        }

        if word_width == 0 {
            continue;
        }

        let line_has_space = line_width + word_width <= width;
        if line_has_space {
            blocks.read(&mut buf, word_size);
            line_width += word_width;
            continue;
        }

        let is_small_word = word_width <= width;
        if is_small_word {
            blocks.finish(&mut buf);
            buf.push_str(suffix);
            buf.push('\n');
            buf.push_str(prefix);
            blocks.start(&mut buf);
            blocks.read(&mut buf, word_size);
            line_width = word_width;
            continue;
        }

        // take 1 char by 1 and just push it
        for c in word.chars() {
            let char_width = std::cmp::max(1, get_char_width(c));
            let char_size = c.len_utf8();

            let line_has_space = line_width + char_width <= width;
            if line_has_space {
                blocks.read_char(&mut buf);
                line_width += char_width;
                continue;
            }

            let is_char_small = char_width <= width;
            if is_char_small {
                blocks.finish(&mut buf);
                buf.push_str(suffix);
                buf.push('\n');
                buf.push_str(prefix);
                blocks.start(&mut buf);
                blocks.read_char(&mut buf);
                line_width = char_width;
                continue;
            }

            if line_width == width {
                blocks.finish(&mut buf);
                buf.push_str(suffix);
                buf.push('\n');
                buf.push_str(prefix);
                blocks.start(&mut buf);
                line_width = 0;
            }

            // NOTE:
            // Practically it only can happen if we wrap some late UTF8 symbol.
            // For example:
            // Emojie with width 2 but and wrap width 1
            let available = width - line_width;
            buf.extend(std::iter::repeat_n(REPLACEMENT, available));
            line_width = width;
            blocks.skip(&mut buf, char_size);
        }
    }

    buf.push_str(suffix);

    buf
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
    count_rows: usize,
    count_columns: usize,
) -> Vec<(Position, usize)> {
    let mut points = Vec::new();
    for col in 0..count_columns {
        for row in 0..count_rows {
            let pos = Position::new(row, col);
            if !cfg.is_cell_visible(pos) {
                continue;
            }

            let (width, width_min) = match cfg.get_column_span(pos) {
                Some(span) => {
                    let width = (col..col + span).map(|i| widths[i]).sum::<usize>();
                    let min_width = (col..col + span).map(|i| min_widths[i]).sum::<usize>();
                    let count_borders = count_borders(cfg, col, col + span, count_columns);
                    (width + count_borders, min_width + count_borders)
                }
                None => (widths[col], min_widths[col]),
            };

            if width >= width_min {
                let padding = cfg.get_padding(pos);
                let width = width.saturating_sub(padding.left.size + padding.right.size);

                points.push((pos, width));
            }
        }
    }

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

    #[test]
    fn split_by_line_keeping_words_test() {
        #[cfg(not(feature = "ansi"))]
        let split_keeping_words = |text, width| wrap_text_keeping_words_noansi(text, width);
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
        let split_keeping_words = |text, width| wrap_text_keeping_words(text, width, "", "");

        let text = "\u{1b}[36mJapanese “vacancy” button\u{1b}[0m";
        assert_eq!(split_keeping_words(text, 2), "\u{1b}[36mJa\u{1b}[39m\n\u{1b}[36mpa\u{1b}[39m\n\u{1b}[36mne\u{1b}[39m\n\u{1b}[36mse\u{1b}[39m\n\u{1b}[36m“v\u{1b}[39m\n\u{1b}[36mac\u{1b}[39m\n\u{1b}[36man\u{1b}[39m\n\u{1b}[36mcy\u{1b}[39m\n\u{1b}[36m” \u{1b}[39m\n\u{1b}[36mbu\u{1b}[39m\n\u{1b}[36mtt\u{1b}[39m\n\u{1b}[36mon\u{1b}[39m");
        assert_eq!(split_keeping_words(text, 1), "\u{1b}[36mJ\u{1b}[39m\n\u{1b}[36ma\u{1b}[39m\n\u{1b}[36mp\u{1b}[39m\n\u{1b}[36ma\u{1b}[39m\n\u{1b}[36mn\u{1b}[39m\n\u{1b}[36me\u{1b}[39m\n\u{1b}[36ms\u{1b}[39m\n\u{1b}[36me\u{1b}[39m\n\u{1b}[36m \u{1b}[39m\n\u{1b}[36m“\u{1b}[39m\n\u{1b}[36mv\u{1b}[39m\n\u{1b}[36ma\u{1b}[39m\n\u{1b}[36mc\u{1b}[39m\n\u{1b}[36ma\u{1b}[39m\n\u{1b}[36mn\u{1b}[39m\n\u{1b}[36mc\u{1b}[39m\n\u{1b}[36my\u{1b}[39m\n\u{1b}[36m”\u{1b}[39m\n\u{1b}[36m \u{1b}[39m\n\u{1b}[36mb\u{1b}[39m\n\u{1b}[36mu\u{1b}[39m\n\u{1b}[36mt\u{1b}[39m\n\u{1b}[36mt\u{1b}[39m\n\u{1b}[36mo\u{1b}[39m\n\u{1b}[36mn\u{1b}[39m");
    }

    #[cfg(feature = "ansi")]
    #[test]
    fn split_by_line_keeping_words_color_2_test() {
        use ansi_str::AnsiStr;

        let split_keeping_words = |text, width| wrap_text_keeping_words(text, width, "", "");

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
                "\u{1b}[37mOM\u{1b}[39m",
                "\u{1b}[37mYA\u{1b}[39m",
                "\u{1b}[37mAn\u{1b}[39m",
                "\u{1b}[37mdi\u{1b}[39m",
                "\u{1b}[37mna\u{1b}[39m",
                "\u{1b}[37m38\u{1b}[39m",
                "\u{1b}[37m24\u{1b}[39m",
                "\u{1b}[37m90\u{1b}[39m",
                "\u{1b}[37m99\u{1b}[39m",
                "\u{1b}[37m99\u{1b}[39m",
                "\u{1b}[37mCa\u{1b}[39m",
                "\u{1b}[37mlc\u{1b}[39m",
                "\u{1b}[37miu\u{1b}[39m",
                "\u{1b}[37mm \u{1b}[39m",
                "\u{1b}[37mca\u{1b}[39m",
                "\u{1b}[37mrb\u{1b}[39m",
                "\u{1b}[37mon\u{1b}[39m",
                "\u{1b}[37mat\u{1b}[39m",
                "\u{1b}[37me \u{1b}[39m",
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
            "\u{1b}[37m🚵\u{1b}[39m\n\u{1b}[37m🏻\u{1b}[39m\n\u{1b}[37m🚵\u{1b}[39m\n\u{1b}[37m🏻\u{1b}[39m\n\u{1b}[37m🚵\u{1b}[39m\n\u{1b}[37m🏻\u{1b}[39m\n\u{1b}[37m🚵\u{1b}[39m\n\u{1b}[37m🏻\u{1b}[39m\n\u{1b}[37m🚵\u{1b}[39m\n\u{1b}[37m🏻\u{1b}[39m\n\u{1b}[37m🚵\u{1b}[39m\n\u{1b}[37m🏻\u{1b}[39m\n\u{1b}[37m🚵\u{1b}[39m\n\u{1b}[37m🏻\u{1b}[39m\n\u{1b}[37m🚵\u{1b}[39m\n\u{1b}[37m🏻\u{1b}[39m\n\u{1b}[37m🚵\u{1b}[39m\n\u{1b}[37m🏻\u{1b}[39m\n\u{1b}[37m🚵\u{1b}[39m\n\u{1b}[37m🏻\u{1b}[39m",
        );
        assert_eq!(
            split("\u{1b}[37mthis is a long sentence\u{1b}[0m", 7),
            "\u{1b}[37mthis is\u{1b}[39m\n\u{1b}[37ma long \u{1b}[39m\n\u{1b}[37msentenc\u{1b}[39m\n\u{1b}[37me\u{1b}[39m"
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

    #[test]
    fn split_keeping_words_4_test() {
        #[cfg(feature = "ansi")]
        let split_keeping_words = |text, width| wrap_text_keeping_words(text, width, "", "");
        #[cfg(not(feature = "ansi"))]
        let split_keeping_words = |text, width| wrap_text_keeping_words_noansi(text, width);

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
                "^\u{1b}[37mOM\u{1b}[39m$",
                "^\u{1b}[37mYA\u{1b}[39m$",
                "^\u{1b}[37mAn\u{1b}[39m$",
                "^\u{1b}[37mdi\u{1b}[39m$",
                "^\u{1b}[37mna\u{1b}[39m$",
                "^\u{1b}[37m38\u{1b}[39m$",
                "^\u{1b}[37m24\u{1b}[39m$",
                "^\u{1b}[37m90\u{1b}[39m$",
                "^\u{1b}[37m99\u{1b}[39m$",
                "^\u{1b}[37m99\u{1b}[39m$",
                "^\u{1b}[37mCa\u{1b}[39m$",
                "^\u{1b}[37mlc\u{1b}[39m$",
                "^\u{1b}[37miu\u{1b}[39m$",
                "^\u{1b}[37mm \u{1b}[39m$",
                "^\u{1b}[37mca\u{1b}[39m$",
                "^\u{1b}[37mrb\u{1b}[39m$",
                "^\u{1b}[37mon\u{1b}[39m$",
                "^\u{1b}[37mat\u{1b}[39m$",
                "^\u{1b}[37me \u{1b}[39m$",
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

    #[test]
    fn chunks_wrap_5_keeping_words() {
        #[cfg(feature = "ansi")]
        let split_keeping_words = |text, width| wrap_text_keeping_words(text, width, "", "");
        #[cfg(not(feature = "ansi"))]
        let split_keeping_words = |text, width| wrap_text_keeping_words_noansi(text, width);

        let text = "修复 zlib 软件包中 CMake 配置不一致的问题，该问题先前导致部分软件无法正常构建";
        assert_eq!(
            split_keeping_words(text, 44),
            "修复 zlib 软件包中 CMake 配置不一致的问题，\n该问题先前导致部分软件无法正常构建"
        );
    }

    #[test]
    fn chunks_chinese_0() {
        #[cfg(feature = "ansi")]
        let split_keeping_words = |text, width| wrap_text_keeping_words(text, width, "", "");
        #[cfg(not(feature = "ansi"))]
        let split_keeping_words = |text, width| wrap_text_keeping_words_noansi(text, width);

        let text = "(公司{ 名称:\"腾讯科技（深圳）有限公司\",成立时间:\"1998年11月\"}";
        assert_eq!(
            split_keeping_words(text, 40),
            concat!(
                "(公司{ 名称:\"腾讯科技（深圳）有限公司\",\n",
                "成立时间:\"1998年11月\"}",
            ),
        );
    }

    #[test]
    fn chunks_keeping_chinese_0() {
        #[cfg(feature = "ansi")]
        let split_keeping_words = |text, width| wrap_text_keeping_words(text, width, "", "");
        #[cfg(not(feature = "ansi"))]
        let split_keeping_words = |text, width| wrap_text_keeping_words_noansi(text, width);

        let text = "(公司{ 名称:\"腾讯科技（深圳）有限公司\",成立时间:\"1998年11月\"}";
        assert_eq!(
            split_keeping_words(text, 40),
            concat!(
                "(公司{ 名称:\"腾讯科技（深圳）有限公司\",\n",
                "成立时间:\"1998年11月\"}",
            ),
        );
    }
}
