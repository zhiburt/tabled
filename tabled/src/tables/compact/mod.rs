//! This module contains a [`CompactTable`] table.
//!
//! In contrast to [`Table`] [`CompactTable`] does no allocations but it consumes an iterator.
//! It's usefull when you dont want to re/allocate a buffer for your data.
//!
//! # Example
//!
//! It works smoothly with arrays.
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
//!     concat!(
//!         " FreeBSD     | 1993 | William and Lynne Jolitz     | ? \n",
//!         "-------------+------+------------------------------+---\n",
//!         " OpenBSD     | 1995 | Theo de Raadt                |   \n",
//!         " HardenedBSD | 2014 | Oliver Pinter and Shawn Webb |   ",
//!     )
//! );
//! ```
//!
//! But it's default creation requires to be given an estimated cell width, and the amount of columns.
//!
#![cfg_attr(feature = "std", doc = "```")]
#![cfg_attr(not(feature = "std"), doc = "```ignore")]
//!use tabled::{settings::style::Style, tables::compact::CompactTable};
//!
//! let data = [
//!     ["FreeBSD", "1993", "William and Lynne Jolitz", "?"],
//!     ["OpenBSD", "1995", "Theo de Raadt", ""],
//!     ["HardenedBSD", "2014", "Oliver Pinter and Shawn Webb", ""],
//! ];
//!
//! // See what will happen if the given width is too narrow
//!
//! let table = CompactTable::new(&data)
//!     .columns(4)
//!     .width(5)
//!     .with(Style::ascii())
//!     .to_string();
//!
//! assert_eq!(
//!     table,
//!     "+-------+-------+-------+-------+\n\
//!      | FreeBSD | 1993  | William and Lynne Jolitz | ?     |\n\
//!      |-------+-------+-------+-------|\n\
//!      | OpenBSD | 1995  | Theo de Raadt |       |\n\
//!      |-------+-------+-------+-------|\n\
//!      | HardenedBSD | 2014  | Oliver Pinter and Shawn Webb |       |\n\
//!      +-------+-------+-------+-------+"
//! );
//! ```
//!
//! [`Table`]: crate::Table

pub mod dimension;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
mod utf8_writer;

use core::cmp::max;
use core::fmt;

use papergrid::{
    grid::compact::{CompactConfig, CompactGrid},
    util::string::{count_lines, string_width_tab},
};

use crate::{
    grid::config::AlignmentHorizontal,
    grid::config::{Indent, Sides},
    records::{
        into_records::{LimitColumns, LimitRows},
        IntoRecords, IterRecords,
    },
    settings::{style::Style, TableOption},
};

use self::dimension::{ConstSize, ConstantDimension};

/// A table which consumes an [`IntoRecords`] iterator.
///
/// To be able to build table we need a descrit dimensions.
#[derive(Debug, Clone)]
pub struct CompactTable<I, const ROWS: usize, const COLS: usize> {
    records: I,
    cfg: CompactConfig,
    table: TableConfig<ROWS, COLS>,
}

#[derive(Debug, Clone)]
struct TableConfig<const ROWS: usize, const COLS: usize> {
    width: ConstSize<COLS>,
    height: ConstSize<ROWS>,
    count_columns: usize,
    count_rows: Option<usize>,
}

impl<I> CompactTable<I, 0, 0> {
    /// Creates a new [`CompactTable`] structure with a width dimension for all columns.
    pub const fn new(iter: I) -> Self
    where
        I: IntoRecords,
    {
        Self {
            records: iter,
            cfg: create_config(),
            table: TableConfig {
                count_columns: 0,
                count_rows: None,
                width: ConstSize::Value(0),
                height: ConstSize::Value(1),
            },
        }
    }
}

impl<I, const ROWS: usize, const COLS: usize> CompactTable<I, ROWS, COLS> {
    /// Creates a new [`CompactTable`] structure with a width dimension for all columns.
    pub const fn with_dimension(iter: I, width: ConstSize<COLS>, height: ConstSize<ROWS>) -> Self
    where
        I: IntoRecords,
    {
        let count_rows = if ROWS == 0 { None } else { Some(ROWS) };

        Self {
            records: iter,
            cfg: create_config(),
            table: TableConfig {
                width,
                height,
                count_columns: COLS,
                count_rows,
            },
        }
    }

