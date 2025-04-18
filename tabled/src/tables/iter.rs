//! This module contains a [`IterTable`] table.
//!
//! In contrast to [`Table`] [`IterTable`] does no allocations but it consumes an iterator.
//! It's useful when you don't want to re/allocate a buffer for your data.
//!
//! # Example
//!
//! ```
//! use tabled::{grid::records::IterRecords, tables::IterTable};
//!
//! let iterator = vec![vec!["First", "row"], vec!["Second", "row"]];
//! let records = IterRecords::new(iterator, 2, Some(2));
//! let table = IterTable::new(records);
//!
//! let s = table.to_string();
//!
//! assert_eq!(
//!     s,
//!     "+--------+-----+\n\
//!      | First  | row |\n\
//!      +--------+-----+\n\
//!      | Second | row |\n\
//!      +--------+-----+",
//! );
//! ```
//!
//! [`Table`]: crate::Table

use std::{fmt, io};

use crate::{
    grid::{
        colors::NoColors,
        config::{AlignmentHorizontal, CompactConfig, Indent, Sides, SpannedConfig},
        dimension::{
            CompactGridDimension, Dimension, DimensionValue, StaticDimension, ZeroDimension,
        },
        records::{
            into_records::{BufRecords, LimitColumns, LimitRows, TruncateContent},
            IntoRecords, IterRecords,
        },
        IterGrid,
    },
    settings::{Style, TableOption},
};

use crate::util::utf8_writer::UTF8Writer;

/// A table which consumes an [`IntoRecords`] iterator.
///
/// To be able to build table we need a dimensions.
/// If no width and count_columns is set, [`IterTable`] will sniff the records, by
/// keeping a number of rows buffered (You can set the number via [`IterTable::sniff`]).
///
/// In contrast to [`Table`] [`IterTable`] does no allocations but it consumes an iterator.
/// It's useful when you don't want to re/allocate a buffer for your data.
///
/// # Example
///
/// ```
/// use tabled::{grid::records::IterRecords, tables::IterTable};
///
/// let data = vec![
///     vec!["First", "row"],
///     vec!["Second", "row"],
///     vec!["Third", "big row"],
/// ];
///
/// let records = IterRecords::new(data, 2, Some(2));
/// let table = IterTable::new(records).sniff(1);
///
/// // notice because of sniff 1 we have all rows after the first one being truncated
/// assert_eq!(
///     table.to_string(),
///     "+-------+-----+\n\
///      | First | row |\n\
///      +-------+-----+\n\
///      | Secon | row |\n\
///      +-------+-----+\n\
///      | Third | big |\n\
///      +-------+-----+",
/// );
/// ```
///
/// [`Table`]: crate::Table
#[derive(Debug, Clone)]
pub struct IterTable<I> {
    records: I,
    cfg: CompactConfig,
    table: Settings,
}

#[derive(Debug, Clone)]
struct Settings {
    sniff: usize,
    count_columns: Option<usize>,
    count_rows: Option<usize>,
    width: Option<usize>,
    height: Option<usize>,
}

impl<I> IterTable<I> {
    /// Creates a new [`IterTable`] structure.
    pub fn new(iter: I) -> Self
    where
        I: IntoRecords,
    {
        Self {
            records: iter,
            cfg: create_config(),
            table: Settings {
                sniff: 1000,
                count_columns: None,
                count_rows: None,
                height: None,
                width: None,
            },
        }
    }

    /// With is a generic function which applies options to the [`IterTable`].
    pub fn with<O>(mut self, option: O) -> Self
    where
        for<'a> O: TableOption<IterRecords<&'a I>, CompactConfig, ZeroDimension>,
    {
        let count_columns = self.table.count_columns.unwrap_or(0);
        let mut records = IterRecords::new(&self.records, count_columns, self.table.count_rows);
        let mut dims = ZeroDimension::new();
        option.change(&mut records, &mut self.cfg, &mut dims);

        self
    }

