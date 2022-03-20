#[allow(unused)]
use crate::Table;
use crate::{style::Border, TableOption};
use papergrid::{Entity, Grid, Settings};

/// Highlight modifies a table style by changing a border of a target [Table] segment.
///
/// # Example
///
/// ```
/// use tabled::{TableIteratorExt, Highlight, style::{Border, Style}};
///
/// let data = [
///     ("ELF", "Extensible Linking Format", true),
///     ("DWARF", "", true),
///     ("PE", "Portable Executable", false),
/// ];
///
/// let table = data.iter()
///                .enumerate()
///                .table()
///                .with(Style::github_markdown())
///                .with(Highlight::frame(Border::default().top('^').bottom('v')))
///                .to_string();
///
/// assert_eq!(
///     table,
///     concat!(
///         " ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ \n",
///         "| usize | &str  |           &str            | bool  |\n",
///         "|-------+-------+---------------------------+-------|\n",
///         "|   0   |  ELF  | Extensible Linking Format | true  |\n",
///         "|   1   | DWARF |                           | true  |\n",
///         "|   2   |  PE   |    Portable Executable    | false |\n",
///         " vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv \n",
///     ),
/// );
/// ```
///
pub struct Highlight {
    target: Target,
    border: Border,
}

impl Highlight {
    pub fn frame(border: Border) -> Self {
        Self::new(Target::Frame, border)
    }

    pub fn cell(row: usize, column: usize, border: Border) -> Self {
        Self::new(Target::Cell { row, column }, border)
    }

    pub fn row(row: usize, border: Border) -> Self {
        Self::row_range(row, row + 1, border)
    }

    pub fn row_range(start: usize, end: usize, border: Border) -> Self {
        assert!(end > start);
        Self::new(
            Target::Row {
                from: start,
                to: end,
            },
            border,
        )
    }

    pub fn column(column: usize, border: Border) -> Self {
        Self::column_range(column, column + 1, border)
    }

    pub fn column_range(start: usize, end: usize, border: Border) -> Self {
        assert!(end > start);
        Self::new(
            Target::Column {
                from: start,
                to: end,
            },
            border,
        )
    }

    fn new(target: Target, border: Border) -> Self {
        Self { target, border }
    }
}

pub enum Target {
    Cell { row: usize, column: usize },
    Row { from: usize, to: usize },
    Column { from: usize, to: usize },
    Frame,
}

impl TableOption for Highlight {
    fn change(&mut self, grid: &mut Grid) {
        let sector = match self.target {
            Target::Frame => GridSector::new(0, grid.count_columns(), 0, grid.count_rows()),
            Target::Cell { row, column } => GridSector::new(column, column + 1, row, row + 1),
            Target::Row { from, to } => GridSector::new(0, grid.count_columns(), from, to),
            Target::Column { from, to } => GridSector::new(from, to, 0, grid.count_rows()),
        };

        set_border(grid, sector, self.border.clone());
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
struct GridSector {
    left: usize,
    right: usize,
    top: usize,
    bottom: usize,
}

impl GridSector {
    fn new(left: usize, right: usize, top: usize, bottom: usize) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }
}

fn set_border(grid: &mut Grid, sector: GridSector, border: Border) {
    let first_row = sector.top;
    let first_col = sector.left;

    let last_row = if sector.bottom > 0 {
        sector.bottom - 1
    } else {
        sector.bottom
    };

    let last_col = if sector.right > 0 {
        sector.right - 1
    } else {
        sector.right
    };

    if let Some(c) = border.top {
        let border = Border::default().top(c);
        for column in sector.left..sector.right {
            grid.set(
                &Entity::Cell(first_row, column),
                Settings::default()
                    .border_restriction(false)
                    .border(border.clone()),
            );
        }

        let border = Border::default().top_left_corner(c);
        for column in sector.left + 1..sector.right {
            grid.set(
                &Entity::Cell(first_row, column),
                Settings::default()
                    .border_restriction(false)
                    .border(border.clone()),
            );
        }
    }

    if let Some(c) = border.bottom {
        let border = Border::default().bottom(c);
        for column in sector.left..sector.right {
            grid.set(
                &Entity::Cell(last_row, column),
                Settings::default()
                    .border_restriction(false)
                    .border(border.clone()),
            );
        }

        let border = Border::default().bottom_left_corner(c);
        for column in sector.left + 1..sector.right {
            grid.set(
                &Entity::Cell(last_row, column),
                Settings::default()
                    .border_restriction(false)
                    .border(border.clone()),
            );
        }
    }

    if let Some(c) = border.left {
        let border = Border::default().left(c);
        for row in sector.top..sector.bottom {
            grid.set(
                &Entity::Cell(row, first_col),
                Settings::default()
                    .border_restriction(false)
                    .border(border.clone()),
            );
        }

        let border = Border::default().top_left_corner(c);
        for row in sector.top + 1..sector.bottom {
            grid.set(
                &Entity::Cell(row, first_col),
                Settings::default()
                    .border_restriction(false)
                    .border(border.clone()),
            );
        }
    }

    if let Some(c) = border.right {
        let border = Border::default().right(c);
        for row in sector.top..sector.bottom {
            grid.set(
                &Entity::Cell(row, last_col),
                Settings::default()
                    .border_restriction(false)
                    .border(border.clone()),
            );
        }

        let border = Border::default().top_right_corner(c);
        for row in sector.top + 1..sector.bottom {
            grid.set(
                &Entity::Cell(row, last_col),
                Settings::default()
                    .border_restriction(false)
                    .border(border.clone()),
            );
        }
    }

    if let Some(c) = border.left_top_corner {
        let border = Border::default().top_left_corner(c);
        grid.set(
            &Entity::Cell(first_row, first_col),
            Settings::default().border_restriction(false).border(border),
        );
    }

    if let Some(c) = border.left_bottom_corner {
        let border = Border::default().bottom_left_corner(c);
        grid.set(
            &Entity::Cell(last_row, first_col),
            Settings::default().border_restriction(false).border(border),
        );
    }

    if let Some(c) = border.right_top_corner {
        let border = Border::default().top_right_corner(c);
        grid.set(
            &Entity::Cell(first_row, last_col),
            Settings::default().border_restriction(false).border(border),
        );
    }

    if let Some(c) = border.right_bottom_corner {
        let border = Border::default().bottom_right_corner(c);
        grid.set(
            &Entity::Cell(last_row, last_col),
            Settings::default().border_restriction(false).border(border),
        );
    }
}
