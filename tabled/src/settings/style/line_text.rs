use papergrid::config::{AlignmentHorizontal, AlignmentVertical};

use crate::{
    grid::{
        ansi::ANSIBuf,
        config::{self, ColoredConfig, Entity, SpannedConfig},
        dimension::{Dimension, Estimate},
        records::{ExactRecords, Records},
        util::string::get_text_width,
    },
    settings::{
        object::{
            Column, FirstColumn, FirstRow, LastColumn, LastColumnOffset, LastRow, LastRowOffset,
            Object, Row,
        },
        Alignment, Color, TableOption,
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
pub struct LineText<Line> {
    // todo: change to T and specify to be As<str>
    text: String,
    offset: Offset,
    color: Option<ANSIBuf>,
    alignment: Option<Alignment>,
    line: Line,
}

impl<Line> LineText<Line> {
    /// Creates a [`LineText`] instance.
    ///
    /// Line can be a column or a row.
    /// Lines are numbered from 0 to the `count_rows`/`count_columns` included:
    /// (`line >= 0 && line <= count_rows`)
    /// (`line >= 0 && line <= count_columns`).
    ///
    /// ```
    /// use tabled::{Table, settings::style::LineText, settings::object::Columns};
    ///
    /// let mut table = Table::new(["Hello World"]);
    /// table.with(LineText::new("TABLE", Columns::single(0)));
    /// table.with(LineText::new("TABLE", Columns::single(1)));
    ///
    /// assert_eq!(
    ///     table.to_string(),
    ///     "T-------------T\n\
    ///      A &str        A\n\
    ///      B-------------B\n\
    ///      L Hello World L\n\
    ///      E-------------E"
    /// );
    /// ```
    pub fn new<S>(text: S, line: Line) -> Self
    where
        S: Into<String>,
    {
        LineText {
            line,
            text: text.into(),
            offset: Offset::Begin(0),
            color: None,
            alignment: None,
        }
    }

    /// Set an offset from which the text will be started.
    ///
    /// ```
    /// use tabled::Table;
    /// use tabled::settings::{Alignment, style::LineText, object::Rows};
    ///
    /// let mut table = Table::new(["Hello World"]);
    /// table.with(LineText::new("TABLE", Rows::first()).align(Alignment::center()));
    ///
    /// assert_eq!(
    ///     table.to_string(),
    ///     "+----TABLE----+\n\
    ///      | &str        |\n\
    ///      +-------------+\n\
    ///      | Hello World |\n\
    ///      +-------------+"
    /// );
    /// ```
    pub fn align(mut self, alignment: Alignment) -> Self {
        self.alignment = Some(alignment);
        self
    }

    /// Set an offset from which the text will be started.
    ///
    /// ```
    /// use tabled::{Table, settings::style::LineText, settings::object::Rows};
    ///
    /// let mut table = Table::new(["Hello World"]);
    /// table.with(LineText::new("TABLE", Rows::first()).offset(3));
    ///
    /// assert_eq!(
    ///     table.to_string(),
    ///     "+--TABLE------+\n\
    ///      | &str        |\n\
    ///      +-------------+\n\
    ///      | Hello World |\n\
    ///      +-------------+"
    /// );
    /// ```
    pub fn offset(mut self, offset: impl Into<Offset>) -> Self {
        self.offset = offset.into();
        self
    }

    /// Set a color of the text.
    ///
    /// ```
    /// use tabled::Table;
    /// use tabled::settings::{object::Rows, Color, style::LineText};
    ///
    /// let mut table = Table::new(["Hello World"]);
    /// table.with(LineText::new("TABLE", Rows::first()).color(Color::FG_BLUE));
    ///
    /// assert_eq!(
    ///     table.to_string(),
    ///     "\u{1b}[34mT\u{1b}[39m\u{1b}[34mA\u{1b}[39m\u{1b}[34mB\u{1b}[39m\u{1b}[34mL\u{1b}[39m\u{1b}[34mE\u{1b}[39m---------+\n\
    ///      | &str        |\n\
    ///      +-------------+\n\
    ///      | Hello World |\n\
    ///      +-------------+"
    /// );
    /// ```
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color.into());
        self
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
        change_horizontal_chars(records, dims, cfg, create_line(self, line))
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
        change_horizontal_chars(records, dims, cfg, create_line(self, 0))
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
        let line = self.line.cells(records).next();
        if let Some(Entity::Row(line)) = line {
            change_horizontal_chars(records, dims, cfg, create_line(self, line))
        }
    }
}

