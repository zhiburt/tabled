use std::borrow::Cow;

use crate::{
    grid::{
        config::spanned::{self, SpannedConfig},
        dimension::{Dimension, Estimate},
    },
    records::{ExactRecords, Records},
    settings::TableOption,
    tables::table::{ColoredConfig, TableDimension},
};

use super::Offset;

/// [`BorderText`] writes a custom text on a border.
///
/// # Example
///
/// ```rust
/// use tabled::{Table, settings::style::BorderText};
///
/// let mut table = Table::new(["Hello World"]);
/// table
///     .with(BorderText::first("+-.table"));
///
/// assert_eq!(
///     table.to_string(),
///     "+-.table------+\n\
///      | &str        |\n\
///      +-------------+\n\
///      | Hello World |\n\
///      +-------------+"
/// );
/// ```
#[derive(Debug)]
pub struct BorderText<'a, Line> {
    text: Cow<'a, str>,
    offset: Offset,
    line: Line,
}

#[derive(Debug)]
pub struct LineIndex(usize);

#[derive(Debug)]
pub struct LineFirst;

#[derive(Debug)]
pub struct LineLast;

impl<'a> BorderText<'a, ()> {
    /// Creates a [`BorderText`] instance.
    ///
    /// Lines are numbered from 0 to the `count_rows` included
    /// (`line >= 0 && line <= count_rows`).
    pub fn new<S: Into<Cow<'a, str>>>(line: usize, text: S) -> BorderText<'a, LineIndex> {
        BorderText::create(text.into(), Offset::Begin(0), LineIndex(line))
    }

    /// Creates a [`BorderText`] instance for a top line.
    pub fn first<S: Into<Cow<'a, str>>>(text: S) -> BorderText<'a, LineFirst> {
        BorderText::create(text.into(), Offset::Begin(0), LineFirst)
    }

    /// Creates a [`BorderText`] instance for a bottom line.
    pub fn last<S: Into<Cow<'a, str>>>(text: S) -> BorderText<'a, LineLast> {
        BorderText::create(text.into(), Offset::Begin(0), LineLast)
    }

    fn create<L>(text: Cow<'a, str>, offset: Offset, line: L) -> BorderText<'a, L> {
        BorderText { text, line, offset }
    }
}

impl<L> BorderText<'_, L> {
    /// Set an offset from which the text will be started.
    pub fn offset(mut self, offset: Offset) -> Self {
        self.offset = offset;
        self
    }
}

impl<R> TableOption<R, TableDimension<'_>, ColoredConfig> for BorderText<'_, LineIndex>
where
    R: Records + ExactRecords,
    for<'a> &'a R: Records,
{
    fn change(&mut self, records: &mut R, cfg: &mut ColoredConfig, dims: &mut TableDimension<'_>) {
        set_chars(dims, &*records, cfg, self.offset, self.line.0, &self.text);
    }
}

impl<R> TableOption<R, TableDimension<'_>, ColoredConfig> for BorderText<'_, LineFirst>
where
    R: Records + ExactRecords,
    for<'a> &'a R: Records,
{
    fn change(&mut self, records: &mut R, cfg: &mut ColoredConfig, dims: &mut TableDimension<'_>) {
        set_chars(dims, &*records, cfg, self.offset, 0, &self.text);
    }
}

impl<R> TableOption<R, TableDimension<'_>, ColoredConfig> for BorderText<'_, LineLast>
where
    R: Records + ExactRecords,
    for<'a> &'a R: Records,
{
    fn change(&mut self, records: &mut R, cfg: &mut ColoredConfig, dims: &mut TableDimension<'_>) {
        set_chars(
            dims,
            &*records,
            cfg,
            self.offset,
            records.count_rows(),
            &self.text,
        );
    }
}

fn set_chars<R>(
    dims: &mut TableDimension<'_>,
    records: &R,
    cfg: &mut SpannedConfig,
    offset: Offset,
    line: usize,
    text: &str,
) where
    for<'a> &'a R: Records + ExactRecords,
{
    dims.estimate(records, cfg);

    let count_columns = records.count_columns();
    let count_rows = records.count_rows();
    let pos = get_start_pos(offset, cfg, dims, count_columns);
    let pos = match pos {
        Some(pos) => pos,
        None => return,
    };

    let mut chars = text.chars();
    let mut i = cfg.has_vertical(0, count_columns) as usize;
    if i == 1 && pos == 0 {
        match chars.next() {
            Some(c) => {
                let mut b = cfg.get_border((line, 0), (count_rows, count_columns));
                b.left_top_corner = b.left_top_corner.map(|_| c);
                cfg.set_border((line, 0), b);
            }
            None => return,
        }
    }

    for col in 0..count_columns {
        let w = dims.get_width(col);
        if i + w > pos {
            for off in 0..w {
                if i + off < pos {
                    continue;
                }

                match chars.next() {
                    Some(c) => {
                        cfg.override_horizontal_border((line, col), c, spanned::Offset::Begin(off))
                    }
                    None => return,
                }
            }
        }

        i += w;

        if cfg.has_vertical(col + 1, count_columns) {
            i += 1;

            if i > pos {
                match chars.next() {
                    Some(c) => {
                        let mut b = cfg.get_border((line, col), (count_rows, count_columns));
                        b.right_top_corner = b.right_top_corner.map(|_| c);
                        cfg.set_border((line, col), b);
                    }
                    None => return,
                }
            }
        }
    }
}

fn get_start_pos(
    offset: Offset,
    cfg: &SpannedConfig,
    dims: &TableDimension<'_>,
    count_columns: usize,
) -> Option<usize> {
    let totalw = total_width(cfg, dims, count_columns);
    match offset {
        Offset::Begin(i) => {
            if i > totalw {
                None
            } else {
                Some(i)
            }
        }
        Offset::End(i) => {
            if i > totalw {
                None
            } else {
                Some(totalw - i)
            }
        }
    }
}

fn total_width(cfg: &SpannedConfig, dims: &TableDimension<'_>, count_columns: usize) -> usize {
    let mut totalw = cfg.has_vertical(0, count_columns) as usize;
    for col in 0..count_columns {
        totalw += dims.get_width(col);
        totalw += cfg.has_vertical(col + 1, count_columns) as usize;
    }

    totalw
}
