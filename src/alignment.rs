use papergrid::{AlignmentHorizontal, AlignmentVertical, Entity, Grid, Settings};

use crate::{Object, TableOption};

/// Alignment represent a horizontal and vertical alignemt setting for a [`table` macros](./macro.table.html)
///
/// ```rust,no_run
///   # use tabled::{Style, Alignment, Row, table};
///   # let data: Vec<&'static str> = Vec::new();
///     let table = table!(&data, Alignment::center_horizontal(Row(..1)));
/// ```
#[derive(Debug)]
pub struct Alignment<O: Object>(O, AlignmentType);

#[derive(Debug)]
enum AlignmentType {
    Horizontal(AlignmentHorizontal),
    Vertical(AlignmentVertical),
}

impl<O: Object> Alignment<O> {
    /// Top constructs a vertical alignment to TOP
    pub fn top(obj: O) -> Self {
        Self::vertical(obj, AlignmentVertical::Top)
    }

    /// Bottom constructs a vertical alignment to BOTTOM
    pub fn bottom(obj: O) -> Self {
        Self::vertical(obj, AlignmentVertical::Bottom)
    }

    /// Center_vertical constructs a vertical alignment to CENTER
    pub fn center_vertical(obj: O) -> Self {
        Self::vertical(obj, AlignmentVertical::Center)
    }

    /// Left constructs a horizontal alignment to LEFT
    pub fn left(obj: O) -> Self {
        Self::horizontal(obj, AlignmentHorizontal::Left)
    }

    /// Right constructs a horizontal alignment to RIGHT
    pub fn right(obj: O) -> Self {
        Self::horizontal(obj, AlignmentHorizontal::Right)
    }

    /// Center_horizontal constructs a horizontal alignment to CENTER
    pub fn center_horizontal(obj: O) -> Self {
        Self::horizontal(obj, AlignmentHorizontal::Center)
    }

    fn horizontal(obj: O, alignment: AlignmentHorizontal) -> Self {
        Self(obj, AlignmentType::Horizontal(alignment))
    }

    fn vertical(obj: O, alignment: AlignmentVertical) -> Self {
        Self(obj, AlignmentType::Vertical(alignment))
    }
}

impl<O: Object> TableOption for Alignment<O> {
    fn change(&self, grid: &mut Grid) {
        let setting = match &self.1 {
            AlignmentType::Horizontal(a) => Settings::new().alignment(a.clone()),
            AlignmentType::Vertical(a) => Settings::new().vertical_alignment(a.clone()),
        };

        for (row, column) in self.0.cells(grid.count_rows(), grid.count_columns()) {
            grid.set(Entity::Cell(row, column), setting.clone())
        }
    }
}