    /// Limit a number of columns.
    pub fn columns(mut self, count_columns: usize) -> Self {
        self.table.count_columns = Some(count_columns);
        self
    }

    /// Limit a number of rows.
    pub fn rows(mut self, count_rows: usize) -> Self {
        self.table.count_rows = Some(count_rows);
        self
    }

    /// Limit an amount of rows will be read for dimension estimations.
    ///
    /// By default it's 1000.
    pub fn sniff(mut self, count: usize) -> Self {
        self.table.sniff = count;
        self
    }

    /// Set a height for each row.
    pub fn height(mut self, size: usize) -> Self {
        self.table.height = Some(size);
        self
    }

    /// Set a width for each column.
    pub fn width(mut self, size: usize) -> Self {
        self.table.width = Some(size);
        self
    }

    /// Build a string.
    ///
    /// We can't implement [`std::string::ToString`] cause it does takes `&self` reference.
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(self) -> String
    where
        I: IntoRecords,
        I::Cell: AsRef<str>,
    {
        let mut buf = String::new();
        self.fmt(&mut buf)
            .expect("according to a doc is safe to fmt() a string");

        buf
    }

    /// Format table into [`io::Write`]r.
    pub fn build<W>(self, writer: W) -> io::Result<()>
    where
        W: io::Write,
        I: IntoRecords,
        I::Cell: AsRef<str>,
    {
        let writer = UTF8Writer::new(writer);
        self.fmt(writer)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
    }

    /// Format table into [fmt::Write]er.
    pub fn fmt<W>(self, writer: W) -> fmt::Result
    where
        W: fmt::Write,
        I: IntoRecords,
        I::Cell: AsRef<str>,
    {
        build_grid(writer, self.records, self.cfg, self.table)
    }
}

fn build_grid<W, I>(f: W, iter: I, cfg: CompactConfig, opts: Settings) -> fmt::Result
where
    W: fmt::Write,
    I: IntoRecords,
    I::Cell: AsRef<str>,
{
    let width_config = opts.width.is_some() && opts.count_columns.is_some();
    if width_config {
        build_table_with_static_dims(f, iter, cfg, opts)
    } else if opts.width.is_some() {
        build_table_sniffing_with_width(f, iter, cfg, opts)
    } else {
        build_table_sniffing(f, iter, cfg, opts)
    }
}

fn build_table_with_static_dims<W, I>(
    f: W,
    iter: I,
    cfg: CompactConfig,
    opts: Settings,
) -> fmt::Result
where
    W: fmt::Write,
    I: IntoRecords,
    I::Cell: AsRef<str>,
{
    let count_columns = opts.count_columns.unwrap();
    let width = opts.width.unwrap();
    let height = opts.height.unwrap_or(1);
    let contentw = WidthDimension::Exact(width);
    let pad = cfg.get_padding();
    let w = DimensionValue::Exact(width + pad.left.size + pad.right.size);
    let h = DimensionValue::Exact(height + pad.top.size + pad.bottom.size);
    let dims = StaticDimension::new(w, h);
    let cfg = SpannedConfig::from(cfg);

    match opts.count_rows {
        Some(limit) => {
            let records = LimitRows::new(iter, limit);
            let records = build_records(records, contentw, count_columns, Some(limit));
            IterGrid::new(records, dims, cfg, NoColors).build(f)
        }
        None => {
            let records = build_records(iter, contentw, count_columns, None);
            IterGrid::new(records, dims, cfg, NoColors).build(f)
        }
    }
}

