//! This module contains a [Span] settings, it helps to
//! make a cell take more space then it generally takes.
//!
//! # Example
//!
//! ```
//! use tabled::{object::Cell, ModifyObject, TableIteratorExt, Span};
//!
//! let data = [[1, 2, 3], [4, 5, 6]];
//!
//! let table = data.table()
//!     .with(Cell(2, 0).modify().with(Span::column(2)))
//!     .with(Cell(0, 1).modify().with(Span::column(2)))
//!     .with(Cell(0, 0).modify().with(Span::row(2)))
//!     .with(Cell(1, 2).modify().with(Span::row(2)))
//!     .to_string();
//!
//! assert_eq!(
//!     table,
//!     concat!(
//!         "+---+---+---+\n",
//!         "| 0 |   1   |\n",
//!         "+   +---+---+\n",
//!         "|   | 2 | 3 |\n",
//!         "+---+---+   +\n",
//!         "|   4   |   |\n",
//!         "+---+---+---+\n",
//!     )
//! )
//! ```

use papergrid::{Entity, Grid, Settings};

use crate::CellOption;

pub use papergrid::{AlignmentHorizontal, AlignmentVertical};

/// Span represent a horizontal/column span setting for any cell on a [Table].
///
/// # Example
/// 
/// ```rust,no_run
///   # use tabled::{Style, Span, ModifyObject, object::Columns, TableIteratorExt};
///   # let data: Vec<&'static str> = Vec::new();
///     let table = data.table()
///         .with(
///             Columns::single(0)
///                 .modify()
///                 .with(Span::column(2))
///                 .with(Span::row(2))
///         );
/// ```
///
/// [Table]: crate::Table
#[derive(Debug)]
pub struct Span;

impl Span {
    /// New constructs a horizontal/column [Span].
    ///
    /// # Example
    /// 
    /// ```rust,no_run
    ///   # use tabled::{Style, Span, ModifyObject, object::Cell, TableIteratorExt};
    ///   # let data: Vec<&'static str> = Vec::new();
    ///     let table = data.table().with(Cell(0, 1).modify().with(Span::column(2)));
    /// ```
    pub fn column(size: usize) -> ColumnSpan {
        ColumnSpan(size)
    }

    /// New constructs a vertical/row [Span].
    ///
    /// # Example
    ///
    /// ```rust,no_run
    ///   # use tabled::{Style, Span, ModifyObject, object::Cell, TableIteratorExt};
    ///   # let data: Vec<&'static str> = Vec::new();
    ///     let table = data.table().with(Cell(0, 1).modify().with(Span::row(2)));
    /// ```
    pub fn row(size: usize) -> RowSpan {
        RowSpan(size)
    }
}

/// ColumnSpan represents a horizontal span.
pub struct ColumnSpan(usize);

/// RowSpan represents a vertical span.
pub struct RowSpan(usize);

impl CellOption for ColumnSpan {
    fn change_cell(&mut self, grid: &mut Grid, row: usize, column: usize) {
        grid.set(Entity::Cell(row, column), Settings::new().span(self.0));
    }
}

impl CellOption for RowSpan {
    fn change_cell(&mut self, grid: &mut Grid, row: usize, column: usize) {
        grid.set(
            Entity::Cell(row, column),
            Settings::new().span_vertical(self.0),
        );
    }
}
