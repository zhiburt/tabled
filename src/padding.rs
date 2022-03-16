use crate::CellOption;
use papergrid::{Entity, Grid, Settings, DEFAULT_INDENT_FILL_CHAR};

/// Padding is responsible for a left/right/top/bottom indent of particular and fill one set characters.
///
/// ```rust,no_run
///   # use tabled::{Style, Padding, Row, Table, Modify};
///   # let data: Vec<&'static str> = Vec::new();
///     let table = Table::new(&data).with(Modify::new(Row(..1)).with(Padding::new(0, 0, 1, 1).set_fill('>', '<', '^', 'V')));
/// ```
pub struct Padding(papergrid::Padding);

impl Padding {
    /// Construct's an Padding object.
    pub fn new(left: usize, right: usize, top: usize, bottom: usize) -> Self {
        Self(papergrid::Padding {
            top: papergrid::Indent::new(top, DEFAULT_INDENT_FILL_CHAR),
            bottom: papergrid::Indent::new(bottom, DEFAULT_INDENT_FILL_CHAR),
            left: papergrid::Indent::new(left, DEFAULT_INDENT_FILL_CHAR),
            right: papergrid::Indent::new(right, DEFAULT_INDENT_FILL_CHAR),
        })
    }

    pub fn set_fill(mut self, left: char, right: char, top: char, bottom: char) -> Self {
        self.0.left.fill = left;
        self.0.right.fill = right;
        self.0.top.fill = top;
        self.0.bottom.fill = bottom;
        self
    }
}

impl CellOption for Padding {
    fn change_cell(&mut self, grid: &mut Grid, row: usize, column: usize) {
        grid.set(
            &Entity::Cell(row, column),
            Settings::new().padding(self.0.left, self.0.right, self.0.top, self.0.bottom),
        )
    }
}
