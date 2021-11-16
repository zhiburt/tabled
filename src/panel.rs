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
        grid.insert_row(self.1);
        grid.set(
            &Entity::Cell(self.1, 0),
            Settings::new()
                .text(self.0.as_ref().to_owned())
                .span(grid.count_columns()),
        )
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
