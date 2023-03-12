//! This module contains a [`Span`] settings, it helps to
//! make a cell take more space then it generally takes.
//!
//! # Example
//!
//! ```
//! use tabled::{settings::{Span, Modify}, Table};
//!
//! let data = [[1, 2, 3], [4, 5, 6]];
//!
//! let table = Table::new(data)
//!     .with(Modify::new((2, 0)).with(Span::column(2)))
//!     .with(Modify::new((0, 1)).with(Span::column(2)))
//!     .to_string();
//!
//! assert_eq!(
//!     table,
//!     concat!(
//!         "+---+---+---+\n",
//!         "| 0 | 1     |\n",
//!         "+---+---+---+\n",
//!         "| 1 | 2 | 3 |\n",
//!         "+---+---+---+\n",
//!         "| 4     | 6 |\n",
//!         "+---+---+---+",
//!     )
//! )
//! ```

mod column;
mod row;

pub use column::ColumnSpan;
pub use row::RowSpan;

/// Span represent a horizontal/column span setting for any cell on a [`Table`].
///
/// It will be ignored if:
///  - cell position is out of scope
///  - size is bigger then the total number of columns.
///  - size is bigger then the total number of rows.
///
/// ```rust,no_run
/// # use tabled::{Table, settings::{Style, Span, Modify, object::Columns}};
/// # let data: Vec<&'static str> = Vec::new();
/// let table = Table::new(&data)
///     .with(Modify::new(Columns::single(0)).with(Span::column(2)));
/// ```
///
/// [`Table`]: crate::Table
#[derive(Debug)]
pub struct Span;

impl Span {
    /// New constructs a horizontal/column [`Span`].
    ///
    /// If size is bigger then the total number of columns it will be ignored.
    pub fn column(size: usize) -> ColumnSpan {
        ColumnSpan::new(size)
    }

    /// New constructs a vertical/row [`Span`].
    ///
    /// If size is bigger then the total number of rows it will be ignored.
    pub fn row(size: usize) -> RowSpan {
        RowSpan::new(size)
    }
}
