use crate::{
    grid::{
        color::AnsiColor,
        config::{self, ColoredConfig, SpannedConfig},
        dimension::{Dimension, Estimate},
        records::{ExactRecords, Records},
    },
    settings::{
        object::{FirstRow, LastRow},
        Color, TableOption,
    },
};

use super::Offset;

/// [`BorderText`] writes a custom text on a border.
///
/// # Example
///
/// ```rust
/// use tabled::{Table, settings::style::BorderText, settings::object::Rows};
///
/// let mut table = Table::new(["Hello World"]);
/// table
///     .with(BorderText::new("+-.table").horizontal(Rows::first()));
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
pub struct BorderText<L> {
    text: String,
    offset: Offset,
    color: Option<AnsiColor<'static>>,
    line: L,
}

impl BorderText<()> {
    /// Creates a [`BorderText`] instance.
    ///
    /// Lines are numbered from 0 to the `count_rows` included
    /// (`line >= 0 && line <= count_rows`).
    pub fn new<S: Into<String>>(text: S) -> Self {
        BorderText {
            text: text.into(),
            line: (),
            offset: Offset::Begin(0),
            color: None,
        }
    }
}

impl<Line> BorderText<Line> {
    /// Set a line on which we will set the text.
    pub fn horizontal<L>(self, line: L) -> BorderText<L> {
        BorderText {
            line,
            text: self.text,
            offset: self.offset,
            color: self.color,
        }
    }

    /// Set an offset from which the text will be started.
    pub fn offset(self, offset: Offset) -> Self {
        BorderText {
            offset,
            text: self.text,
            line: self.line,
            color: self.color,
        }
    }

    /// Set a color of the text.
    pub fn color(self, color: Color) -> Self {
        BorderText {
            color: Some(color.into()),
            text: self.text,
            line: self.line,
            offset: self.offset,
        }
    }
}

impl<R, D> TableOption<R, D, ColoredConfig> for BorderText<usize>
where
    R: Records + ExactRecords,
    for<'a> &'a R: Records,
    for<'a> D: Estimate<&'a R, ColoredConfig>,
    D: Dimension,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, dims: &mut D) {
        dims.estimate(records, cfg);
        let shape = (records.count_rows(), records.count_columns());
        let line = self.line;
        set_horizontal_chars(cfg, dims, self.offset, line, &self.text, &self.color, shape);
    }
}

impl<R, D> TableOption<R, D, ColoredConfig> for BorderText<FirstRow>
where
    R: Records + ExactRecords,
    for<'a> &'a R: Records,
    for<'a> D: Estimate<&'a R, ColoredConfig>,
    D: Dimension,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, dims: &mut D) {
        dims.estimate(records, cfg);
        let shape = (records.count_rows(), records.count_columns());
        let line = 0;
        set_horizontal_chars(cfg, dims, self.offset, line, &self.text, &self.color, shape);
    }
}

impl<R, D> TableOption<R, D, ColoredConfig> for BorderText<LastRow>
where
    R: Records + ExactRecords,
    for<'a> &'a R: Records,
    for<'a> D: Estimate<&'a R, ColoredConfig>,
    D: Dimension,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, dims: &mut D) {
        dims.estimate(records, cfg);
        let shape = (records.count_rows(), records.count_columns());
        let line = records.count_rows();
        set_horizontal_chars(cfg, dims, self.offset, line, &self.text, &self.color, shape);
    }
}

fn set_horizontal_chars<D: Dimension>(
    cfg: &mut SpannedConfig,
    dims: &D,
    offset: Offset,
    line: usize,
    text: &str,
    color: &Option<AnsiColor<'static>>,
    shape: (usize, usize),
) {
    let (_, count_columns) = shape;
    let pos = get_start_pos(cfg, dims, offset, count_columns);
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

fn get_start_pos<D: Dimension>(
    cfg: &SpannedConfig,
    dims: &D,
    offset: Offset,
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

fn total_width<D: Dimension>(cfg: &SpannedConfig, dims: &D, count_columns: usize) -> usize {
    let mut totalw = cfg.has_vertical(0, count_columns) as usize;
    for col in 0..count_columns {
        totalw += dims.get_width(col);
        totalw += cfg.has_vertical(col + 1, count_columns) as usize;
    }

    totalw
}
