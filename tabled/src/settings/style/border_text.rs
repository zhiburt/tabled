use crate::{
    grid::{
        ansi::ANSIBuf,
        config::{self, ColoredConfig, SpannedConfig},
        dimension::{Dimension, Estimate},
        records::{ExactRecords, Records},
    },
    settings::{
        object::{Column, FirstColumn, FirstRow, LastColumn, LastRow, Row},
        Color, TableOption,
    },
};

use super::Offset;

/// [`LineText`] writes a custom text on a border.
///
/// # Example
///
/// ```rust
/// use tabled::{Table, settings::style::LineText, settings::object::Rows};
///
/// let mut table = Table::new(["Hello World"]);
/// table.with(LineText::new("+-.table", Rows::first()));
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
pub struct LineText<L> {
    // todo: change to T and specify to be As<str>
    text: String,
    offset: Offset,
    color: Option<ANSIBuf>,
    line: L,
}

impl<Line> LineText<Line> {
    /// Creates a [`LineText`] instance.
    ///
    /// Lines are numbered from 0 to the `count_rows` included
    /// (`line >= 0 && line <= count_rows`).
    pub fn new<S>(text: S, line: Line) -> Self
    where
        S: Into<String>,
    {
        LineText {
            text: text.into(),
            line,
            offset: Offset::Begin(0),
            color: None,
        }
    }

    /// Set an offset from which the text will be started.
    pub fn offset(self, offset: impl Into<Offset>) -> Self {
        LineText {
            offset: offset.into(),
            text: self.text,
            line: self.line,
            color: self.color,
        }
    }

    /// Set a color of the text.
    pub fn color(self, color: Color) -> Self {
        LineText {
            color: Some(color.into()),
            text: self.text,
            line: self.line,
            offset: self.offset,
        }
    }
}

impl<R, D> TableOption<R, ColoredConfig, D> for LineText<Row>
where
    R: Records + ExactRecords,
    for<'a> &'a R: Records,
    for<'a> D: Estimate<&'a R, ColoredConfig>,
    D: Dimension,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, dims: &mut D) {
        let line = self.line.into();
        change_horizontal_chars(records, dims, cfg, line, self.text, self.offset, self.color)
    }
}

impl<R, D> TableOption<R, ColoredConfig, D> for LineText<FirstRow>
where
    R: Records + ExactRecords,
    for<'a> &'a R: Records,
    for<'a> D: Estimate<&'a R, ColoredConfig>,
    D: Dimension,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, dims: &mut D) {
        change_horizontal_chars(records, dims, cfg, 0, self.text, self.offset, self.color)
    }
}

impl<R, D> TableOption<R, ColoredConfig, D> for LineText<LastRow>
where
    R: Records + ExactRecords,
    for<'a> &'a R: Records,
    for<'a> D: Estimate<&'a R, ColoredConfig>,
    D: Dimension,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, dims: &mut D) {
        let line = records.count_rows();
        change_horizontal_chars(records, dims, cfg, line, self.text, self.offset, self.color)
    }
}

impl<R, D> TableOption<R, ColoredConfig, D> for LineText<Column>
where
    R: Records + ExactRecords,
    for<'a> &'a R: Records,
    for<'a> D: Estimate<&'a R, ColoredConfig>,
    D: Dimension,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, dims: &mut D) {
        let line = self.line.into();
        change_vertical_chars(records, dims, cfg, line, self.text, self.offset, self.color)
    }
}

impl<R, D> TableOption<R, ColoredConfig, D> for LineText<FirstColumn>
where
    R: Records + ExactRecords,
    for<'a> &'a R: Records,
    for<'a> D: Estimate<&'a R, ColoredConfig>,
    D: Dimension,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, dims: &mut D) {
        change_vertical_chars(records, dims, cfg, 0, self.text, self.offset, self.color)
    }
}

impl<R, D> TableOption<R, ColoredConfig, D> for LineText<LastColumn>
where
    R: Records + ExactRecords,
    for<'a> &'a R: Records,
    for<'a> D: Estimate<&'a R, ColoredConfig>,
    D: Dimension,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, dims: &mut D) {
        let line = records.count_rows();
        change_vertical_chars(records, dims, cfg, line, self.text, self.offset, self.color)
    }
}

fn set_horizontal_chars<D: Dimension>(
    cfg: &mut SpannedConfig,
    dims: &D,
    offset: Offset,
    line: usize,
    text: &str,
    color: &Option<ANSIBuf>,
    shape: (usize, usize),
) {
    let (_, count_columns) = shape;
    let total_width = total_width(cfg, dims, count_columns);
    let pos = get_start_pos(offset, total_width);

    let pos = match pos {
        Some(pos) => pos,
        None => return,
    };

    let mut chars = text.chars();
    let mut i = cfg.has_vertical(0, count_columns) as usize;
    if i == 1 && pos == 0 {
        let c = match chars.next() {
            Some(c) => c,
            None => return,
        };

        let mut b = cfg.get_border((line, 0), shape);
        b.left_top_corner = b.left_top_corner.map(|_| c);
        cfg.set_border((line, 0), b);

        if let Some(color) = color.as_ref() {
            let mut b = cfg.get_border_color((line, 0), shape).cloned();
            b.left_top_corner = Some(color.clone());
            cfg.set_border_color((line, 0), b);
        }
    }

    for col in 0..count_columns {
        let w = dims.get_width(col);
        if i + w > pos {
            for off in 0..w {
                if i + off < pos {
                    continue;
                }

                let c = match chars.next() {
                    Some(c) => c,
                    None => return,
                };

                cfg.set_horizontal_char((line, col), c, config::Offset::Begin(off));
                if let Some(color) = color.as_ref() {
                    cfg.set_horizontal_color(
                        (line, col),
                        color.clone(),
                        config::Offset::Begin(off),
                    );
                }
            }
        }

        i += w;

        if cfg.has_vertical(col + 1, count_columns) {
            i += 1;

            if i > pos {
                let c = match chars.next() {
                    Some(c) => c,
                    None => return,
                };

                let mut b = cfg.get_border((line, col), shape);
                b.right_top_corner = b.right_top_corner.map(|_| c);
                cfg.set_border((line, col), b);

                if let Some(color) = color.as_ref() {
                    let mut b = cfg.get_border_color((line, col), shape).cloned();
                    b.right_top_corner = Some(color.clone());
                    cfg.set_border_color((line, col), b);
                }
            }
        }
    }
}

