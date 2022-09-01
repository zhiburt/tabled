//! This module contains [`Truncate`] structure, used to decrease width of a [`Table`]s or a cell on a [`Table`] by truncating the width.

use std::{borrow::Cow, marker::PhantomData};

use papergrid::{
    records::{empty::EmptyRecords, Records, RecordsMut},
    util::cut_str,
    width::{CfgWidthFunction, WidthFunc},
    Entity, GridConfig,
};

use crate::{
    width::{
        count_borders, get_table_widths, get_table_widths_with_total, get_width_value,
        ColumnPeaker, PriorityNone, WidthValue,
    },
    CellOption, Table, TableOption,
};

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
/// use tabled::{object::Segment, truncate::Truncate, Modify, Table};
///
/// let table = Table::new(&["Hello World!"])
///     .with(Modify::new(Segment::all()).with(Truncate::new(3)));
/// ```
///
/// [`Padding`]: crate::Padding
#[derive(Debug)]
pub struct Truncate<'a, W = usize, P = PriorityNone> {
    width: W,
    suffix: Option<TruncateSuffix<'a>>,
    _priority: PhantomData<P>,
}

#[derive(Debug)]
struct TruncateSuffix<'a> {
    text: Cow<'a, str>,
    limit: SuffixLimit,
    #[cfg(feature = "color")]
    try_color: bool,
}

impl Default for TruncateSuffix<'_> {
    fn default() -> Self {
        Self {
            text: Cow::default(),
            limit: SuffixLimit::Cut,
            #[cfg(feature = "color")]
            try_color: false,
        }
    }
}

/// A suffix limit settings.
#[derive(Debug, Clone, Copy)]
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
    W: WidthValue,
{
    /// Creates a [`Truncate`] object
    pub fn new(width: W) -> Truncate<'static, W> {
        Self {
            width,
            suffix: None,
            _priority: PhantomData::default(),
        }
    }
}

impl<'a, W, P> Truncate<'a, W, P> {
    /// Sets a suffix which will be appended to a resultant string.
    ///
    /// The suffix is used in 3 circamstances:
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
            suffix: Some(suff),
            _priority: PhantomData::default(),
        }
    }

    /// Sets a suffix limit, which is used when the suffix is too big to be used.
    pub fn suffix_limit(self, limit: SuffixLimit) -> Truncate<'a, W, P> {
        let mut suff = self.suffix.unwrap_or_default();
        suff.limit = limit;

        Truncate {
            width: self.width,
            suffix: Some(suff),
            _priority: PhantomData::default(),
        }
    }

    #[cfg(feature = "color")]
    /// Sets a optional logic to try to colorize a suffix.
    pub fn suffix_try_color(self, color: bool) -> Truncate<'a, W, P> {
        let mut suff = self.suffix.unwrap_or_default();
        suff.try_color = color;

        Truncate {
            width: self.width,
            suffix: Some(suff),
            _priority: PhantomData::default(),
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
    /// [`PriorityMax`]: crate::width::PriorityMax
    /// [`PriorityMin`]: crate::width::PriorityMin
    pub fn priority<PP: ColumnPeaker>(self) -> Truncate<'a, W, PP> {
        Truncate {
            width: self.width,
            suffix: self.suffix,
            _priority: PhantomData::default(),
        }
    }
}

impl<W, P, R> CellOption<R> for Truncate<'_, W, P>
where
    W: WidthValue,
    R: Records + RecordsMut<String>,
{
    fn change_cell(&mut self, table: &mut Table<R>, entity: Entity) {
        let width_ctrl = CfgWidthFunction::from_cfg(table.get_config());
        let set_width = self
            .width
            .width(table.get_records(), table.get_config(), &width_ctrl);

        let mut width = set_width;
        let suffix = match self.suffix.as_ref() {
            Some(suffix) => {
                let suffix_length = width_ctrl.width(&suffix.text);
                if width > suffix_length {
                    width -= suffix_length;
                    Cow::Borrowed(suffix.text.as_ref())
                } else {
                    match suffix.limit {
                        SuffixLimit::Ignore => Cow::Borrowed(""),
                        SuffixLimit::Cut => {
                            width = 0;
                            cut_str(&suffix.text, set_width)
                        }
                        SuffixLimit::Replace(c) => {
                            width = 0;
                            Cow::Owned(std::iter::repeat(c).take(set_width).collect())
                        }
                    }
                }
            }
            None => Cow::Borrowed(""),
        };

        let (count_rows, count_cols) = table.shape();
        for pos in entity.iter(count_rows, count_cols) {
            let cell_width = table.get_records().get_width(pos, &width_ctrl);
            if set_width >= cell_width {
                continue;
            }

            let suffix_color_try_keeping;
            #[cfg(not(feature = "color"))]
            {
                suffix_color_try_keeping = false;
            }
            #[cfg(feature = "color")]
            {
                suffix_color_try_keeping = self.suffix.as_ref().map_or(false, |s| s.try_color);
            }

            let records = table.get_records();
            let text = records.get_text(pos);
            // todo: Think about it.
            //       We could eliminate this allcation if we would be allowed to cut '\t' with unknown characters.
            //       Currently we don't do that.
            let text = papergrid::util::replace_tab(text, table.get_config().get_tab_width());
            let text = truncate_text(&text, width, set_width, &suffix, suffix_color_try_keeping)
                .into_owned();

            let records = table.get_records_mut();
            records.set(pos, text, &width_ctrl);
        }

        table.destroy_width_cache();
    }
}

