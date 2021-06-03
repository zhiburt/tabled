use papergrid::{Alignment, Entity, Grid, Settings};

use crate::{Object, TableOption};

/// HorizontalAlignment represent a horizontal alignemt setting for a [`table` macros](./macro.table.html)
///
/// ```rust,no_run
///   # use tabled::{Style, HorizontalAlignment, Alignment, Row, table};
///   # let data: Vec<&'static str> = Vec::new();
///     let table = table!(&data, HorizontalAlignment(Row(..1), Alignment::Center));
/// ```
///
#[derive(Debug)]
pub struct HorizontalAlignment<O: Object>(pub O, pub Alignment);

impl<O: Object> TableOption for HorizontalAlignment<O> {
    fn change(&self, grid: &mut Grid) {
        for (row, column) in self.0.cells(grid.count_rows(), grid.count_columns()) {
            grid.set(
                Entity::Cell(row, column),
                Settings::new().alignment(self.1.clone()),
            )
        }
    }
}