fn set_vertical_chars<D>(
    cfg: &mut SpannedConfig,
    dims: &D,
    offset: Offset,
    line: usize,
    text: &str,
    color: &Option<ANSIBuf>,
    shape: (usize, usize),
) where
    D: Dimension,
{
    let (count_rows, _) = shape;
    let total_width = total_height(cfg, dims, count_rows);
    let pos = get_start_pos(offset, total_width);

    let pos = match pos {
        Some(pos) => pos,
        None => return,
    };

    let mut chars = text.chars();
    let mut i = cfg.has_horizontal(0, count_rows) as usize;
    if i == 1 && pos == 0 {
        let c = match chars.next() {
            Some(c) => c,
            None => return,
        };

        let mut b = cfg.get_border((0, line), shape);
        b.left_top_corner = b.left_top_corner.map(|_| c);
        cfg.set_border((0, line), b);

        if let Some(color) = color.as_ref() {
            let mut b = cfg.get_border_color((0, line), shape).cloned();
            b.left_top_corner = Some(color.clone());
            cfg.set_border_color((0, line), b);
        }
    }

    for row in 0..count_rows {
        let row_height = dims.get_height(row);
        if i + row_height > pos {
            for off in 0..row_height {
                if i + off < pos {
                    continue;
                }

                let c = match chars.next() {
                    Some(c) => c,
                    None => return,
                };

                cfg.set_vertical_char((row, line), c, config::Offset::Begin(off)); // todo: is this correct? I thik it shall be off + i

                if let Some(color) = color.as_ref() {
                    cfg.set_vertical_color((row, line), color.clone(), config::Offset::Begin(off));
                }
            }
        }

        i += row_height;

        if cfg.has_horizontal(row + 1, count_rows) {
            i += 1;

            if i > pos {
                let c = match chars.next() {
                    Some(c) => c,
                    None => return,
                };

                let mut b = cfg.get_border((row, line), shape);
                b.left_bottom_corner = b.left_bottom_corner.map(|_| c);
                cfg.set_border((row, line), b);

                if let Some(color) = color.as_ref() {
                    let mut b = cfg.get_border_color((row, line), shape).cloned();
                    b.left_bottom_corner = Some(color.clone());
                    cfg.set_border_color((row, line), b);
                }
            }
        }
    }
}

fn get_start_pos(offset: Offset, total: usize) -> Option<usize> {
    match offset {
        Offset::Begin(i) => {
            if i > total {
                None
            } else {
                Some(i)
            }
        }
        Offset::End(i) => {
            if i > total {
                None
            } else {
                Some(total - i)
            }
        }
    }
}

fn total_width<D>(cfg: &SpannedConfig, dims: &D, count_columns: usize) -> usize
where
    D: Dimension,
{
    let mut total = cfg.has_vertical(count_columns, count_columns) as usize;
    for col in 0..count_columns {
        total += dims.get_width(col);
        total += cfg.has_vertical(col, count_columns) as usize;
    }

    total
}

fn total_height<D>(cfg: &SpannedConfig, dims: &D, count_rows: usize) -> usize
where
    D: Dimension,
{
    let mut total = cfg.has_horizontal(count_rows, count_rows) as usize;
    for row in 0..count_rows {
        total += dims.get_height(row);
        total += cfg.has_horizontal(row, count_rows) as usize;
    }

    total
}

fn change_horizontal_chars<R, D>(
    records: &mut R,
    dims: &mut D,
    cfg: &mut ColoredConfig,
    line: usize,
    text: String,
    offset: Offset,
    color: Option<ANSIBuf>,
) where
    R: Records + ExactRecords,
    for<'a> D: Estimate<&'a R, ColoredConfig>,
    D: Dimension,
{
    dims.estimate(records, cfg);
    let shape = (records.count_rows(), records.count_columns());
    set_horizontal_chars(cfg, dims, offset, line, &text, &color, shape);
}

fn change_vertical_chars<R, D>(
    records: &mut R,
    dims: &mut D,
    cfg: &mut ColoredConfig,
    line: usize,
    text: String,
    offset: Offset,
    color: Option<ANSIBuf>,
) where
    R: Records + ExactRecords,
    for<'a> D: Estimate<&'a R, ColoredConfig>,
    D: Dimension,
{
    dims.estimate(records, cfg);
    let shape = (records.count_rows(), records.count_columns());
    set_vertical_chars(cfg, dims, offset, line, &text, &color, shape);
}
