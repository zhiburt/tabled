//! This module contains a [`Disable`] structure which helps to
//! remove an etheir column or row from a [`Table`].
//!
//! Essentially it is better to provide a correct data initially and not use disable.
//!
//! # Example
//!
//! ```
//! use tabled::{Disable, TableIteratorExt};
//!
//! let data = vec!["Hello", "World", "!!!"];
//!
//! let table = data.table().with(Disable::Row(1..2)).to_string();
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

use std::ops::RangeBounds;

use papergrid::records::{Records, Resizable};

use crate::{object::bounds_to_usize, Table, TableOption};

/// Disable removes particular rows/columns from a [`Table`].
///
/// It tries to keeps track of style changes which may occur.
/// But it's not guaranteed will be the way you would expect it to be.
///
/// # Example
///
/// ```rust,no_run
/// # use tabled::{Disable, Table};
/// # let data: Vec<&'static str> = Vec::new();
/// let table = Table::new(&data).with(Disable::Row(..1));
/// ```
///
/// [`Table`]: crate::Table
#[derive(Debug)]
pub enum Disable<R: RangeBounds<usize>> {
    /// Columns of the grid.
    /// Range is used to locate columns.
    Column(R),
    /// Rows of the grid.
    /// Range is used to locate rows.
    Row(R),
}

impl<R, D> TableOption<D> for Disable<R>
where
    R: RangeBounds<usize>,
    D: Records + Resizable,
{
    fn change(&mut self, table: &mut Table<D>) {
        let (count_rows, count_cols) = table.shape();

        match self {
            Self::Column(range) => {
                let (x, y) = bounds_to_usize(range.start_bound(), range.end_bound(), count_cols);

                let records = table.get_records_mut();
                for col in (x..y).enumerate().map(|(shift, col)| col - shift) {
                    records.remove_column(col);
                }
            }
            Self::Row(range) => {
                let (x, y) = bounds_to_usize(range.start_bound(), range.end_bound(), count_rows);

                let records = table.get_records_mut();
                for row in (x..y).enumerate().map(|(shift, col)| col - shift) {
                    records.remove_row(row);
                }
            }
        }

        table.destroy_width_cache();

        // fixme: I am pretty sure that we violate span constrains by removing rows/cols
        //        Because span may be bigger then the max number of rows/cols
    }
}
