use papergrid::{Entity, Grid, Settings};

use crate::{Object, TableOption};

/// Indent is responsilbe for a left/right/top/bottom indent.
///
/// ```rust,no_run
///   # use tabled::{Style, Indent, Row, table};
///   # let data: Vec<&'static str> = Vec::new();
///     let table = table!(&data, Indent::new(Row(..1), 0, 0, 1, 1));
/// ```
#[derive(Debug)]
pub struct Indent<O: Object>(O, usize, usize, usize, usize);

impl<O: Object> Indent<O> {
    pub fn new(obj: O, left: usize, right: usize, top: usize, bottom: usize) -> Self {
        Self(obj, left, right, top, bottom)
    }
}

impl<O: Object> TableOption for Indent<O> {
    fn change(&self, grid: &mut Grid) {
        for (row, column) in self.0.cells(grid.count_rows(), grid.count_columns()) {
            grid.set(
                Entity::Cell(row, column),
                Settings::new().indent(self.1, self.2, self.3, self.4),
            )
        }
    }
}
