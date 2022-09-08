//! This module contains a [`Highlight`] primitive, which helps
//! changing a [`Border`] style of any segment on a [`Table`].
//!
//! [`Table`]: crate::Table

use std::collections::HashSet;

use papergrid::{records::Records, Entity, GridConfig, Position};

use crate::{object::Object, Border, Table, TableOption};

#[cfg(feature = "color")]
use crate::style::BorderColored;

/// Highlight modifies a table style by changing a border of a target [`Table`] segment.
///
/// # Example
///
/// ```
/// use tabled::{TableIteratorExt, Highlight, Border, Style, object::Segment};
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
///                .with(Style::markdown())
///                .with(Highlight::new(Segment::all(), Border::default().top('^').bottom('v')))
///                .to_string();
///
/// assert_eq!(
///     table,
///     concat!(
///         " ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ \n",
///         "| usize | &str  | &str                      | bool  |\n",
///         "|-------|-------|---------------------------|-------|\n",
///         "| 0     | ELF   | Extensible Linking Format | true  |\n",
///         "| 1     | DWARF |                           | true  |\n",
///         "| 2     | PE    | Portable Executable       | false |\n",
///         " vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv ",
///     ),
/// );
/// ```
///
/// It's possible to use [`Highlight`] for many kinds of figures.
///
/// ```
/// use tabled::{TableIteratorExt, Highlight, Border, Style, object::{Segment, Cell, Object}};
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
///                .with(Style::markdown())
///                .with(Highlight::new(Segment::all().not(Cell(0,0).and(Cell(1, 0).and(Cell(0, 1)).and(Cell(0, 3)))), Border::filled('*')))
///                .to_string();
///
/// println!("{}", table);
///
/// assert_eq!(
///     table,
///     concat!(
///         "                *****************************        \n",
///         "| usize | &str  * &str                      * bool  |\n",
///         "|-------*********---------------------------*********\n",
///         "| 0     * ELF   | Extensible Linking Format | true  *\n",
///         "*********                                           *\n",
///         "* 1     | DWARF |                           | true  *\n",
///         "*                                                   *\n",
///         "* 2     | PE    | Portable Executable       | false *\n",
///         "*****************************************************",
///     ),
/// );
/// ```
///
/// [`Table`]: crate::Table
#[derive(Debug)]
pub struct Highlight<O> {
    target: O,
    border: Border,
}

impl<O> Highlight<O>
where
    O: Object,
{
    /// Build a new instance of [`Highlight`]
    ///
    /// BE AWARE: if target exceeds boundaries it may panic.
    pub fn new(target: O, border: Border) -> Self {
        Self { target, border }
    }
}

impl<O> Highlight<O> {
    /// Build a new instance of [`HighlightColored`]
    #[cfg(feature = "color")]
    #[cfg_attr(docsrs, doc(cfg(feature = "color")))]
    pub fn colored(target: O, border: BorderColored) -> HighlightColored<O> {
        HighlightColored { target, border }
    }
}

impl<O, R> TableOption<R> for Highlight<O>
where
    O: Object,
    R: Records,
{
    fn change(&mut self, table: &mut Table<R>) {
        let (count_rows, count_cols) = table.shape();
        let cells = self.target.cells(table);
        let segments = split_segments(cells, count_rows, count_cols);

        for sector in segments {
            set_border(table.get_config_mut(), &sector, self.border.clone());
        }

        table.destroy_width_cache();
    }
}

/// A [`Highlight`] object which works with a [`BorderColored`]
///
/// [`BorderColored`]: crate::style::BorderColored
#[cfg(feature = "color")]
#[cfg_attr(docsrs, doc(cfg(feature = "color")))]
#[derive(Debug)]
pub struct HighlightColored<O> {
    target: O,
    border: BorderColored,
}

#[cfg(feature = "color")]
impl<O, R> TableOption<R> for HighlightColored<O>
where
    O: Object,
    R: Records,
{
    fn change(&mut self, table: &mut Table<R>) {
        let (count_rows, count_cols) = table.shape();
        let cells = self.target.cells(table);
        let segments = split_segments(cells, count_rows, count_cols);

        for sector in segments {
            set_border_colored(table.get_config_mut(), sector, &self.border);
        }

        table.destroy_width_cache();
    }
}

#[cfg(feature = "color")]
fn set_border_colored(
    cfg: &mut GridConfig,
    sector: HashSet<(usize, usize)>,
    border: &BorderColored,
) {
    if sector.is_empty() {
        return;
    }

    let color = border.clone().into();
    for &(row, col) in &sector {
        let border = build_cell_border(&sector, (row, col), &color);
        cfg.set_border((row, col), border);
    }

    let color = border.clone().into();
    for &(row, col) in &sector {
        let border = build_cell_border(&sector, (row, col), &color);
        cfg.set_border_color((row, col), border);
    }
}

