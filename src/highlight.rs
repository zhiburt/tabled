use std::collections::HashSet;

#[allow(unused)]
use crate::Table;
use crate::{
    object::{Frame, Object},
    style::Border,
    TableOption,
};
use papergrid::{Entity, Grid, Settings};

/// Highlight modifies a table style by changing a border of a target [Table] segment.
///
/// [Default] implementation runs Highlight for a [Frame].
///
/// # Example
///
/// ```
/// use tabled::{TableIteratorExt, Highlight, style::{Border, Style}, object::Full};
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
///                .with(Highlight::new(Full, Border::default().top('^').bottom('v')))
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
/// It's possible to use [Highlight] for many kinds of figures.
///
///
/// ```
/// use tabled::{TableIteratorExt, Highlight, style::{Border, Style}, object::{Full, Cell, Object}};
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
///                .with(Highlight::new(Full.not(Cell(0,0).and(Cell(1, 0).and(Cell(0, 1)).and(Cell(0, 3)))), Border::filled('*')))
///                .to_string();
///
/// assert_eq!(
///     table,
///     concat!(
///         "                *****************************        \n",
///         "| usize | &str  *           &str            * bool  |\n",
///         "|-------*********---------------------------*********\n",
///         "|   0   *  ELF  | Extensible Linking Format | true  *\n",
///         "*********                                           *\n",
///         "*   1   | DWARF |                           | true  *\n",
///         "*                                                   *\n",
///         "*   2   |  PE   |    Portable Executable    | false *\n",
///         "*****************************************************\n",
///     ),
/// );
/// ```
///
pub struct Highlight<O> {
    target: O,
    border: Border,
}

impl Default for Highlight<Frame> {
    fn default() -> Self {
        Self {
            target: Frame,
            border: Default::default(),
        }
    }
}

impl<O> Highlight<O>
where
    O: Object,
{
    /// Build a new instance of [Highlight]
    ///
    /// BE AWARE: if target exeeds boundries it may panic.
    pub fn new(target: O, border: Border) -> Self {
        Self { target, border }
    }
}

impl<O> TableOption for Highlight<O>
where
    O: Object,
{
    fn change(&mut self, grid: &mut Grid) {
        let cells = self.target.cells(grid.count_rows(), grid.count_columns());
        let segments = split_segments(cells);

        for sector in segments {
            set_border(grid, sector, self.border.clone());
        }
    }
}

fn split_segments(cells: Vec<(usize, usize)>) -> Vec<HashSet<(usize, usize)>> {
    let mut segments: Vec<HashSet<(usize, usize)>> = Vec::new();
    for cell in cells {
        let found_segment = segments
            .iter_mut()
            .find(|s| s.iter().any(|&c| is_cell_connected(cell, c)));

        match found_segment {
            Some(segment) => {
                segment.insert(cell);
            }
            None => {
                let mut segment = HashSet::new();
                segment.insert(cell);
                segments.push(segment);
            }
        }
    }

    let mut squashed_segments: Vec<HashSet<(usize, usize)>> = Vec::new();
    while !segments.is_empty() {
        let mut segment = segments.remove(0);

        let mut i = 0;
        while i < segments.len() {
            if is_segment_connected(&segment, &segments[i]) {
                segment.extend(&segments[i]);
                segments.remove(i);
            } else {
                i += 1;
            }
        }

        squashed_segments.push(segment);
    }

    squashed_segments
}

fn is_cell_connected((row1, col1): (usize, usize), (row2, col2): (usize, usize)) -> bool {
    if col1 == col2 && row1 == row2 + 1 {
        return true;
    }

    if col1 == col2 && (row2 > 0 && row1 == row2 - 1) {
        return true;
    }

    if row1 == row2 && col1 == col2 + 1 {
        return true;
    }

    if row1 == row2 && (col2 > 0 && col1 == col2 - 1) {
        return true;
    }

    false
}

fn is_segment_connected(
    segment1: &HashSet<(usize, usize)>,
    segment2: &HashSet<(usize, usize)>,
) -> bool {
    for &cell1 in segment1.iter() {
        for &cell2 in segment2.iter() {
            if is_cell_connected(cell1, cell2) {
                return true;
            }
        }
    }

    false
}

