use crate::CellOption;
use papergrid::{Entity, Grid, Settings};

pub use papergrid::{AlignmentHorizontal, AlignmentVertical};

/// Alignment represent a horizontal and vertical alignemt setting for any cell on a [crate::Table].
///
/// ```rust,no_run
///   # use tabled::{Alignment, Modify, object::Rows, Table};
///   # let data: Vec<&'static str> = Vec::new();
///     let table = Table::new(&data).with(Modify::new(Rows::single(0)).with(Alignment::center_horizontal()));
/// ```
#[derive(Debug)]
pub enum Alignment {
    Horizontal(AlignmentHorizontal),
    Vertical(AlignmentVertical),
}

impl Alignment {
    /// Top constructs a vertical alignment to [AlignmentVertical::Top]
    pub fn top() -> Self {
        Self::vertical(AlignmentVertical::Top)
    }

    /// Bottom constructs a vertical alignment to [AlignmentVertical::Bottom]
    pub fn bottom() -> Self {
        Self::vertical(AlignmentVertical::Bottom)
    }

    /// Center_vertical constructs a vertical alignment to [AlignmentVertical::Center]
    pub fn center_vertical() -> Self {
        Self::vertical(AlignmentVertical::Center)
    }

    /// Left constructs a horizontal alignment to [AlignmentHorizontal::Left]
    pub fn left() -> Self {
        Self::horizontal(AlignmentHorizontal::Left)
    }

    /// Right constructs a horizontal alignment to [AlignmentHorizontal::Right]
    pub fn right() -> Self {
        Self::horizontal(AlignmentHorizontal::Right)
    }

    /// Center_horizontal constructs a horizontal alignment to [AlignmentHorizontal::Center]
    pub fn center_horizontal() -> Self {
        Self::horizontal(AlignmentHorizontal::Center)
    }

    /// Returns an alignment with the given horizontal alignment.
    fn horizontal(alignment: AlignmentHorizontal) -> Self {
        Self::Horizontal(alignment)
    }

    /// Returns an alignment with the given vertical alignment.
    fn vertical(alignment: AlignmentVertical) -> Self {
        Self::Vertical(alignment)
    }
}

impl CellOption for Alignment {
    fn change_cell(&mut self, grid: &mut Grid, row: usize, column: usize) {
        let settings = match &self {
            Self::Horizontal(a) => Settings::new().alignment(*a),
            Self::Vertical(a) => Settings::new().vertical_alignment(*a),
        };

        grid.set(&Entity::Cell(row, column), settings);
    }
}
