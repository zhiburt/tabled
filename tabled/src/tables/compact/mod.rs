//! This module contains a [`CompactTable`] table.
//!
//! In contrast to [`Table`] [`CompactTable`] does no allocations but it consumes an iterator.
//! It's useful when you don't want to re/allocate a buffer for your data.
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
//! let mut buf = String::new();
//! CompactTable::new(&data)
//!     .columns(4)
//!     .width(5)
//!     .with(Style::ascii())
//!     .build(&mut buf);
//!
//! assert_eq!(
//!     buf,
//!     "+-----+-----+-----+-----+\n\
//!      | FreeBSD | 1993 | William and Lynne Jolitz | ?   |\n\
//!      |-----+-----+-----+-----|\n\
//!      | OpenBSD | 1995 | Theo de Raadt |     |\n\
//!      |-----+-----+-----+-----|\n\
//!      | HardenedBSD | 2014 | Oliver Pinter and Shawn Webb |     |\n\
//!      +-----+-----+-----+-----+"
//! );
//! ```
//!
//! [`Table`]: crate::Table

pub mod dimension;

use core::cmp::max;
use core::fmt;

use papergrid::{
    dimension::{Dimension, Estimate},
    grid::compact::{CompactConfig, CompactGrid},
    util::string::string_width_tab,
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
/// It assumes that the content has only single line.
#[derive(Debug, Clone)]
pub struct CompactTable<I, D> {
    records: I,
    cfg: CompactConfig,
    dims: D,
    count_columns: usize,
    count_rows: Option<usize>,
}

impl<I> CompactTable<I, ConstantDimension<0, 0>> {
    /// Creates a new [`CompactTable`] structure with a width dimension for all columns.
    pub const fn new(iter: I) -> Self
    where
        I: IntoRecords,
    {
        Self {
            records: iter,
            cfg: create_config(),
            count_columns: 0,
            count_rows: None,
            dims: ConstantDimension::new(ConstSize::Value(2), ConstSize::Value(1)),
        }
    }
}

impl<I, const ROWS: usize, const COLS: usize> CompactTable<I, ConstantDimension<COLS, ROWS>> {
    /// Set a height for each row.
    pub fn height<S: Into<ConstSize<COUNT_ROWS>>, const COUNT_ROWS: usize>(
        self,
        size: S,
    ) -> CompactTable<I, ConstantDimension<COLS, COUNT_ROWS>> {
        let (width, _) = self.dims.into();
        CompactTable {
            dims: ConstantDimension::new(width, size.into()),
            records: self.records,
            cfg: self.cfg,
            count_columns: self.count_columns,
            count_rows: self.count_rows,
        }
    }

    /// Set a width for each column.
    pub fn width<S: Into<ConstSize<COUNT_COLUMNS>>, const COUNT_COLUMNS: usize>(
        self,
        size: S,
    ) -> CompactTable<I, ConstantDimension<COUNT_COLUMNS, ROWS>> {
        let (_, height) = self.dims.into();
        CompactTable {
            dims: ConstantDimension::new(size.into(), height),
            records: self.records,
            cfg: self.cfg,
            count_columns: self.count_columns,
            count_rows: self.count_rows,
        }
    }
}

impl<I, D> CompactTable<I, D> {
    /// Creates a new [`CompactTable`] structure with a width dimension for all columns.
    pub const fn with_dimension(iter: I, dimension: D) -> Self
    where
        I: IntoRecords,
        D: Dimension,
    {
        Self {
            records: iter,
            dims: dimension,
            cfg: create_config(),
            count_columns: 0,
            count_rows: None,
        }
    }

    /// With is a generic function which applies options to the [`CompactTable`].
    pub fn with<O>(mut self, option: O) -> Self
    where
        for<'a> O: TableOption<IterRecords<&'a I>, D, CompactConfig>,
    {
        let mut records = IterRecords::new(&self.records, self.count_columns, self.count_rows);

        let mut option = option;
        option.change(&mut records, &mut self.cfg, &mut self.dims);

        self
    }

    /// Limit a number of rows.
    pub const fn rows(mut self, count_rows: usize) -> Self {
        self.count_rows = Some(count_rows);
        self
    }

    /// Limit a number of columns.
    pub const fn columns(mut self, count: usize) -> Self {
        self.count_columns = count;
        self
    }

    /// Format table into [fmt::Write]er.
    pub fn build<W: fmt::Write>(self, writer: W) -> fmt::Result
    where
        I: IntoRecords,
        D: Dimension,
    {
        build_grid(
            writer,
            self.records,
            self.dims,
            self.cfg,
            self.count_columns,
            self.count_rows,
        )
    }
}

impl<I, D> core::fmt::Display for CompactTable<I, D>
where
    for<'a> &'a I: IntoRecords,
    D: Dimension + Estimate<CompactConfig> + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> core::fmt::Result {
        let mut dims = self.dims.clone();
        dims.estimate(
            IterRecords::new(&self.records, self.count_columns, self.count_rows),
            &self.cfg,
        );

        build_grid(
            f,
            &self.records,
            dims,
            self.cfg,
            self.count_columns,
            self.count_rows,
        )
    }
}

impl<T, const ROWS: usize, const COLS: usize> From<[[T; COLS]; ROWS]>
    for CompactTable<[[T; COLS]; ROWS], ConstantDimension<COLS, ROWS>>
where
    T: AsRef<str>,
{
    fn from(mat: [[T; COLS]; ROWS]) -> Self {
        let mut width = [0; COLS];
        for row in mat.iter() {
            for (col, text) in row.iter().enumerate() {
                let text = text.as_ref();
                let text_width = string_width_tab(text, 4);
                width[col] = max(width[col], text_width);
            }
        }

        // add padding
        for w in &mut width {
            *w += 2;
        }

        let dims = ConstantDimension::new(ConstSize::List(width), ConstSize::Value(1));
        Self::with_dimension(mat, dims).columns(COLS).rows(ROWS)
    }
}

fn build_grid<W: fmt::Write, I: IntoRecords, D: Dimension>(
    writer: W,
    records: I,
    dims: D,
    config: CompactConfig,
    cols: usize,
    rows: Option<usize>,
) -> Result<(), fmt::Error> {
    match rows {
        Some(limit) => {
            let records = LimitRows::new(records, limit);
            let records = LimitColumns::new(records, cols);
            let records = IterRecords::new(records, cols, rows);
            CompactGrid::new(records, dims, config).build(writer)
        }
        None => {
            let records = LimitColumns::new(records, cols);
            let records = IterRecords::new(records, cols, rows);
            CompactGrid::new(records, dims, config).build(writer)
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