    /// With is a generic function which applies options to the [`CompactTable`].
    pub fn with<O>(mut self, option: O) -> Self
    where
        for<'a> O: TableOption<IterRecords<&'a I>, ConstantDimension<ROWS, COLS>, CompactConfig>,
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

    /// Limit a number of columns.
    pub const fn columns(mut self, count: usize) -> Self {
        self.table.count_columns = count;
        self
    }

    /// Set a height for each row.
    pub fn height<S: Into<ConstSize<COUNT_ROWS>>, const COUNT_ROWS: usize>(
        self,
        size: S,
    ) -> CompactTable<I, COUNT_ROWS, COLS> {
        CompactTable {
            records: self.records,
            cfg: self.cfg,
            table: TableConfig {
                width: self.table.width,
                height: size.into(),
                count_columns: self.table.count_columns,
                count_rows: self.table.count_rows,
            },
        }
    }

    /// Set a width for each column.
    pub fn width<S: Into<ConstSize<COUNT_COLUMNS>>, const COUNT_COLUMNS: usize>(
        self,
        size: S,
    ) -> CompactTable<I, ROWS, COUNT_COLUMNS> {
        CompactTable {
            records: self.records,
            cfg: self.cfg,
            table: TableConfig {
                width: size.into(),
                height: self.table.height,
                count_columns: self.table.count_columns,
                count_rows: self.table.count_rows,
            },
        }
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
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn to_string(self) -> String
    where
        I: IntoRecords,
    {
        let mut buf = String::new();
        self.fmt(&mut buf).expect("safe");

        buf
    }

    /// Format table into [`std::io::Write`]r.
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn build<W: std::io::Write>(self, writer: W) -> std::io::Result<()>
    where
        I: IntoRecords,
    {
        let writer = utf8_writer::UTF8Writer::new(writer);
        self.fmt(writer)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))
    }
}

impl<I, const ROWS: usize, const COLS: usize> core::fmt::Display for CompactTable<I, ROWS, COLS>
where
    for<'a> &'a I: IntoRecords,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> core::fmt::Result {
        build_grid(f, &self.records, self.cfg, &self.table)
    }
}

impl<T, const ROWS: usize, const COLS: usize> From<[[T; COLS]; ROWS]>
    for CompactTable<[[T; COLS]; ROWS], ROWS, COLS>
where
    T: AsRef<str>,
{
    fn from(mat: [[T; COLS]; ROWS]) -> Self {
        let mut width = [0; COLS];
        let mut height = [0; ROWS];
        for (i, row) in mat.iter().enumerate() {
            for (col, text) in row.iter().enumerate() {
                let text = text.as_ref();
                let text_width = string_width_tab(text, 4);
                let text_lines = count_lines(text);
                width[col] = max(width[col], text_width);
                height[i] = max(height[i], text_lines)
            }
        }

        Self::with_dimension(mat, ConstSize::List(width), ConstSize::List(height))
    }
}

fn build_grid<W: fmt::Write, I: IntoRecords, const ROWS: usize, const COLS: usize>(
    writer: W,
    records: I,
    config: CompactConfig,
    iter_cfg: &TableConfig<ROWS, COLS>,
) -> Result<(), fmt::Error> {
    let pad = config.get_padding();
    let hpad = pad.left.size + pad.right.size;
    let vpad = pad.top.size + pad.bottom.size;

    let mut width = iter_cfg.width;
    match &mut width {
        ConstSize::List(list) => {
            for w in list.iter_mut() {
                *w += hpad;
            }
        }
        ConstSize::Value(w) => *w += hpad,
    }

    let mut height = iter_cfg.height;
    match &mut height {
        ConstSize::List(list) => {
            for w in list.iter_mut() {
                *w += vpad;
            }
        }
        ConstSize::Value(w) => *w += vpad,
    }

    let dimension = dimension::ConstantDimension::new(width, iter_cfg.height);

    let count_rows = iter_cfg.count_rows;
    let count_columns = iter_cfg.count_columns;
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
        .set_padding(Sides::new(
            Indent::spaced(1),
            Indent::spaced(1),
            Indent::zero(),
            Indent::zero(),
        ))
        .set_alignment_horizontal(AlignmentHorizontal::Left)
        .set_borders(*Style::ascii().get_borders())
}