impl<W, P, R> TableOption<R> for Truncate<'_, W, P>
where
    W: WidthValue,
    P: ColumnPeaker,
    R: Records + RecordsMut<String>,
{
    fn change(&mut self, table: &mut Table<R>) {
        if table.is_empty() {
            return;
        }

        let width = get_width_value(&self.width, table);
        let (widths, total_width) =
            get_table_widths_with_total(table.get_records(), table.get_config());
        if total_width <= width {
            return;
        }

        let suffix = self.suffix.as_ref().map(|s| TruncateSuffix {
            limit: s.limit,
            text: Cow::Borrowed(&s.text),
            #[cfg(feature = "color")]
            try_color: s.try_color,
        });

        truncate_total_width(table, widths, total_width, width, suffix, P::create());
    }
}

fn truncate_text<'a>(
    content: &'a str,
    width: usize,
    original_width: usize,
    suffix: &'a str,
    _suffix_color_try_keeping: bool,
) -> Cow<'a, str> {
    if width == 0 {
        if original_width == 0 {
            Cow::Borrowed("")
        } else {
            Cow::Borrowed(suffix)
        }
    } else {
        let content = cut_str(content, width);

        if suffix.is_empty() {
            content
        } else {
            #[cfg(feature = "color")]
            {
                if _suffix_color_try_keeping {
                    if let Some(clr) = ansi_str::get_blocks(&content).last() {
                        if clr.has_ansi() {
                            Cow::Owned(format!("{}{}{}{}", content, clr.start(), suffix, clr.end()))
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

            #[cfg(not(feature = "color"))]
            {
                let mut content = content.into_owned();
                content.push_str(suffix);
                Cow::Owned(content)
            }
        }
    }
}

pub(crate) fn get_decrease_cell_list(
    cfg: &GridConfig,
    widths: &[usize],
    min_widths: &[usize],
    (count_rows, count_cols): (usize, usize),
) -> Vec<((usize, usize), usize)> {
    let mut points = Vec::new();
    (0..count_cols).for_each(|col| {
        (0..count_rows)
            .filter(|&row| cfg.is_cell_visible((row, col), (count_rows, count_cols)))
            .for_each(|row| {
                let (width, width_min) =
                    match cfg.get_column_span((row, col), (count_rows, count_cols)) {
                        Some(span) => {
                            let width = (col..col + span).map(|i| widths[i]).sum::<usize>();
                            let min_width = (col..col + span).map(|i| min_widths[i]).sum::<usize>();
                            let count_borders = count_borders(cfg, col, col + span, count_cols);
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

pub(crate) fn decrease_widths<F>(
    widths: &mut [usize],
    min_widths: &[usize],
    total_width: usize,
    mut width: usize,
    mut peeaker: F,
) where
    F: ColumnPeaker,
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

fn truncate_total_width<P, R>(
    table: &mut Table<R>,
    mut widths: Vec<usize>,
    widths_total: usize,
    width: usize,
    suffix: Option<TruncateSuffix<'_>>,
    priority: P,
) where
    P: ColumnPeaker,
    R: Records + RecordsMut<String>,
{
    let (count_rows, count_cols) = table.shape();
    let cfg = table.get_config();
    let min_widths = get_table_widths(EmptyRecords::new(count_rows, count_cols), cfg);

    decrease_widths(&mut widths, &min_widths, widths_total, width, priority);

    let points = get_decrease_cell_list(cfg, &widths, &min_widths, (count_rows, count_cols));

    table.destroy_width_cache();
    table.cache_width(widths);

    let mut truncate = Truncate::new(0);
    truncate.suffix = suffix;
    for ((row, col), width) in points {
        truncate.width = width;
        truncate.change_cell(table, (row, col).into());
    }
}

#[cfg(feature = "color")]
#[cfg(test)]
mod tests {
    use owo_colors::{colors::Yellow, OwoColorize};
    use papergrid::util::cut_str;

    #[test]
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
