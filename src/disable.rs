//! This module contains a [`Disable`] structure which helps to
//! remove an etheir column or row from a [`Table`].
//!
//! Generally you should avoid use of [`Disable`] because it's a slow function and modifies the underlying records.
//! Providing correct data right away is better.
//!
//! # Example
//!
//! ```
//! use tabled::{Disable, TableIteratorExt, object::Rows};
//!
//! let data = vec!["Hello", "World", "!!!"];
//!
//! let table = data.table().with(Disable::row(Rows::new(1..2))).to_string();
//!
//! assert_eq!(
//!     table,
//!     "+-------+\n\
//!      | &str  |\n\
//!      +-------+\n\
//!      | World |\n\
//!      +-------+\n\
//!      | !!!   |\n\
//!      +-------+"
//! );
//! ```
//!
//! [`Table`]: crate::Table

// todo: Refactoring Disable to relay on Object instead

use std::{
    iter::Once,
    marker::PhantomData,
    ops::{Range, RangeBounds},
};

use papergrid::records::{Records, Resizable};

use crate::{
    object::{
        bounds_to_usize, Column, Columns, FirstColumn, FirstRow, LastColumn, LastRow, Row, Rows,
    },
    Table, TableOption,
};

/// Disable removes particular rows/columns from a [`Table`].
///
/// It tries to keeps track of style changes which may occur.
/// But it's not guaranteed will be the way you would expect it to be.
///
/// # Example
///
/// ```rust,no_run
/// # use tabled::{Disable, Table, object::Rows};
/// # let data: Vec<&'static str> = Vec::new();
/// let table = Table::new(&data).with(Disable::row(Rows::first()));
/// ```
///
/// [`Table`]: crate::Table
#[derive(Debug)]
pub struct Disable<L, Target> {
    locator: L,
    target: PhantomData<Target>,
}

impl<L> Disable<L, TargetColumn> {
    /// Disable columns.
    ///
    /// Available locators are:
    ///
    /// - [`Columns`]
    /// - [`Column`]
    /// - [`FirstColumn`]
    /// - [`LastColumn`]
    /// - [`ByColumnName`]
    ///
    /// ```rust
    /// use tabled::{disable::{Disable, ByColumnName}, builder::Builder, object::Columns};
    ///
    /// let mut builder = Builder::default();
    ///
    /// builder.add_record(["col1", "col2", "col3"]);
    /// builder.add_record(["Hello", "World", "1"]);
    ///
    /// let table = builder.build()
    ///     .with(Disable::column(ByColumnName::new("col3")))
    ///     .to_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     "+-------+-------+\n\
    ///      | col1  | col2  |\n\
    ///      +-------+-------+\n\
    ///      | Hello | World |\n\
    ///      +-------+-------+"
    /// );
    /// ```
    pub fn column(locator: L) -> Self
    where
        L: Locator<Coordinate = usize>,
    {
        Self {
            locator,
            target: PhantomData,
        }
    }
}

impl<L> Disable<L, TargetRow> {
    /// Disable rows.
    ///
    /// Available locators are:
    ///
    /// - [`Rows`]
    /// - [`Row`]
    /// - [`FirstRow`]
    /// - [`LastRow`]
    ///
    /// ```rust
    /// use tabled::{Disable, builder::Builder, object::Rows};
    ///
    /// let mut builder = Builder::default();
    ///
    /// builder.add_record(["col1", "col2", "col3"]);
    /// builder.add_record(["Hello", "World", "1"]);
    ///
    /// let table = builder.build()
    ///     .with(Disable::row(Rows::first()))
    ///     .to_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     "+-------+-------+---+\n\
    ///      | Hello | World | 1 |\n\
    ///      +-------+-------+---+"
    /// );
    /// ```
    pub fn row(locator: L) -> Self
    where
        L: Locator<Coordinate = usize>,
    {
        Self {
            locator,
            target: PhantomData,
        }
    }
}

/// A marker struct for [`Disable`].
#[derive(Debug)]
#[doc(hidden)]
pub struct TargetRow;

/// A marker struct for [`Disable`].
#[derive(Debug)]
#[doc(hidden)]
pub struct TargetColumn;

impl<L, D> TableOption<D> for Disable<L, TargetColumn>
where
    L: Locator<Coordinate = usize>,
    D: Records + Resizable,
{
    fn change(&mut self, table: &mut Table<D>) {
        let columns = self.locator.locate(table.get_records());
        let records = table.get_records_mut();
        let mut shift = 0;
        for col in columns.into_iter() {
            if col - shift > records.count_columns() {
                continue;
            }

            records.remove_column(col - shift);
            shift += 1;
        }

        table.destroy_width_cache();

        // fixme: I am pretty sure that we violate span constrains by removing rows/cols
        //        Because span may be bigger then the max number of rows/cols
    }
}

