use crate::TableOption;
use papergrid::{Entity, Grid, Settings};

/// Panel allows to add a custom panel to table.
///
/// Don't use `Disable` after the calling `Panel`.
#[derive(Debug)]
pub struct Panel<S: AsRef<str>>(pub S, pub usize);

impl<S: AsRef<str>> TableOption for Panel<S> {
    fn change(&self, grid: &mut Grid) {
        grid.insert_row(self.1);
        grid.set(
            Entity::Cell(self.1, 0),
            Settings::new()
                .text(self.0.as_ref().to_owned())
                .set_span(grid.count_columns()),
        )
    }
}

/// Header renders information at the top.
/// see `Panel`
#[derive(Debug)]
pub struct Header<S: AsRef<str>>(pub S);

impl<S: AsRef<str>> TableOption for Header<S> {
    fn change(&self, grid: &mut Grid) {
        Panel(self.0.as_ref(), 0).change(grid)
    }
}

/// Footer renders information at the bottom.
/// see `Panel`
#[derive(Debug)]
pub struct Footer<S: AsRef<str>>(pub S);

impl<S: AsRef<str>> TableOption for Footer<S> {
    fn change(&self, grid: &mut Grid) {
        Panel(self.0.as_ref(), grid.count_rows()).change(grid)
    }
}