impl<R, D> TableOption<R, ColoredConfig, D> for LineText<LastRowOffset>
where
    R: Records + ExactRecords,
    for<'a> &'a R: Records,
    for<'a> D: Estimate<&'a R, ColoredConfig>,
    D: Dimension,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, dims: &mut D) {
        let line = self.line.cells(records).next();
        if let Some(Entity::Row(line)) = line {
            change_horizontal_chars(records, dims, cfg, create_line(self, line))
        }
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
        change_vertical_chars(records, dims, cfg, create_line(self, line))
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
        change_vertical_chars(records, dims, cfg, create_line(self, 0))
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
        change_vertical_chars(records, dims, cfg, create_line(self, line))
    }
}

impl<R, D> TableOption<R, ColoredConfig, D> for LineText<LastColumnOffset>
where
    R: Records + ExactRecords,
    for<'a> &'a R: Records,
    for<'a> D: Estimate<&'a R, ColoredConfig>,
    D: Dimension,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, dims: &mut D) {
        let line = self.line.cells(records).next();
        if let Some(Entity::Column(line)) = line {
            change_vertical_chars(records, dims, cfg, create_line(self, line))
        }
    }
}

fn set_horizontal_chars<D>(
    cfg: &mut SpannedConfig,
    dims: &D,
    line: LineText<usize>,
    shape: (usize, usize),
) where
    D: Dimension,
{
    let alignment = line.alignment.and_then(|a| a.as_horizontal());
    let offset = line.offset;
    let text = &line.text;
    let color = &line.color;
    let line = line.line;

    let (_, count_columns) = shape;
    let total_width = total_width(cfg, dims, count_columns);

    let offset = match alignment {
        Some(alignment) => {
            let off = get_horizontal_alignment_offset(text, alignment, total_width);
            offset_sum(off, offset)
        }
        None => offset,
    };

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

        let mut b = cfg.get_border((line, 0).into(), shape);
        b.left_top_corner = b.left_top_corner.map(|_| c);
        cfg.set_border((line, 0).into(), b);

        if let Some(color) = color.as_ref() {
            let mut b = cfg.get_border_color((line, 0).into(), shape).cloned();
            b.left_top_corner = Some(color.clone());
            cfg.set_border_color((line, 0).into(), b);
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

                cfg.set_horizontal_char((line, col).into(), c, config::Offset::Begin(off));
                if let Some(color) = color.as_ref() {
                    cfg.set_horizontal_color(
                        (line, col).into(),
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

                let mut b = cfg.get_border((line, col).into(), shape);
                b.right_top_corner = b.right_top_corner.map(|_| c);
                cfg.set_border((line, col).into(), b);

                if let Some(color) = color.as_ref() {
                    let mut b = cfg.get_border_color((line, col).into(), shape).cloned();
                    b.right_top_corner = Some(color.clone());
                    cfg.set_border_color((line, col).into(), b);
                }
            }
        }
    }
}

