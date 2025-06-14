//! This module contains [`Truncate`] structure, used to decrease width of a [`Table`]s or a cell on a [`Table`] by truncating the width.
//!
//! [`Table`]: crate::Table

use std::{borrow::Cow, iter};

use crate::{
    grid::{
        config::{ColoredConfig, Entity, Position, SpannedConfig},
        dimension::{CompleteDimension, Estimate, IterGridDimension},
        records::{
            vec_records::Cell, EmptyRecords, ExactRecords, IntoRecords, PeekableRecords, Records,
            RecordsMut,
        },
        util::string::{get_line_width, get_lines},
    },
    settings::{
        measurement::Measurement,
        peaker::{Peaker, PriorityNone},
        CellOption, TableOption, Width,
    },
};

use super::util::get_table_total_width;
use crate::util::string::cut_str;

/// Truncate cut the string to a given width if its length exceeds it.
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
/// use tabled::{Table, settings::{object::Segment, Width, Modify}};
///
/// let table = Table::new(&["Hello World!"])
///     .with(Modify::new(Segment::all()).with(Width::truncate(3)));
/// ```
///
/// [`Padding`]: crate::settings::Padding
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Truncate<'a, W = usize, P = PriorityNone> {
    width: W,
    suffix: Option<TruncateSuffix<'a>>,
    multiline: bool,
    priority: P,
}

#[cfg(feature = "ansi")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct TruncateSuffix<'a> {
    text: Cow<'a, str>,
    limit: SuffixLimit,
    try_color: bool,
}

#[cfg(not(feature = "ansi"))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct TruncateSuffix<'a> {
    text: Cow<'a, str>,
    limit: SuffixLimit,
}

impl Default for TruncateSuffix<'_> {
    fn default() -> Self {
        Self {
            text: Cow::default(),
            limit: SuffixLimit::Cut,
            #[cfg(feature = "ansi")]
            try_color: false,
        }
    }
}

/// A suffix limit settings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SuffixLimit {
    /// Cut the suffix.
    Cut,
    /// Don't show the suffix.
    Ignore,
    /// Use a string with n chars instead.
    Replace(char),
}

impl<W> Truncate<'static, W>
where
    W: Measurement<Width>,
{
    /// Creates a [`Truncate`] object
    pub const fn new(width: W) -> Truncate<'static, W> {
        Self {
            width,
            multiline: false,
            suffix: None,
            priority: PriorityNone::new(),
        }
    }
}

impl<'a, W, P> Truncate<'a, W, P> {
    /// Sets a suffix which will be appended to a resultant string.
    ///
    /// The suffix is used in 3 circumstances:
    ///     1. If original string is *bigger* than the suffix.
    ///        We cut more of the original string and append the suffix.
    ///     2. If suffix is bigger than the original string.
    ///        We cut the suffix to fit in the width by default.
    ///        But you can peak the behaviour by using [`Truncate::suffix_limit`]
    pub fn suffix<S: Into<Cow<'a, str>>>(self, suffix: S) -> Truncate<'a, W, P> {
        let mut suff = self.suffix.unwrap_or_default();
        suff.text = suffix.into();

        Truncate {
            width: self.width,
            multiline: self.multiline,
            priority: self.priority,
            suffix: Some(suff),
        }
    }

    /// Sets a suffix limit, which is used when the suffix is too big to be used.
    pub fn suffix_limit(self, limit: SuffixLimit) -> Truncate<'a, W, P> {
        let mut suff = self.suffix.unwrap_or_default();
        suff.limit = limit;

        Truncate {
            width: self.width,
            multiline: self.multiline,
            priority: self.priority,
            suffix: Some(suff),
        }
    }

    /// Use trancate logic per line, not as a string as a whole.
    pub fn multiline(self, on: bool) -> Truncate<'a, W, P> {
        Truncate {
            width: self.width,
            multiline: on,
            suffix: self.suffix,
            priority: self.priority,
        }
    }

    #[cfg(feature = "ansi")]
    /// Sets a optional logic to try to colorize a suffix.
    pub fn suffix_try_color(self, color: bool) -> Truncate<'a, W, P> {
        let mut suff = self.suffix.unwrap_or_default();
        suff.try_color = color;

        Truncate {
            width: self.width,
            multiline: self.multiline,
            priority: self.priority,
            suffix: Some(suff),
        }
    }
}

