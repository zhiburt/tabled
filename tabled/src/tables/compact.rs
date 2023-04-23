//! This module contains a [`CompactTable`] table.
//!
//! In contrast to [`Table`] [`CompactTable`] does no allocations but it consumes an iterator.
//! It's useful when you don't want to re/allocate a buffer for your data.
//!
//! # Example
//!
//! It works smoothly with arrays.
//!
#![cfg_attr(feature = "std", doc = "```")]
#![cfg_attr(not(feature = "std"), doc = "```ignore")]
//!use tabled::{settings::Style, tables::CompactTable};
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
//!use tabled::{settings::Style, tables::CompactTable};
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

use core::cmp::max;
use core::fmt;

use crate::{
    grid::{
        config::{AlignmentHorizontal, CompactConfig, Indent, Sides},
        dimension::{ConstDimension, ConstSize, Dimension},
        records::{
            into_records::{LimitColumns, LimitRows},
            IntoRecords, IterRecords,
        },
        util::string::string_width,
        CompactGrid,
    },
    settings::{Style, TableOption},
};

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

impl<I> CompactTable<I, ConstDimension<0, 0>> {
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
            dims: ConstDimension::new(ConstSize::Value(2), ConstSize::Value(1)),
        }
    }
}

impl<I, const ROWS: usize, const COLS: usize> CompactTable<I, ConstDimension<COLS, ROWS>> {
    /// Set a height for each row.
    pub fn height<S: Into<ConstSize<COUNT_ROWS>>, const COUNT_ROWS: usize>(
        self,
        size: S,
    ) -> CompactTable<I, ConstDimension<COLS, COUNT_ROWS>> {
        let (width, _) = self.dims.into();
        CompactTable {
            dims: ConstDimension::new(width, size.into()),
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
    ) -> CompactTable<I, ConstDimension<COUNT_COLUMNS, ROWS>> {
        let (_, height) = self.dims.into();
        CompactTable {
            dims: ConstDimension::new(size.into(), height),
            records: self.records,
            cfg: self.cfg,
            count_columns: self.count_columns,
            count_rows: self.count_rows,
        }
    }
}

impl<I, D> CompactTable<I, D> {
    /// Creates a new [`CompactTable`] structure with a known dimension.
    ///
    /// Notice that the function wont call [`Estimate`].
    ///
    /// [`Estimate`]: crate::grid::dimension::Estimate
    pub fn with_dimension(iter: I, dimension: D) -> Self
    where
        I: IntoRecords,
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

    /// Returns a table config.
    pub fn get_config(&self) -> &CompactConfig {
        &self.cfg
    }

    /// Returns a table config.
    pub fn get_config_mut(&mut self) -> &mut CompactConfig {
        &mut self.cfg
    }

    /// Format table into [fmt::Write]er.
    pub fn fmt<W>(self, writer: W) -> fmt::Result
    where
        I: IntoRecords,
        D: Dimension,
        W: fmt::Write,
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

    /// Format table into a writer.
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn build<W>(self, writer: W) -> std::io::Result<()>
    where
        I: IntoRecords,
        D: Dimension,
        W: std::io::Write,
    {
        let writer = super::util::utf8_writer::UTF8Writer::new(writer);
        self.fmt(writer)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))
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
        D: Dimension,
    {
        let mut buf = String::new();
        self.fmt(&mut buf).unwrap();
        buf
    }
}

impl<T, const ROWS: usize, const COLS: usize> From<[[T; COLS]; ROWS]>
    for CompactTable<[[T; COLS]; ROWS], ConstDimension<COLS, ROWS>>
where
    T: AsRef<str>,
{
    fn from(mat: [[T; COLS]; ROWS]) -> Self {
        let mut width = [0; COLS];
        for row in mat.iter() {
            for (col, text) in row.iter().enumerate() {
                let text = text.as_ref();
                let text_width = string_width(text);
                width[col] = max(width[col], text_width);
            }
        }

        // add padding
        for w in &mut width {
            *w += 2;
        }

        let dims = ConstDimension::new(ConstSize::List(width), ConstSize::Value(1));
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
        .set_padding(Sides::new(
            Indent::spaced(1),
            Indent::spaced(1),
            Indent::zero(),
            Indent::zero(),
        ))
        .set_alignment_horizontal(AlignmentHorizontal::Left)
        .set_borders(*Style::ascii().get_borders())
}

impl<R, D> TableOption<R, D, CompactConfig> for CompactConfig {
    fn change(self, _: &mut R, cfg: &mut CompactConfig, _: &mut D) {
        *cfg = self;
    }
}