fn set_border(grid: &mut Grid, sector: HashSet<(usize, usize)>, border: Border) {
    if sector.is_empty() {
        return;
    }

    for &(row, col) in &sector {
        let mut cell_border = Border::default();

        let cell_has_top_neighbor = cell_has_top_neighbor(&sector, row, col);
        let cell_has_bottom_neighbor = cell_has_bottom_neighbor(&sector, row, col);
        let cell_has_left_neighbor = cell_has_left_neighbor(&sector, row, col);
        let cell_has_right_neighbor = cell_has_right_neighbor(&sector, row, col);

        let this_has_left_top_neighbor = is_there_left_top_cell(&sector, row, col);
        let this_has_right_top_neighbor = is_there_right_top_cell(&sector, row, col);
        let this_has_left_bottom_neighbor = is_there_left_bottom_cell(&sector, row, col);
        let this_has_right_bottom_neighbor = is_there_right_bottom_cell(&sector, row, col);

        if let Some(c) = border.top {
            if !cell_has_top_neighbor {
                cell_border = cell_border.top(c);

                if cell_has_right_neighbor && !this_has_right_top_neighbor {
                    cell_border = cell_border.top_right_corner(c);
                }
            }
        }

        if let Some(c) = border.bottom {
            if !cell_has_bottom_neighbor {
                cell_border = cell_border.bottom(c);

                if cell_has_right_neighbor && !this_has_right_bottom_neighbor {
                    cell_border = cell_border.bottom_right_corner(c);
                }
            }
        }

        if let Some(c) = border.left {
            if !cell_has_left_neighbor {
                cell_border = cell_border.left(c);

                if cell_has_bottom_neighbor && !this_has_left_bottom_neighbor {
                    cell_border = cell_border.bottom_left_corner(c);
                }
            }
        }

        if let Some(c) = border.right {
            if !cell_has_right_neighbor {
                cell_border = cell_border.right(c);

                if cell_has_bottom_neighbor && !this_has_right_bottom_neighbor {
                    cell_border = cell_border.bottom_right_corner(c);
                }
            }
        }

        if let Some(c) = border.left_top_corner {
            if !cell_has_left_neighbor && !cell_has_top_neighbor {
                cell_border = cell_border.top_left_corner(c);
            }
        }

        if let Some(c) = border.left_bottom_corner {
            if !cell_has_left_neighbor && !cell_has_bottom_neighbor {
                cell_border = cell_border.bottom_left_corner(c);
            }
        }

        if let Some(c) = border.right_top_corner {
            if !cell_has_right_neighbor && !cell_has_top_neighbor {
                cell_border = cell_border.top_right_corner(c);
            }
        }

        if let Some(c) = border.right_bottom_corner {
            if !cell_has_right_neighbor && !cell_has_bottom_neighbor {
                cell_border = cell_border.bottom_right_corner(c);
            }
        }

        {
            if !cell_has_bottom_neighbor {
                if !cell_has_left_neighbor && this_has_left_top_neighbor {
                    if let Some(c) = border.right_top_corner {
                        cell_border = cell_border.top_left_corner(c);
                    }
                }

                if cell_has_left_neighbor && this_has_left_bottom_neighbor {
                    if let Some(c) = border.left_top_corner {
                        cell_border = cell_border.bottom_left_corner(c);
                    }
                }

                if !cell_has_right_neighbor && this_has_right_top_neighbor {
                    if let Some(c) = border.left_top_corner {
                        cell_border = cell_border.top_right_corner(c);
                    }
                }

                if cell_has_right_neighbor && this_has_right_bottom_neighbor {
                    if let Some(c) = border.right_top_corner {
                        cell_border = cell_border.bottom_right_corner(c);
                    }
                }
            }

            if !cell_has_top_neighbor {
                if !cell_has_left_neighbor && this_has_left_bottom_neighbor {
                    if let Some(c) = border.right_bottom_corner {
                        cell_border = cell_border.bottom_left_corner(c);
                    }
                }

                if cell_has_left_neighbor && this_has_left_top_neighbor {
                    if let Some(c) = border.left_bottom_corner {
                        cell_border = cell_border.top_left_corner(c);
                    }
                }

                if !cell_has_right_neighbor && this_has_right_bottom_neighbor {
                    if let Some(c) = border.left_bottom_corner {
                        cell_border = cell_border.bottom_right_corner(c);
                    }
                }

                if cell_has_right_neighbor && this_has_right_top_neighbor {
                    if let Some(c) = border.right_bottom_corner {
                        cell_border = cell_border.top_right_corner(c);
                    }
                }
            }
        }

        grid.set(
            &Entity::Cell(row, col),
            Settings::default()
                .border(cell_border)
                .border_restriction(false),
        );
    }
}

fn cell_has_top_neighbor(sector: &HashSet<(usize, usize)>, row: usize, col: usize) -> bool {
    row > 0 && sector.contains(&(row - 1, col))
}

fn cell_has_bottom_neighbor(sector: &HashSet<(usize, usize)>, row: usize, col: usize) -> bool {
    sector.contains(&(row + 1, col))
}

fn cell_has_left_neighbor(sector: &HashSet<(usize, usize)>, row: usize, col: usize) -> bool {
    col > 0 && sector.contains(&(row, col - 1))
}

fn cell_has_right_neighbor(sector: &HashSet<(usize, usize)>, row: usize, col: usize) -> bool {
    sector.contains(&(row, col + 1))
}

fn is_there_left_top_cell(sector: &HashSet<(usize, usize)>, row: usize, col: usize) -> bool {
    row > 0 && col > 0 && sector.contains(&(row - 1, col - 1))
}

fn is_there_right_top_cell(sector: &HashSet<(usize, usize)>, row: usize, col: usize) -> bool {
    row > 0 && sector.contains(&(row - 1, col + 1))
}

fn is_there_left_bottom_cell(sector: &HashSet<(usize, usize)>, row: usize, col: usize) -> bool {
    col > 0 && sector.contains(&(row + 1, col - 1))
}

fn is_there_right_bottom_cell(sector: &HashSet<(usize, usize)>, row: usize, col: usize) -> bool {
    sector.contains(&(row + 1, col + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_connected() {
        assert!(is_cell_connected((0, 0), (0, 1)));
        assert!(is_cell_connected((0, 0), (1, 0)));
        assert!(!is_cell_connected((0, 0), (1, 1)));

        assert!(is_cell_connected((0, 1), (0, 0)));
        assert!(is_cell_connected((1, 0), (0, 0)));
        assert!(!is_cell_connected((1, 1), (0, 0)));

        assert!(is_cell_connected((1, 1), (0, 1)));
        assert!(is_cell_connected((1, 1), (1, 0)));
        assert!(is_cell_connected((1, 1), (2, 1)));
        assert!(is_cell_connected((1, 1), (1, 2)));
        assert!(!is_cell_connected((1, 1), (1, 1)));
    }
}