impl<L, D> TableOption<D> for Disable<L, TargetRow>
where
    L: Locator<Coordinate = usize>,
    D: Records + Resizable,
{
    fn change(&mut self, table: &mut Table<D>) {
        let rows = self.locator.locate(table.get_records());
        let records = table.get_records_mut();
        let mut shift = 0;
        for row in rows.into_iter() {
            if row - shift > records.count_rows() {
                continue;
            }

            records.remove_row(row - shift);
            shift += 1;
        }

        table.destroy_width_cache();

        // fixme: I am pretty sure that we violate span constrains by removing rows/cols
        //        Because span may be bigger then the max number of rows/cols
    }
}

/// Locator is an interface which searches for a particular thing in the [`Records`],
/// and returns coordinate of the foundings if any.
pub trait Locator {
    /// A coordinate of the finding.
    type Coordinate;
    /// An iterator of the coordinates.
    /// If it's empty it's consideret that nothing is found.
    type IntoIter: IntoIterator<Item = Self::Coordinate>;

    /// Search for the thing in [`Records`], returning a list of coordinates.
    fn locate<R>(&mut self, records: R) -> Self::IntoIter
    where
        R: Records;
}

impl<B> Locator for Columns<B>
where
    B: RangeBounds<usize>,
{
    type Coordinate = usize;
    type IntoIter = Range<usize>;

    fn locate<R>(&mut self, records: R) -> Self::IntoIter
    where
        R: Records,
    {
        let (from, to) = bounds_to_usize(
            self.get_range().start_bound(),
            self.get_range().end_bound(),
            records.count_columns(),
        );

        from..to
    }
}

impl Locator for Column {
    type Coordinate = usize;
    type IntoIter = Once<usize>;

    fn locate<R>(&mut self, _: R) -> Self::IntoIter
    where
        R: Records,
    {
        std::iter::once((*self).into())
    }
}

impl Locator for FirstColumn {
    type Coordinate = usize;
    type IntoIter = Once<usize>;

    fn locate<R>(&mut self, _: R) -> Self::IntoIter
    where
        R: Records,
    {
        std::iter::once(0)
    }
}

impl Locator for LastColumn {
    type Coordinate = usize;
    type IntoIter = Once<usize>;

    fn locate<R>(&mut self, records: R) -> Self::IntoIter
    where
        R: Records,
    {
        if records.count_columns() > 0 {
            std::iter::once(records.count_columns() - 1)
        } else {
            std::iter::once(0)
        }
    }
}

impl<B> Locator for Rows<B>
where
    B: RangeBounds<usize>,
{
    type Coordinate = usize;
    type IntoIter = Range<usize>;

    fn locate<R>(&mut self, records: R) -> Self::IntoIter
    where
        R: Records,
    {
        let (from, to) = bounds_to_usize(
            self.get_range().start_bound(),
            self.get_range().end_bound(),
            records.count_columns(),
        );

        from..to
    }
}

impl Locator for Row {
    type Coordinate = usize;
    type IntoIter = Once<usize>;

    fn locate<R>(&mut self, _: R) -> Self::IntoIter
    where
        R: Records,
    {
        std::iter::once((*self).into())
    }
}

impl Locator for FirstRow {
    type Coordinate = usize;
    type IntoIter = Once<usize>;

    fn locate<R>(&mut self, _: R) -> Self::IntoIter
    where
        R: Records,
    {
        std::iter::once(0)
    }
}

impl Locator for LastRow {
    type Coordinate = usize;
    type IntoIter = Once<usize>;

    fn locate<R>(&mut self, records: R) -> Self::IntoIter
    where
        R: Records,
    {
        if records.count_rows() > 0 {
            std::iter::once(records.count_rows() - 1)
        } else {
            std::iter::once(0)
        }
    }
}

/// The structure is an implementaion of [`Locator`] to search for a column by it's name.
/// A name is considerent be a value in a first row.
///
/// So even if in reality there's no header, first row will be consideret the one.
#[derive(Debug, Clone, Copy)]
pub struct ByColumnName<S>(S);

impl<S> ByColumnName<S> {
    /// Constructs a new object of the structure.
    pub fn new(text: S) -> Self
    where
        S: AsRef<str>,
    {
        Self(text)
    }
}

impl<S> Locator for ByColumnName<S>
where
    S: AsRef<str>,
{
    type Coordinate = usize;
    type IntoIter = Vec<usize>;

    fn locate<R>(&mut self, records: R) -> Self::IntoIter
    where
        R: Records,
    {
        (0..records.count_columns())
            .filter(|col| records.get_text((0, *col)) == self.0.as_ref())
            .collect::<Vec<_>>()
    }
}
