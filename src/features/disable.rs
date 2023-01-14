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

use std::marker::PhantomData;

use papergrid::records::{ExactRecords, Records, Resizable};

use crate::{locator::Locator, Table, TableOption};

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
    /// use tabled::{Disable, locator::ByColumnName, builder::Builder, object::Columns};
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
    ///
    /// [`Columns`]: crate::object::Columns
    /// [`Column`]: crate::object::Column
    /// [`FirstColumn`]: crate::object::FirstColumn
    /// [`LastColumn`]: crate::object::LastColumn
    /// [`ByColumnName`]: crate::locator::ByColumnName
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
    ///
    /// [`Rows`]: crate::object::Rows
    /// [`Row`]: crate::object::Row
    /// [`FirstRow`]: crate::object::FirstRow
    /// [`LastRow`]: crate::object::LastRow
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
pub struct TargetRow;

/// A marker struct for [`Disable`].
#[derive(Debug)]
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
        table.destroy_height_cache();

        // fixme: I am pretty sure that we violate span constrains by removing rows/cols
        //        Because span may be bigger then the max number of rows/cols
    }
}

impl<L, D> TableOption<D> for Disable<L, TargetRow>
where
    L: Locator<Coordinate = usize>,
    D: Records + ExactRecords + Resizable,
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
        table.destroy_height_cache();

        // fixme: I am pretty sure that we violate span constrains by removing rows/cols
        //        Because span may be bigger then the max number of rows/cols
    }
}