fn build_table_sniffing<W, I>(f: W, iter: I, cfg: CompactConfig, opts: Settings) -> fmt::Result
where
    W: fmt::Write,
    I: IntoRecords,
    I::Cell: AsRef<str>,
{
    let records = BufRecords::new(iter, opts.sniff);

    let count_columns = get_count_columns(&opts, records.as_slice());

    let (mut width, height) = {
        let records = LimitColumns::new(records.as_slice(), count_columns);
        let records = IterRecords::new(records, count_columns, None);
        CompactGridDimension::dimension(records, &cfg)
    };

    let padding = cfg.get_padding();
    let pad = padding.left.size + padding.right.size;
    let padv = padding.top.size + padding.bottom.size;

    if opts.sniff == 0 {
        width = std::iter::repeat_n(pad, count_columns).collect::<Vec<_>>();
    }

    let content_width = WidthDimension::List(width.iter().map(|i| i.saturating_sub(pad)).collect());
    let dims_width = DimensionValue::List(width);

    let height_exact = opts.height.unwrap_or(1) + padv;
    let mut dims_height = DimensionValue::Partial(height, height_exact);

    if opts.height.is_some() {
        dims_height = DimensionValue::Exact(height_exact);
    }

    let dims = StaticDimension::new(dims_width, dims_height);
    let cfg = SpannedConfig::from(cfg);

    match opts.count_rows {
        Some(limit) => {
            let records = LimitRows::new(records, limit);
            let records = build_records(records, content_width, count_columns, Some(limit));
            IterGrid::new(records, dims, cfg, NoColors).build(f)
        }
        None => {
            let records = build_records(records, content_width, count_columns, None);
            IterGrid::new(records, dims, cfg, NoColors).build(f)
        }
    }
}

fn build_table_sniffing_with_width<W, I>(
    f: W,
    iter: I,
    cfg: CompactConfig,
    opts: Settings,
) -> fmt::Result
where
    W: fmt::Write,
    I: IntoRecords,
    I::Cell: AsRef<str>,
{
    let records = BufRecords::new(iter, opts.sniff);

    let count_columns = get_count_columns(&opts, records.as_slice());

    let width = opts.width.unwrap();
    let contentw = WidthDimension::Exact(width);

    let padding = cfg.get_padding();
    let pad = padding.left.size + padding.right.size;
    let padv = padding.top.size + padding.bottom.size;

    let height = opts.height.unwrap_or(1) + padv;
    let dimsh = DimensionValue::Exact(height);
    let dimsw = DimensionValue::Exact(width + pad);
    let dims = StaticDimension::new(dimsw, dimsh);

    let cfg = SpannedConfig::from(cfg);

    match opts.count_rows {
        Some(limit) => {
            let records = LimitRows::new(records, limit);
            let records = build_records(records, contentw, count_columns, Some(limit));
            IterGrid::new(records, dims, cfg, NoColors).build(f)
        }
        None => {
            let records = build_records(records, contentw, count_columns, None);
            IterGrid::new(records, dims, cfg, NoColors).build(f)
        }
    }
}

fn get_count_columns<T>(opts: &Settings, buf: &[Vec<T>]) -> usize {
    match opts.count_columns {
        Some(size) => size,
        None => buf.iter().map(|row| row.len()).max().unwrap_or(0),
    }
}

const fn create_config() -> CompactConfig {
    CompactConfig::new()
        .set_padding(Sides::new(
            Indent::spaced(1),
            Indent::spaced(1),
            Indent::zero(),
            Indent::zero(),
        ))
        .set_alignment_horizontal(AlignmentHorizontal::Left)
        .set_borders(Style::ascii().get_borders())
}

fn build_records<I>(
    records: I,
    width: WidthDimension,
    count_columns: usize,
    count_rows: Option<usize>,
) -> IterRecords<LimitColumns<TruncateContent<I, WidthDimension>>>
where
    I: IntoRecords,
{
    let records = TruncateContent::new(records, width);
    let records = LimitColumns::new(records, count_columns);
    IterRecords::new(records, count_columns, count_rows)
}

/// A dimension value.
#[derive(Debug, Clone)]
enum WidthDimension {
    Exact(usize),
    List(Vec<usize>),
}

impl Dimension for WidthDimension {
    fn get_width(&self, column: usize) -> usize {
        match self {
            WidthDimension::Exact(value) => *value,
            WidthDimension::List(list) => list[column],
        }
    }

    fn get_height(&self, _row: usize) -> usize {
        unreachable!("A height method is not supposed to be called");
    }
}
