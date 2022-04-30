#[allow(unused)]
use crate::Table;
use crate::TableOption;
use papergrid::{Entity, Grid, Settings};

/// Panel allows to add a Row which has 1 continues Cell to a [Table].
///
/// See `examples/panel.rs`.
#[derive(Debug)]
pub struct Panel<S: AsRef<str>>(pub S, pub usize);

impl<S: AsRef<str>> TableOption for Panel<S> {
    fn change(&mut self, grid: &mut Grid) {
        let mut new_grid = Grid::new(grid.count_rows() + 1, grid.count_columns());
        for row in 0..grid.count_rows() {
            for column in 0..grid.count_columns() {
                let cell_settings = grid.get_settings(row, column).border_restriction(false);
                if row >= self.1 {
                    new_grid.set(Entity::Cell(row + 1, column), cell_settings);
                } else {
                    new_grid.set(Entity::Cell(row, column), cell_settings);
                }
            }
        }

        new_grid.set(
            Entity::Cell(self.1, 0),
            Settings::new()
                .text(self.0.as_ref().to_owned())
                .span(new_grid.count_columns()),
        );

        *grid = new_grid;
    }
}

/// Header inserts a [Panel] at the top.
/// See [Panel].
#[derive(Debug)]
pub struct Header<S: AsRef<str>>(pub S);

impl<S: AsRef<str>> TableOption for Header<S> {
    fn change(&mut self, grid: &mut Grid) {
        Panel(self.0.as_ref(), 0).change(grid)
    }
}

/// Footer renders a [Panel] at the bottom.
/// See [Panel].
#[derive(Debug)]
pub struct Footer<S: AsRef<str>>(pub S);

impl<S: AsRef<str>> TableOption for Footer<S> {
    fn change(&mut self, grid: &mut Grid) {
        Panel(self.0.as_ref(), grid.count_rows()).change(grid)
    }
}
