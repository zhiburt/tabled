//! This module contains a [`IterTable`] table.
//!
//! In contrast to [`Table`] [`IterTable`] does no allocations but it consumes an iterator.
//! It's useful when you don't want to re/allocate a buffer for your data.
//!
//! # Example
//!
//! ```
//! use tabled::{records::IterRecords, tables::iter::IterTable};
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

mod dimension;
pub(crate) mod utf8_writer;

use std::{cmp, fmt, io};

use crate::{
    grid::{
        colors::NoColors,
        config::compact::CompactConfig,
        config::spanned::SpannedConfig,
        config::{AlignmentHorizontal, Indent, Sides},
        dimension::compact::ExactDimension,
        dimension::Dimension,
        iterable::Grid,
    },
    records::{
        into_records::{
            truncate_records::ExactValue, BufColumns, BufRows, LimitColumns, LimitRows,
            TruncateContent,
        },
        IntoRecords, IterRecords,
    },
    settings::{Style, TableOption},
};

use self::dimension::ExactList;
use self::{dimension::IterTableDimension, utf8_writer::UTF8Writer};

/// A table which consumes an [`IntoRecords`] iterator.
///
/// To be able to build table we need a dimensions.
/// If no width and count_columns is set, [`IterTable`] will sniff the records, by
/// keeping a number of rows buffered (You can set the number via [`IterTable::sniff`]).
#[derive(Debug, Clone)]
pub struct IterTable<I> {
    records: I,
    cfg: CompactConfig,
    dim: IterTableDimension,
    table: Settings,
}

#[derive(Debug, Clone)]
struct Settings {
    sniff: usize,
    count_columns: Option<usize>,
    count_rows: Option<usize>,
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
            dim: IterTableDimension::new(ExactList::Exact(0), ExactList::Exact(1)),
            table: Settings {
                sniff: 1000,
                count_columns: None,
                count_rows: None,
            },
        }
    }

    /// With is a generic function which applies options to the [`IterTable`].
    pub fn with<O>(mut self, mut option: O) -> Self
    where
        for<'a> O: TableOption<IterRecords<&'a I>, IterTableDimension, CompactConfig>,
    {
        let count_columns = self.table.count_columns.unwrap_or(0);
        let mut records = IterRecords::new(&self.records, count_columns, self.table.count_rows);
        option.change(&mut records, &mut self.cfg, &mut self.dim);

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
    pub fn sniff(mut self, count: usize) -> Self {
        self.table.sniff = count;
        self
    }

    /// Set a height for each row.
    pub fn height(mut self, size: usize) -> Self {
        let pad = self.cfg.get_padding();
        let pad = pad.top.size + pad.bottom.size;
        let (w, _) = self.dim.into();
        self.dim = IterTableDimension::new(w, ExactList::Exact(size + pad));
        self
    }

    /// Set a width for each column.
    pub fn width(mut self, size: usize) -> Self {
        let pad = self.cfg.get_padding();
        let pad = pad.left.size + pad.right.size;
        let (_, h) = self.dim.into();
        self.dim = IterTableDimension::new(ExactList::Exact(size + pad), h);
        self
    }

    /// Build a string.
    ///
    /// We can't implement [`std::string::ToString`] cause it does takes `&self` reference.
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(self) -> String
    where
        I: IntoRecords,
    {
        let mut buf = String::new();
        self.fmt(&mut buf).expect("safe");

        buf
    }

    /// Format table into [`io::Write`]r.
    pub fn build<W: io::Write>(self, writer: W) -> io::Result<()>
    where
        I: IntoRecords,
    {
        let writer = UTF8Writer::new(writer);
        self.fmt(writer)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
    }

    /// Format table into [fmt::Write]er.
    pub fn fmt<W: fmt::Write>(self, writer: W) -> fmt::Result
    where
        I: IntoRecords,
    {
        let (width, height) = self.dim.into();
        let width = exact_list_to_exact_value(width);
        let height = exact_list_to_exact_value(height);
        let dims = Dims::new(width, height);

        build_grid(writer, self.records, self.cfg, &self.table, dims)
    }
}

