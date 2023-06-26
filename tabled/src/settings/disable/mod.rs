//! This module contains a [`Disable`] structure which helps to
//! remove an etheir column or row from a [`Table`].
//!
//! # Example
//!
//! ```rust,no_run
//! # use tabled::{Table, settings::{Disable, object::Rows}};
//! # let data: Vec<&'static str> = Vec::new();
//! let table = Table::new(&data).with(Disable::row(Rows::first()));
//! ```
//!
//! [`Table`]: crate::Table

use std::marker::PhantomData;

use crate::{
    grid::records::{ExactRecords, Records, Resizable},
    settings::{locator::Locator, TableOption},
};

/// Disable removes particular rows/columns from a [`Table`].
///
/// It tries to keeps track of style changes which may occur.
/// But it's not guaranteed will be the way you would expect it to be.
///
/// Generally you should avoid use of [`Disable`] because it's a slow function and modifies the underlying records.
/// Providing correct data right away is better.
///
/// # Example
///
/// ```
/// use tabled::{Table, settings::{Disable, object::Rows}};
///
/// let data = vec!["Hello", "World", "!!!"];
///
/// let table = Table::new(data).with(Disable::row(Rows::new(1..2))).to_string();
///
/// assert_eq!(
///     table,
///     "+-------+\n\
///      | &str  |\n\
///      +-------+\n\
///      | World |\n\
///      +-------+\n\
///      | !!!   |\n\
///      +-------+"
/// );
///
/// ```
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
    /// use tabled::{builder::Builder, settings::{Disable, locator::ByColumnName, object::Columns}};
    ///
    /// let mut builder = Builder::default();
    ///
    /// builder.push_record(["col1", "col2", "col3"]);
    /// builder.push_record(["Hello", "World", "1"]);
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
    /// [`Columns`]: crate::settings::object::Columns
    /// [`Column`]: crate::settings::object::Column
    /// [`FirstColumn`]: crate::settings::object::FirstColumn
    /// [`LastColumn`]: crate::settings::object::LastColumn
    /// [`ByColumnName`]: crate::settings::locator::ByColumnName
    pub fn column(locator: L) -> Self {
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
    /// use tabled::{settings::{Disable, object::Rows}, builder::Builder};
    ///
    /// let mut builder = Builder::default();
    /// builder.push_record(["col1", "col2", "col3"]);
    /// builder.push_record(["Hello", "World", "1"]);
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
    /// [`Rows`]: crate::settings::object::Rows
    /// [`Row`]: crate::settings::object::Row
    /// [`FirstRow`]: crate::settings::object::FirstRow
    /// [`LastRow`]: crate::settings::object::LastRow
    pub fn row(locator: L) -> Self {
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

impl<L, R, D, C> TableOption<R, D, C> for Disable<L, TargetColumn>
where
    for<'a> L: Locator<&'a R, Coordinate = usize>,
    R: Records + Resizable,
{
    fn change(mut self, records: &mut R, _: &mut C, _: &mut D) {
        let columns = self.locator.locate(records).into_iter().collect::<Vec<_>>();

        let mut shift = 0;
        for col in columns.into_iter() {
            if col - shift > records.count_columns() {
                continue;
            }

            records.remove_column(col - shift);
            shift += 1;
        }

        // fixme: I am pretty sure that we violate span constrains by removing rows/cols
        //        Because span may be bigger then the max number of rows/cols
    }
}

impl<L, R, D, C> TableOption<R, D, C> for Disable<L, TargetRow>
where
    for<'a> L: Locator<&'a R, Coordinate = usize>,
    R: ExactRecords + Resizable,
{
    fn change(mut self, records: &mut R, _: &mut C, _: &mut D) {
        let rows = self.locator.locate(records).into_iter().collect::<Vec<_>>();

        let mut shift = 0;
        for row in rows.into_iter() {
            if row - shift > records.count_rows() {
                continue;
            }

            records.remove_row(row - shift);
            shift += 1;
        }

        // fixme: I am pretty sure that we violate span constrains by removing rows/cols
        //        Because span may be bigger then the max number of rows/cols
    }
}
