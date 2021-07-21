use crate::CellOption;
use papergrid::{Entity, Grid, Settings};

/// Indent is responsible for a left/right/top/bottom indent of particular cells.
///
/// ```rust,no_run
///   # use tabled::{Style, Indent, Row, Table, Modify};
///   # let data: Vec<&'static str> = Vec::new();
///     let table = Table::new(&data).with(Modify::new(Row(..1)).with(Indent::new(0, 0, 1, 1)));
/// ```
#[derive(Debug)]
pub struct Indent(usize, usize, usize, usize);

impl Indent {
    /// Construct's an Indent object.
    pub fn new(left: usize, right: usize, top: usize, bottom: usize) -> Self {
        Self(left, right, top, bottom)
    }
}

impl CellOption for Indent {
    fn change_cell(&mut self, grid: &mut Grid, row: usize, column: usize) {
        grid.set(
            Entity::Cell(row, column),
            Settings::new().indent(self.0, self.1, self.2, self.3),
        )
    }
}
