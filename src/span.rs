use crate::CellOption;
use papergrid::{Entity, Grid, Settings};

pub use papergrid::{AlignmentHorizontal, AlignmentVertical};

/// Span represent a horizontal/column span setting for any cell on a [crate::Table].
///
/// ```rust,no_run
///   # use tabled::{Style, Span, Modify, object::Columns, Table};
///   # let data: Vec<&'static str> = Vec::new();
///     let table = Table::new(&data)
///         .with(Modify::new(Columns::single(0)).with(Span::column(2)));
/// ```
#[derive(Debug)]
pub struct Span {
    size: usize,
}

impl Span {
    /// New constructs a horizontal/column [Span].
    pub fn column(size: usize) -> Self {
        Self { size }
    }
}

impl CellOption for Span {
    fn change_cell(&mut self, grid: &mut Grid, row: usize, column: usize) {
        grid.set(&Entity::Cell(row, column), Settings::new().span(self.size));
    }
}