fn split_segments(
    cells: impl Iterator<Item = Entity>,
    count_rows: usize,
    count_cols: usize,
) -> Vec<HashSet<(usize, usize)>> {
    let mut segments: Vec<HashSet<(usize, usize)>> = Vec::new();
    for entity in cells {
        for cell in entity.iter(count_rows, count_cols) {
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

fn set_border(cfg: &mut GridConfig, sector: &HashSet<(usize, usize)>, border: Border) {
    if sector.is_empty() {
        return;
    }

    if let Some(border) = border.into() {
        for &pos in sector {
            let border = build_cell_border(sector, pos, &border);
            cfg.set_border(pos, border);
        }
    }
}

fn build_cell_border<T>(
    sector: &HashSet<(usize, usize)>,
    (row, col): Position,
    border: &papergrid::Border<T>,
) -> papergrid::Border<T>
where
    T: Default + Clone,
{
    let cell_has_top_neighbor = cell_has_top_neighbor(sector, row, col);
    let cell_has_bottom_neighbor = cell_has_bottom_neighbor(sector, row, col);
    let cell_has_left_neighbor = cell_has_left_neighbor(sector, row, col);
    let cell_has_right_neighbor = cell_has_right_neighbor(sector, row, col);

    let this_has_left_top_neighbor = is_there_left_top_cell(sector, row, col);
    let this_has_right_top_neighbor = is_there_right_top_cell(sector, row, col);
    let this_has_left_bottom_neighbor = is_there_left_bottom_cell(sector, row, col);
    let this_has_right_bottom_neighbor = is_there_right_bottom_cell(sector, row, col);

    let mut cell_border = papergrid::Border::default();
    if let Some(c) = border.top.clone() {
        if !cell_has_top_neighbor {
            cell_border.top = Some(c.clone());

            if cell_has_right_neighbor && !this_has_right_top_neighbor {
                cell_border.right_top_corner = Some(c);
            }
        }
    }
    if let Some(c) = border.bottom.clone() {
        if !cell_has_bottom_neighbor {
            cell_border.bottom = Some(c.clone());

            if cell_has_right_neighbor && !this_has_right_bottom_neighbor {
                cell_border.right_bottom_corner = Some(c);
            }
        }
    }
    if let Some(c) = border.left.clone() {
        if !cell_has_left_neighbor {
            cell_border.left = Some(c.clone());

            if cell_has_bottom_neighbor && !this_has_left_bottom_neighbor {
                cell_border.left_bottom_corner = Some(c);
            }
        }
    }
    if let Some(c) = border.right.clone() {
        if !cell_has_right_neighbor {
            cell_border.right = Some(c.clone());

            if cell_has_bottom_neighbor && !this_has_right_bottom_neighbor {
                cell_border.right_bottom_corner = Some(c);
            }
        }
    }
    if let Some(c) = border.left_top_corner.clone() {
        if !cell_has_left_neighbor && !cell_has_top_neighbor {
            cell_border.left_top_corner = Some(c);
        }
    }
    if let Some(c) = border.left_bottom_corner.clone() {
        if !cell_has_left_neighbor && !cell_has_bottom_neighbor {
            cell_border.left_bottom_corner = Some(c);
        }
    }
    if let Some(c) = border.right_top_corner.clone() {
        if !cell_has_right_neighbor && !cell_has_top_neighbor {
            cell_border.right_top_corner = Some(c);
        }
    }
    if let Some(c) = border.right_bottom_corner.clone() {
        if !cell_has_right_neighbor && !cell_has_bottom_neighbor {
            cell_border.right_bottom_corner = Some(c);
        }
    }
    {
        if !cell_has_bottom_neighbor {
            if !cell_has_left_neighbor && this_has_left_top_neighbor {
                if let Some(c) = border.right_top_corner.clone() {
                    cell_border.left_top_corner = Some(c);
                }
            }

            if cell_has_left_neighbor && this_has_left_bottom_neighbor {
                if let Some(c) = border.left_top_corner.clone() {
                    cell_border.left_bottom_corner = Some(c);
                }
            }

            if !cell_has_right_neighbor && this_has_right_top_neighbor {
                if let Some(c) = border.left_top_corner.clone() {
                    cell_border.right_top_corner = Some(c);
                }
            }

            if cell_has_right_neighbor && this_has_right_bottom_neighbor {
                if let Some(c) = border.right_top_corner.clone() {
                    cell_border.right_bottom_corner = Some(c);
                }
            }
        }

        if !cell_has_top_neighbor {
            if !cell_has_left_neighbor && this_has_left_bottom_neighbor {
                if let Some(c) = border.right_bottom_corner.clone() {
                    cell_border.left_bottom_corner = Some(c);
                }
            }

            if cell_has_left_neighbor && this_has_left_top_neighbor {
                if let Some(c) = border.left_bottom_corner.clone() {
                    cell_border.left_top_corner = Some(c);
                }
            }

            if !cell_has_right_neighbor && this_has_right_bottom_neighbor {
                if let Some(c) = border.left_bottom_corner.clone() {
                    cell_border.right_bottom_corner = Some(c);
                }
            }

            if cell_has_right_neighbor && this_has_right_top_neighbor {
                if let Some(c) = border.right_bottom_corner.clone() {
                    cell_border.right_top_corner = Some(c);
                }
            }
        }
    }

    cell_border
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
