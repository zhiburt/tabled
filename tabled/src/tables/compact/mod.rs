//! This module contains a [`CompactTable`] table.
//!
//! In contrast to [`Table`] [`CompactTable`] does no allocations but it consumes an iterator.
//! It's usefull when you dont want to re/allocate a buffer for your data.
//!
//! # Example
//!
//! ```
//!use tabled::{settings::style::Style, tables::compact::CompactTable};
//!
//! let data = [
//!     ["FreeBSD", "1993", "William and Lynne Jolitz", "?"],
//!     ["OpenBSD", "1995", "Theo de Raadt", ""],
//!     ["HardenedBSD", "2014", "Oliver Pinter and Shawn Webb", ""],
//! ];
//!
//! let table = CompactTable::from(data)
//!     .with(Style::psql())
//!     .to_string();
//!
//! assert_eq!(
//!     table,
//!     "+-------------+------+------------------------------+---+\n\
//!      | FreeBSD     | 1993 | William and Lynne Jolitz     | ? |\n\
//!      |-------------+------+------------------------------+---|\n\
//!      | OpenBSD     | 1995 | Theo de Raadt                |   |\n\
//!      |-------------+------+------------------------------+---|\n\
//!      | HardenedBSD | 2014 | Oliver Pinter and Shawn Webb |   |\n\
//!      +-------------+------+------------------------------+---+"
//! );
//! ```
//!
//! [`Table`]: crate::Table

mod dimension;
mod utf8_writer;

use std::{fmt, io};

use papergrid::{
    grid::compact::{CompactConfig, CompactGrid},
    util::string::string_width_tab,
};

use crate::{
    grid::config::AlignmentHorizontal,
    grid::{config::Indent, spanned::config::Padding},
    records::{
        into_records::{LimitColumns, LimitRows},
        IntoRecords, IterRecords,
    },
    settings::{style::Style, TableOption},
};

use self::{dimension::ConstantDimension, utf8_writer::UTF8Writer};

/// A table which consumes an [`IntoRecords`] iterator.
///
/// To be able to build table we need a descrit dimensions.
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
    /// Creates a new [`CompactTable`] structure.
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
    /// Creates a new [`CompactTable`] structure with a width dimension for all columns.
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

    /// With is a generic function which applies options to the [`CompactTable`].
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

impl<T, const COUNT_COLUMNS: usize, const COUNT_ROWS: usize> From<[[T; COUNT_COLUMNS]; COUNT_ROWS]>
    for CompactTable<[[T; COUNT_COLUMNS]; COUNT_ROWS], COUNT_COLUMNS>
where
    T: AsRef<str>,
{
    fn from(mat: [[T; COUNT_COLUMNS]; COUNT_ROWS]) -> Self {
        let mut widths = [0; COUNT_COLUMNS];
        for row in &mat {
            for (col, text) in row.iter().enumerate() {
                let w = string_width_tab(text.as_ref(), 4);
                widths[col] = std::cmp::max(widths[col], w);
            }
        }

        Self::with_dimension(mat, widths)
    }
}

fn build_grid<W: fmt::Write, I: IntoRecords, const COUNT_COLUMNS: usize>(
    writer: W,
    records: I,
    config: CompactConfig,
    iter_cfg: &TableConfig<COUNT_COLUMNS>,
) -> Result<(), fmt::Error> {
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
            CompactGrid::new(records, &dimension, config).build(writer)
        }
        None => {
            let records = LimitColumns::new(records, count_columns);
            let records = IterRecords::new(records, count_columns, count_rows);
            CompactGrid::new(records, &dimension, config).build(writer)
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