impl<'a, W, P> Truncate<'a, W, P> {
    /// Priority defines the logic by which a truncate will be applied when is done for the whole table.
    ///
    /// - [`PriorityNone`] which cuts the columns one after another.
    /// - [`PriorityMax`] cuts the biggest columns first.
    /// - [`PriorityMin`] cuts the lowest columns first.
    ///
    /// [`PriorityMax`]: crate::settings::peaker::PriorityMax
    /// [`PriorityMin`]: crate::settings::peaker::PriorityMin
    pub fn priority<PP: Peaker>(self, priority: PP) -> Truncate<'a, W, PP> {
        Truncate {
            width: self.width,
            multiline: self.multiline,
            suffix: self.suffix,
            priority,
        }
    }
}

impl Truncate<'_, (), ()> {
    /// Truncate a given string
    pub fn truncate(text: &str, width: usize) -> Cow<'_, str> {
        truncate_text(text, width, "", false)
    }
}

impl<W, P, R> CellOption<R, ColoredConfig> for Truncate<'_, W, P>
where
    W: Measurement<Width>,
    R: Records + ExactRecords + PeekableRecords + RecordsMut<String>,
    for<'a> &'a R: Records,
    for<'a> <<&'a R as Records>::Iter as IntoRecords>::Cell: AsRef<str>,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, entity: Entity) {
        let available = self.width.measure(&*records, cfg);

        let mut width = available;

        let colorize = need_suffix_color_preservation(&self.suffix);
        let mut suffix = Cow::Borrowed("");
        if let Some(x) = self.suffix.as_ref() {
            let (cut_suffix, rest_width) = make_suffix(x, width);
            suffix = cut_suffix;
            width = rest_width;
        }

        let count_rows = records.count_rows();
        let count_columns = records.count_columns();
        let max_pos = Position::new(count_rows, count_columns);

        for pos in entity.iter(count_rows, count_columns) {
            if !max_pos.has_coverage(pos) {
                continue;
            }

            let cell_width = records.get_width(pos);
            if available >= cell_width {
                continue;
            }

            let text = records.get_text(pos);
            let text =
                truncate_multiline(text, &suffix, width, available, colorize, self.multiline);

            records.set(pos, text.into_owned());
        }
    }
}

impl<W, P, R> TableOption<R, ColoredConfig, CompleteDimension> for Truncate<'_, W, P>
where
    W: Measurement<Width>,
    P: Peaker,
    R: Records + ExactRecords + PeekableRecords + RecordsMut<String>,
    for<'a> &'a R: Records,
    for<'a> <<&'a R as Records>::Iter as IntoRecords>::Cell: Cell + AsRef<str>,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, dims: &mut CompleteDimension) {
        if records.count_rows() == 0 || records.count_columns() == 0 {
            return;
        }

        let width = self.width.measure(&*records, cfg);

        dims.estimate(&*records, cfg);
        let widths = dims.get_widths().expect("must be present");

        let total = get_table_total_width(widths, cfg);
        if total <= width {
            return;
        }

        let t = Truncate {
            multiline: self.multiline,
            priority: self.priority,
            suffix: self.suffix,
            width,
        };

        let widths = truncate_total_width(records, cfg, widths, total, t);

        dims.set_widths(widths);
        dims.clear_height(); // TODO: Can be optimized -- we must know the proper height already since we did wrap
    }

    fn hint_change(&self) -> Option<Entity> {
        // NOTE: We properly set widths and heights so nothign got need reastimation
        None
    }
}

fn truncate_multiline<'a>(
    text: &'a str,
    suffix: &'a str,
    width: usize,
    twidth: usize,
    suffix_color: bool,
    multiline: bool,
) -> Cow<'a, str> {
    if !multiline {
        return make_text_truncated(text, suffix, width, twidth, suffix_color);
    }

    let mut buf = String::new();
    for (i, line) in get_lines(text).enumerate() {
        if i != 0 {
            buf.push('\n');
        }

        let line = make_text_truncated(&line, suffix, width, twidth, suffix_color);
        buf.push_str(&line);
    }

    Cow::Owned(buf)
}

