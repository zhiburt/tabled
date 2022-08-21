//! This module contains a [`Span`] settings, it helps to
//! make a cell take more space then it generally takes.
//!
//! # Example
//!
//! ```
//! use tabled::{object::Cell, Modify, TableIteratorExt, Span};
//!
//! let data = [[1, 2, 3], [4, 5, 6]];
//!
//! let table = data.table()
//!     .with(Modify::new(Cell(2, 0)).with(Span::column(2)))
//!     .with(Modify::new(Cell(0, 1)).with(Span::column(2)))
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

use papergrid::{records::Records, Entity};

use crate::{CellOption, Table};

/// Span represent a horizontal/column span setting for any cell on a [`Table`].
///
/// It will be ignored if:
///  - cell position is out of scope
///  - size is bigger then the total number of columns.
///  - size is bigger then the total number of rows.
///
/// ```rust,no_run
/// # use tabled::{Style, Span, Modify, object::Columns, Table};
/// # let data: Vec<&'static str> = Vec::new();
/// let table = Table::new(&data)
///     .with(Modify::new(Columns::single(0)).with(Span::column(2)));
/// ```
///
/// [`Table`]: crate::Table
#[derive(Debug)]
pub struct Span(SpanType);

#[derive(Debug)]
enum SpanType {
    Column(usize),
    Row(usize),
}

impl Span {
    /// New constructs a horizontal/column [`Span`].
    ///
    /// If size is bigger then the total number of columns it will be ignored.
    pub fn column(size: usize) -> Self {
        Self(SpanType::Column(size))
    }

    /// New constructs a vertical/row [`Span`].
    ///
    /// If size is bigger then the total number of rows it will be ignored.
    pub fn row(size: usize) -> Self {
        Self(SpanType::Row(size))
    }
}

impl<R> CellOption<R> for Span
where
    R: Records,
{
    fn change_cell(&mut self, table: &mut Table<R>, entity: Entity) {
        let (count_rows, count_cols) = table.shape();
        for pos in entity.iter(count_rows, count_cols) {
            match self.0 {
                SpanType::Column(size) => {
                    table.get_config_mut().set_column_span(pos, size);
                }
                SpanType::Row(size) => {
                    table.get_config_mut().set_row_span(pos, size);
                }
            }
        }
    }
}