fn build_grid<W: fmt::Write, I: IntoRecords>(
    writer: W,
    records: I,
    config: CompactConfig,
    iter_cfg: &Settings,
    dims: Dims<'_>,
) -> Result<(), fmt::Error> {
    let count_rows = iter_cfg.count_rows;

    let padding = config.get_padding();
    let padding = padding.left.size + padding.right.size;

    let dont_sniff =
        !matches!(dims.width, ExactValue::Exact(0)) && iter_cfg.count_columns.is_some();
    if dont_sniff {
        let count_columns = iter_cfg.count_columns.unwrap();

        let content_width = match &dims.width {
            ExactValue::Exact(w) => ExactValue::Exact(w.saturating_sub(padding)),
            ExactValue::List(list) => {
                ExactValue::List(list.iter().map(|w| w.saturating_sub(padding)).collect())
            }
        };

        match count_rows {
            Some(limit) => {
                let records = LimitRows::new(records, limit);
                let records = build_records(records, content_width, count_columns, count_rows);
                let cfg = SpannedConfig::from(config);
                return Grid::new(records, dims, cfg, NoColors).build(writer);
            }
            None => {
                let records = build_records(records, content_width, count_columns, count_rows);
                let cfg = SpannedConfig::from(config);
                return Grid::new(records, dims, cfg, NoColors).build(writer);
            }
        }
    }

    let records = BufRows::new(records, iter_cfg.sniff);
    let records = BufColumns::from(records);

    let count_columns = match iter_cfg.count_columns {
        Some(size) => size,
        None => records
            .as_slice()
            .iter()
            .map(|row| row.len())
            .max()
            .unwrap_or(0),
    };

    let (contentw, dimsw) = match dims.width {
        ExactValue::Exact(0) => {
            let records = LimitColumns::new(records.as_slice(), count_columns);
            let records = IterRecords::new(records, count_columns, None);
            let width = ExactDimension::width(records, &config);

            let dims_width = width.iter().map(|i| cmp::max(*i, padding)).collect();
            let content_width = width.iter().map(|i| i.saturating_sub(padding)).collect();

            (
                ExactValue::List(content_width),
                ExactValue::List(dims_width),
            )
        }
        width => {
            let content_width = match &width {
                ExactValue::Exact(w) => ExactValue::Exact(w.saturating_sub(padding)),
                ExactValue::List(list) => {
                    ExactValue::List(list.iter().map(|w| w.saturating_sub(padding)).collect())
                }
            };

            (content_width, width)
        }
    };

    let dimension = Dims::new(dimsw, dims.height);
    match count_rows {
        Some(limit) => {
            let records = LimitRows::new(records, limit);
            let records = build_records(records, contentw, count_columns, count_rows);
            let cfg = SpannedConfig::from(config);
            Grid::new(records, dimension, cfg, NoColors).build(writer)
        }
        None => {
            let records = build_records(records, contentw, count_columns, count_rows);
            let cfg = SpannedConfig::from(config);
            Grid::new(records, dimension, cfg, NoColors).build(writer)
        }
    }
}

fn create_config() -> CompactConfig {
    CompactConfig::default()
        .set_padding(Sides::new(
            Indent::spaced(1),
            Indent::spaced(1),
            Indent::default(),
            Indent::default(),
        ))
        .set_alignment_horizontal(AlignmentHorizontal::Left)
        .set_borders(*Style::ascii().get_borders())
}

fn build_records<I: IntoRecords>(
    records: I,
    width: ExactValue<'_>,
    count_columns: usize,
    count_rows: Option<usize>,
) -> IterRecords<LimitColumns<TruncateContent<'_, I>>> {
    let records = TruncateContent::new(records, width);
    let records = LimitColumns::new(records, count_columns);
    IterRecords::new(records, count_columns, count_rows)
}

#[derive(Debug, Clone)]
struct Dims<'a> {
    width: ExactValue<'a>,
    height: ExactValue<'a>,
}

impl<'a> Dims<'a> {
    fn new(width: ExactValue<'a>, height: ExactValue<'a>) -> Self {
        Self { width, height }
    }
}

impl Dimension for Dims<'_> {
    fn get_width(&self, column: usize) -> usize {
        self.width.get(column)
    }

    fn get_height(&self, row: usize) -> usize {
        self.height.get(row)
    }
}

fn exact_list_to_exact_value(width: ExactList) -> ExactValue<'static> {
    match width {
        ExactList::Exact(w) => ExactValue::Exact(w),
        ExactList::List(list) => ExactValue::List(list.into()),
    }
}
