//! This module contains a [`Rotate`] primitive which can be used in order to rotate [`Table`].
//!
//! It's also possible to transpose the table at the point of construction.
//! See [`Builder::index`].
//!
//! # Example
//!
#![cfg_attr(feature = "std", doc = "```")]
#![cfg_attr(not(feature = "std"), doc = "```ignore")]
//! use tabled::{Table, settings::Rotate};
//!
//! let data = [[1, 2, 3], [4, 5, 6]];
//!
//! let table = Table::new(data).with(Rotate::Left).to_string();
//!
//! assert_eq!(
//!     table,
//!     concat!(
//!         "+---+---+---+\n",
//!         "| 2 | 3 | 6 |\n",
//!         "+---+---+---+\n",
//!         "| 1 | 2 | 5 |\n",
//!         "+---+---+---+\n",
//!         "| 0 | 1 | 4 |\n",
//!         "+---+---+---+",
//!     )
//! );
//! ```
//!
//! [`Table`]: crate::Table
//! [`Builder::index`]: crate::builder::Builder::index

// use core::cmp::max;
use core::cmp::max;

use crate::{
    grid::records::{ExactRecords, Records, Resizable},
    settings::TableOption,
};

/// Rotate can be used to rotate a table by 90 degrees.
#[derive(Debug)]
pub enum Rotate {
    /// Rotate [`Table`] to the left.
    ///
    /// [`Table`]: crate::Table
    Left,
    /// Rotate [`Table`] to the right.
    ///
    /// [`Table`]: crate::Table
    Right,
    /// Rotate [`Table`] to the top.
    ///
    /// So the top becomes the bottom.
    ///
    /// [`Table`]: crate::Table
    Top,
    /// Rotate [`Table`] to the bottom.
    ///
    /// So the top becomes the bottom.
    ///
    /// [`Table`]: crate::Table
    Bottom,
}

impl<R, D, C> TableOption<R, D, C> for Rotate
where
    R: Records + ExactRecords + Resizable,
{
    fn change(self, records: &mut R, _: &mut C, _: &mut D) {
        let count_rows = records.count_rows();
        let count_cols = records.count_columns();

        match self {
            Self::Left => {
                let size = max(count_rows, count_cols);

                {
                    for _ in count_rows..size {
                        records.push_row();
                    }

                    for _ in count_cols..size {
                        records.push_column();
                    }
                }

                for col in 0..size {
                    for row in col..size {
                        records.swap((col, row), (row, col));
                    }
                }

                for row in 0..count_cols / 2 {
                    records.swap_row(row, count_cols - row - 1);
                }

                {
                    for (shift, row) in (count_rows..size).enumerate() {
                        let row = row - shift;
                        records.remove_column(row);
                    }

                    for (shift, col) in (count_cols..size).enumerate() {
                        let col = col - shift;
                        records.remove_row(col);
                    }
                }
            }
            Self::Right => {
                let size = max(count_rows, count_cols);

                {
                    for _ in count_rows..size {
                        records.push_row();
                    }

                    for _ in count_cols..size {
                        records.push_column();
                    }
                }

                for col in 0..size {
                    for row in col..size {
                        records.swap((col, row), (row, col));
                    }
                }

                for col in 0..count_rows / 2 {
                    records.swap_column(col, count_rows - col - 1);
                }

                {
                    for (shift, row) in (count_rows..size).enumerate() {
                        let row = row - shift;
                        records.remove_column(row);
                    }

                    for (shift, col) in (count_cols..size).enumerate() {
                        let col = col - shift;
                        records.remove_row(col);
                    }
                }
            }
            Self::Bottom | Self::Top => {
                for row in 0..count_rows / 2 {
                    for col in 0..count_cols {
                        let last_row = count_rows - row - 1;
                        records.swap((last_row, col), (row, col));
                    }
                }
            }
        }
    }
}