fn make_text_truncated<'a>(
    text: &'a str,
    suffix: &'a str,
    width: usize,
    twidth: usize,
    suffix_color: bool,
) -> Cow<'a, str> {
    if width == 0 {
        if twidth == 0 {
            Cow::Borrowed("")
        } else {
            Cow::Borrowed(suffix)
        }
    } else {
        truncate_text(text, width, suffix, suffix_color)
    }
}

fn need_suffix_color_preservation(_suffix: &Option<TruncateSuffix<'_>>) -> bool {
    #[cfg(not(feature = "ansi"))]
    {
        false
    }
    #[cfg(feature = "ansi")]
    {
        _suffix.as_ref().is_some_and(|s| s.try_color)
    }
}

fn make_suffix<'a>(suffix: &'a TruncateSuffix<'_>, width: usize) -> (Cow<'a, str>, usize) {
    let suffix_length = get_line_width(&suffix.text);
    if width > suffix_length {
        return (Cow::Borrowed(suffix.text.as_ref()), width - suffix_length);
    }

    match suffix.limit {
        SuffixLimit::Ignore => (Cow::Borrowed(""), width),
        SuffixLimit::Cut => {
            let suffix = cut_str(&suffix.text, width);
            (suffix, 0)
        }
        SuffixLimit::Replace(c) => {
            let suffix = Cow::Owned(iter::repeat_n(c, width).collect());
            (suffix, 0)
        }
    }
}

fn truncate_total_width<P, R>(
    records: &mut R,
    cfg: &mut ColoredConfig,
    widths: &[usize],
    total: usize,
    t: Truncate<'_, usize, P>,
) -> Vec<usize>
where
    P: Peaker,
    R: Records + PeekableRecords + ExactRecords + RecordsMut<String>,
    for<'a> &'a R: Records,
    for<'a> <<&'a R as Records>::Iter as IntoRecords>::Cell: AsRef<str>,
{
    let count_rows = records.count_rows();
    let count_columns = records.count_columns();

    let colorize = need_suffix_color_preservation(&t.suffix);
    let mut suffix = Cow::Borrowed("");

    let min_widths = IterGridDimension::width(EmptyRecords::new(count_rows, count_columns), cfg);

    let mut widths = widths.to_vec();
    decrease_widths(&mut widths, &min_widths, total, t.width, t.priority);

    let points = get_decrease_cell_list(cfg, &widths, &min_widths, count_rows, count_columns);

    for (pos, mut width) in points {
        let text_width = records.get_width(pos);
        if width >= text_width {
            continue;
        }

        let text = records.get_text(pos);
        if let Some(x) = &t.suffix {
            let (cut_suffix, rest_width) = make_suffix(x, width);
            suffix = cut_suffix;
            width = rest_width;
        }

        let text = truncate_multiline(text, &suffix, width, text_width, colorize, t.multiline);

        records.set(pos, text.into_owned());
    }

    widths
}

fn truncate_text<'a>(
    text: &'a str,
    width: usize,
    suffix: &str,
    _suffix_color: bool,
) -> Cow<'a, str> {
    let content = cut_str(text, width);
    if suffix.is_empty() {
        return content;
    }

    #[cfg(feature = "ansi")]
    {
        if _suffix_color {
            if let Some(block) = ansi_str::get_blocks(text).last() {
                if block.has_ansi() {
                    let style = block.style();
                    Cow::Owned(format!(
                        "{}{}{}{}",
                        content,
                        style.start(),
                        suffix,
                        style.end()
                    ))
                } else {
                    let mut content = content.into_owned();
                    content.push_str(suffix);
                    Cow::Owned(content)
                }
            } else {
                let mut content = content.into_owned();
                content.push_str(suffix);
                Cow::Owned(content)
            }
        } else {
            let mut content = content.into_owned();
            content.push_str(suffix);
            Cow::Owned(content)
        }
    }

    #[cfg(not(feature = "ansi"))]
    {
        let mut content = content.into_owned();
        content.push_str(suffix);
        Cow::Owned(content)
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

fn count_borders(cfg: &SpannedConfig, start: usize, end: usize, count_columns: usize) -> usize {
    (start..end)
        .skip(1)
        .filter(|&i| cfg.has_vertical(i, count_columns))
        .count()
}