fn set_vertical_chars<D>(
    cfg: &mut SpannedConfig,
    dims: &D,
    line: LineText<usize>,
    shape: (usize, usize),
) where
    D: Dimension,
{
    let alignment = line.alignment.and_then(|a| a.as_vertical());
    let offset = line.offset;
    let text = &line.text;
    let color = &line.color;
    let line = line.line;

    let (count_rows, _) = shape;
    let total_width = total_height(cfg, dims, count_rows);

    let offset = match alignment {
        Some(alignment) => {
            let off = get_vertical_alignment_offset(text, alignment, total_width);
            offset_sum(off, offset)
        }
        None => offset,
    };

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

        let mut b = cfg.get_border((0, line).into(), shape);
        b.left_top_corner = b.left_top_corner.map(|_| c);
        cfg.set_border((0, line).into(), b);

        if let Some(color) = color.as_ref() {
            let mut b = cfg.get_border_color((0, line).into(), shape).cloned();
            b.left_top_corner = Some(color.clone());
            cfg.set_border_color((0, line).into(), b);
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

                cfg.set_vertical_char((row, line).into(), c, config::Offset::Begin(off)); // todo: is this correct? I think it shall be off + i

                if let Some(color) = color.as_ref() {
                    cfg.set_vertical_color(
                        (row, line).into(),
                        color.clone(),
                        config::Offset::Begin(off),
                    );
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

                let mut b = cfg.get_border((row, line).into(), shape);
                b.left_bottom_corner = b.left_bottom_corner.map(|_| c);
                cfg.set_border((row, line).into(), b);

                if let Some(color) = color.as_ref() {
                    let mut b = cfg.get_border_color((row, line).into(), shape).cloned();
                    b.left_bottom_corner = Some(color.clone());
                    cfg.set_border_color((row, line).into(), b);
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

fn get_horizontal_alignment_offset(
    text: &str,
    alignment: AlignmentHorizontal,
    total: usize,
) -> Offset {
    match alignment {
        AlignmentHorizontal::Center => {
            let width = get_text_width(text);
            let mut off = 0;
            if total > width {
                let center = total / 2;
                let text_center = width / 2;
                off = center.saturating_sub(text_center);
            }

            Offset::Begin(off)
        }
        AlignmentHorizontal::Left => Offset::Begin(0),
        AlignmentHorizontal::Right => Offset::End(0),
    }
}

fn get_vertical_alignment_offset(text: &str, alignment: AlignmentVertical, total: usize) -> Offset {
    match alignment {
        AlignmentVertical::Center => {
            let width = get_text_width(text);
            let mut off = 0;
            if total > width {
                let center = total / 2;
                let text_center = width / 2;
                off = center.saturating_sub(text_center);
            }

            Offset::Begin(off)
        }
        AlignmentVertical::Top => Offset::Begin(0),
        AlignmentVertical::Bottom => Offset::End(0),
    }
}

fn offset_sum(orig: Offset, and: Offset) -> Offset {
    match (orig, and) {
        (Offset::Begin(a), Offset::Begin(b)) => Offset::Begin(a + b),
        (Offset::Begin(a), Offset::End(b)) => Offset::Begin(a.saturating_sub(b)),
        (Offset::End(a), Offset::Begin(b)) => Offset::End(a + b),
        (Offset::End(a), Offset::End(b)) => Offset::End(a.saturating_sub(b)),
    }
}

// todo: Can be move all the estimation function to util or somewhere cause I am sure it's not first place it's defined/used.
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
    line: LineText<usize>,
) where
    R: Records + ExactRecords,
    for<'a> D: Estimate<&'a R, ColoredConfig>,
    D: Dimension,
{
    dims.estimate(records, cfg);
    let shape = (records.count_rows(), records.count_columns());
    set_horizontal_chars(cfg, dims, line, shape);
}

fn change_vertical_chars<R, D>(
    records: &mut R,
    dims: &mut D,
    cfg: &mut ColoredConfig,
    line: LineText<usize>,
) where
    R: Records + ExactRecords,
    for<'a> D: Estimate<&'a R, ColoredConfig>,
    D: Dimension,
{
    dims.estimate(records, cfg);
    let shape = (records.count_rows(), records.count_columns());
    set_vertical_chars(cfg, dims, line, shape);
}

fn create_line<T>(orig: LineText<T>, line: usize) -> LineText<usize> {
    LineText {
        text: orig.text,
        offset: orig.offset,
        color: orig.color,
        alignment: orig.alignment,
        line,
    }
}
