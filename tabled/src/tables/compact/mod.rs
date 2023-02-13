//! This module contains a [`IterTable`] table.
//!
//! In contrast to [`Table`] [`IterTable`] does no allocations but it consumes an iterator.
//! It's usefull when you dont want to re/allocate a buffer for your data.
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
mod utf8_writer;

use std::{borrow::Cow, cmp, fmt, io};

use papergrid::grid::compact::{CompactConfig, CompactGrid};

use crate::{
    grid::config::AlignmentHorizontal,
    grid::{
        config::{Entity, Indent},
        spanned::{
            config::{Formatting, GridConfig, Padding},
            ExactDimension, Grid,
        },
    },
    records::{
        into_records::{
            truncate_records::Width, BufColumns, BufRows, LimitColumns, LimitRows, TruncateContent,
        },
        IntoRecords, IterRecords,
    },
    settings::{style::Style, TableOption},
};

use self::{dimension::ConstantDimension, utf8_writer::UTF8Writer};

/// A table which consumes an [`IntoRecords`] iterator.
///
/// To be able to build table we need a dimensions.
/// If no width and count_columns is set, [`IterTable`] will sniff the records, by
/// keeping a number of rows buffered (You can set the number via [`IterTable::sniff`]).
#[derive(Debug, Clone)]
pub struct CompactTable<I, const COUNT_COLUMNS: usize> {
    records: I,
    cfg: CompactConfig,
    table: TableConfig<COUNT_COLUMNS>,
}

#[derive(Debug, Clone)]
struct TableConfig<const COUNT_COLUMNS: usize> {
    width: dimension::Width<COUNT_COLUMNS>,
    height: usize,
    count_columns: usize,
    count_rows: Option<usize>,
}

impl<I> CompactTable<I, 0> {
    /// Creates a new [`IterTable`] structure.
    pub fn new(iter: I, count_columns: usize, cell_width: usize) -> Self
    where
        I: IntoRecords,
    {
        Self {
            records: iter,
            cfg: create_config(),
            table: TableConfig {
                width: dimension::Width::Const(cell_width),
                height: 1,
                count_columns,
                count_rows: None,
            },
        }
    }
}

impl<I, const COUNT_COLUMNS: usize> CompactTable<I, COUNT_COLUMNS> {
    pub const fn with_dimension(iter: I, widths: [usize; COUNT_COLUMNS]) -> Self
    where
        I: IntoRecords,
    {
        Self {
            records: iter,
            cfg: create_config(),
            table: TableConfig {
                width: dimension::Width::List(widths),
                count_columns: COUNT_COLUMNS,
                height: 1,
                count_rows: None,
            },
        }
    }

    /// With is a generic function which applies options to the [`IterTable`].
    pub fn with<O>(mut self, option: O) -> Self
    where
        for<'a> O: TableOption<IterRecords<&'a I>, ConstantDimension<COUNT_COLUMNS>, CompactConfig>,
    {
        let mut records = IterRecords::new(
            &self.records,
            self.table.count_columns,
            self.table.count_rows,
        );

        let mut dims = ConstantDimension::new(self.table.width, self.table.height);

        let mut option = option;
        option.change(&mut records, &mut self.cfg, &mut dims);

        self
    }

    /// Limit a number of rows.
    pub fn rows(mut self, count_rows: usize) -> Self {
        self.table.count_rows = Some(count_rows);
        self
    }

    /// Set a height for each row.
    pub fn height(mut self, size: usize) -> Self {
        self.table.height = size;
        self
    }

    /// Format table into [fmt::Write]er.
    pub fn fmt<W: fmt::Write>(self, writer: W) -> fmt::Result
    where
        I: IntoRecords,
    {
        build_grid(writer, self.records, self.cfg, &self.table)
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
}

impl<I, const COUNT_COLUMNS: usize> core::fmt::Display for CompactTable<I, COUNT_COLUMNS>
where
    for<'a> &'a I: IntoRecords,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> core::fmt::Result {
        build_grid(f, &self.records, self.cfg, &self.table)
    }
}

fn build_grid<W: fmt::Write, I: IntoRecords, const COUNT_COLUMNS: usize>(
    writer: W,
    records: I,
    config: CompactConfig,
    iter_cfg: &TableConfig<COUNT_COLUMNS>,
) -> Result<(), fmt::Error> {
    let tab_size = config.get_tab_width();
    let count_rows = iter_cfg.count_rows;

    let padding = config.get_padding();
    let padding = padding.left.size + padding.right.size;

    let count_columns = iter_cfg.count_columns;

    let mut width = iter_cfg.width;
    match &mut width {
        dimension::Width::List(list) => {
            for w in list.iter_mut() {
                *w += padding;
            }
        }
        dimension::Width::Const(w) => *w += padding,
    }

    let dimension = dimension::ConstantDimension::new(width, iter_cfg.height);

    match count_rows {
        Some(limit) => {
            let records = LimitRows::new(records, limit);
            let records = LimitColumns::new(records, count_columns);
            let records = IterRecords::new(records, count_columns, count_rows);
            return CompactGrid::new(records, &dimension, config).build(writer);
        }
        None => {
            let records = LimitColumns::new(records, count_columns);
            let records = IterRecords::new(records, count_columns, count_rows);
            return CompactGrid::new(records, &dimension, config).build(writer);
        }
    }
}

const fn create_config() -> CompactConfig {
    CompactConfig::empty()
        .set_tab_width(4)
        .set_padding(Padding::new(
            Indent::spaced(1),
            Indent::spaced(1),
            Indent::zero(),
            Indent::zero(),
        ))
        .set_alignment_horizontal(AlignmentHorizontal::Left)
        .set_borders(*Style::ascii().get_borders())
}
