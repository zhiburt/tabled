use papergrid::{Entity, Grid, Settings};

use crate::{Object, TableOption};

pub struct ChangeRing<O: Object>(pub O, pub Vec<Box<dyn Fn(&str) -> String>>);

impl<O: Object> TableOption for ChangeRing<O> {
    fn change(&self, grid: &mut Grid) {
        if self.1.is_empty() {
            return;
        }

        let mut ring = self.1.iter().cycle();

        let cells = self.0.cells(grid.count_rows(), grid.count_columns());
        for (row, column) in cells {
            let change_function = ring.next().unwrap();
            let content = grid.get_cell_content(row, column);
            let content = change_function(content);
            grid.set(Entity::Cell(row, column), Settings::new().text(content))
        }
    }
}

pub fn multiline(f: Box<dyn Fn(&str) -> String>) -> Box<dyn Fn(&str) -> String> {
    Box::new(move |s: &str| s.lines().map(|s| f(s)).collect::<Vec<_>>().join("\n"))
}
