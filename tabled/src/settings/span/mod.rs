//! This module contains a [`Span`] settings, it helps to
//! make a cell take more space then it generally takes.
//!
//! # Example
//!
//! ```rust,no_run
//! # use tabled::{Table, settings::{Style, Span, Modify, object::Columns}};
//! # let data: Vec<&'static str> = Vec::new();
//! let mut table = Table::new(&data);
//! table.modify(Columns::one(0), Span::column(2));
//! ```

mod column;
mod row;

pub use column::ColumnSpan;
pub use row::RowSpan;

/// Span represent a horizontal/column span setting for any cell on a [`Table`].
///
/// ```
/// use tabled::{settings::{Span, Modify}, Table};
/// use tabled::assert::assert_table;
///
/// let data = [[1, 2, 3], [4, 5, 6]];
///
/// let mut table = Table::new(data);
/// table.modify((0, 0), Span::row(2));
/// table.modify((0, 1), Span::column(2));
/// table.modify((2, 0), Span::column(1000));
///
/// assert_table!(
///     table,
///     "+---+---+---+"
///     "| 0 | 1     |"
///     "+   +---+---+"
///     "|   | 2 | 3 |"
///     "+---+---+---+"
///     "| 4         |"
///     "+---+---+---+"
/// );
/// ```
///
/// [`Table`]: crate::Table
#[derive(Debug)]
pub struct Span;

impl Span {
    /// New constructs a horizontal/column [`Span`].
    ///
    /// Value can be:
    ///     * == 0 - which means spread the cell on the whole line
    ///     * == 1 - which is a default span so can be used for removal of spans
    ///     * > 1 - which means to spread a cell by given number of columns right
    ///     * < 0 - which means to spread a cell by given number of columns left
    ///
    /// # Example
    ///
    /// ```
    /// use tabled::{settings::{Span, Modify}, Table};
    ///
    /// let data = [[1, 2, 3], [4, 5, 6]];
    ///
    /// let table = Table::new(data)
    ///     .modify((0, 0), Span::column(100))
    ///     .modify((1, 1), Span::column(2))
    ///     .modify((2, 1), Span::column(-1))
    ///     .to_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     concat!(
    ///         "+---++---+\n",
    ///         "| 0      |\n",
    ///         "+---++---+\n",
    ///         "| 1 | 2  |\n",
    ///         "+---++---+\n",
    ///         "| 5  | 6 |\n",
    ///         "+---++---+",
    ///     )
    /// )
    /// ```
    pub fn column(size: isize) -> ColumnSpan {
        ColumnSpan::new(size)
    }

    /// New constructs a vertical/row [`Span`].
    ///
    /// Value can be:
    ///     * == 0 - which means spread the cell on the whole line
    ///     * == 1 - which is a default span so can be used for removal of spans
    ///     * > 1 - which means to spread a cell by given number of rows bottom
    ///     * < 0 - which means to spread a cell by given number of rows top
    ///
    /// # Example
    ///
    /// ```
    /// use tabled::{settings::{Span, Modify}, Table};
    ///
    /// let data = [[1, 2, 3], [4, 5, 6]];
    ///
    /// let table = Table::new(data)
    ///     .modify((0, 0), Span::row(100))
    ///     .modify((1, 1), Span::row(2))
    ///     .modify((2, 2), Span::row(-1))
    ///     .to_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     concat!(
    ///         "+---+---+---+\n",
    ///         "| 0 | 1 | 2 |\n",
    ///         "+   +---+---+\n",
    ///         "+   + 2 + 6 +\n",
    ///         "+---+---+---+",
    ///     )
    /// )
    /// ```
    pub fn row(size: isize) -> RowSpan {
        RowSpan::new(size)
    }
}
