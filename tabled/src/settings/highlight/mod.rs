//! This module contains a [`Highlight`] primitive, which helps
//! changing a [`Border`] style of any segment on a [`Table`].
//!
//! [`Table`]: crate::Table

use std::collections::HashSet;

use crate::{
    grid::{
        ansi::ANSIBuf,
        config::{Border as GridBorder, ColoredConfig, Entity, Position, SpannedConfig},
        records::{ExactRecords, Records},
    },
    settings::{object::Object, style::BorderColor, Border, Color, TableOption},
};

/// Highlight modifies a table style by changing a border of a target [`Table`] segment.
///
/// # Example
///
/// ```
/// use tabled::{
///     Table,
///     settings::{Highlight, Border, Style, object::Segment}
/// };
///
/// let data = [
///     ("ELF", "Extensible Linking Format", true),
///     ("DWARF", "", true),
///     ("PE", "Portable Executable", false),
/// ];
///
/// let table = Table::new(data.iter().enumerate())
///                .with(Style::markdown())
///                .with(Highlight::border(Segment::all(), Border::new().set_top('^').set_bottom('v')))
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
/// use tabled::{
///     Table,
///     settings::{
///         Highlight, Border, Style,
///         object::{Segment, Object}
///     }
/// };
///
/// let data = [
///     ("ELF", "Extensible Linking Format", true),
///     ("DWARF", "", true),
///     ("PE", "Portable Executable", false),
/// ];
///
/// let table = Table::new(data.iter().enumerate())
///                .with(Style::markdown())
///                .with(Highlight::border(Segment::all().not((0,0).and((1, 0)).and((0, 1)).and((0, 3))), Border::filled('*')))
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
    border: HighlightInner,
}

#[derive(Debug)]
enum HighlightInner {
    Border(GridBorder<char>),
    // #[cfg(feature = "ansi")]
    Color(GridBorder<ANSIBuf>),
    // #[cfg(feature = "ansi")]
    ColoredBorder(GridBorder<char>, GridBorder<ANSIBuf>),
}

// todo: Docs testss

// todo: Move colors to Highlight itself
// todo: Maybe add a default Border and set ask for a char -> Highlight::new(target).border().outline().color().outline_color()

// todo: Add BorderColor.

impl<O> Highlight<O> {
    /// Build a new instance of [`Highlight`]
    ///
    /// BE AWARE: if target exceeds boundaries it may panic.
    pub const fn border<T, B, L, R>(target: O, border: Border<T, B, L, R>) -> Self {
        Self {
            target,
            border: HighlightInner::Border(border.into_inner()),
        }
    }

    /// Build a new instance of [`Highlight`]
    ///
    /// BE AWARE: if target exceeds boundaries it may panic.
    // #[cfg(feature = "ansi")]
    pub fn color<T, B, L, R>(target: O, border: BorderColor<T, B, L, R>) -> Self {
        let color = border.into_inner();
        let color = color.convert();

        Self {
            target,
            border: HighlightInner::Color(color),
        }
    }

    /// Build a new instance of [`Highlight`]
    ///
    /// BE AWARE: if target exceeds boundaries it may panic.
    pub const fn outline(target: O, c: char) -> Self {
        Self::border(target, Border::filled(c))
    }

    /// Build a new instance of [`Highlight`]
    ///
    /// BE AWARE: if target exceeds boundaries it may panic.
    pub fn colored_outline<T, B, L, R>(target: O, c: char, color: Color) -> Self {
        Self::colored_border(target, Border::filled(c), BorderColor::filled(color))
    }

    /// Build a new instance of [`Highlight`]
    ///
    /// BE AWARE: if target exceeds boundaries it may panic.
    // #[cfg(feature = "ansi")]
    pub fn colored_border<T, B, L, R>(
        target: O,
        border: Border<T, B, L, R>,
        color: BorderColor<T, B, L, R>,
    ) -> Self {
        let border = border.into_inner();
        let color = color.into_inner().convert();

        Self {
            target,
            border: HighlightInner::ColoredBorder(border, color),
        }
    }
}

impl<O, R, D> TableOption<R, ColoredConfig, D> for Highlight<O>
where
    O: Object<R>,
    R: Records + ExactRecords,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        let count_rows = records.count_rows();
        let count_cols = records.count_columns();

        let cells = self.target.cells(records);
        let segments = split_segments(cells, count_rows, count_cols);

        match self.border {
            HighlightInner::Border(border) => {
                for sector in segments {
                    set_border(cfg, &sector, border);
                }
            }
            HighlightInner::Color(color) => {
                for sector in segments {
                    set_border_color(cfg, &sector, &color);
                }
            }
            HighlightInner::ColoredBorder(border, color) => {
                for sector in segments {
                    set_border(cfg, &sector, border);
                    set_border_color(cfg, &sector, &color);
                }
            }
        }
    }

    fn hint_change(&self) -> Option<Entity> {
        None
    }
}

fn set_border_color(
    cfg: &mut SpannedConfig,
    sector: &HashSet<(usize, usize)>,
    border: &GridBorder<ANSIBuf>,
) {
    if sector.is_empty() {
        return;
    }
    let color = border.clone();
    for &(row, col) in sector {
        let border = build_cell_border(sector, (row, col), &color);
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
                    let _ = segment.insert(cell);
                }
                None => {
                    let mut segment = HashSet::new();
                    let _ = segment.insert(cell);
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
                let _ = segments.remove(i);
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

fn set_border(cfg: &mut SpannedConfig, sector: &HashSet<(usize, usize)>, border: GridBorder<char>) {
    if sector.is_empty() {
        return;
    }

    for &pos in sector {
        let border = build_cell_border(sector, pos, &border);
        cfg.set_border(pos, border);
    }
}

fn build_cell_border<T>(
    sector: &HashSet<(usize, usize)>,
    (row, col): Position,
    border: &GridBorder<T>,
) -> GridBorder<T>
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

    let mut cell_border = GridBorder::default();
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

// fn split_tuple_border<A, B>(border: GridBorder<(A, B)>) -> (GridBorder<A>, GridBorder<B>) {
//     macro_rules! set_border {
//         ($border1:expr, $border2:expr => $symbl:expr) => {
//             if let Some((x1, x2)) = $symbl {
//                 $border1 = Some(x1);
//                 $border2 = Some(x2);
//             }
//         };
//     }

//     let mut a = GridBorder::empty();
//     let mut b = GridBorder::empty();

//     set_border!(a.bottom, b.bottom => border.bottom);
//     set_border!(a.top, b.top => border.top);
//     set_border!(a.left, b.left => border.left);
//     set_border!(a.left_top_corner, b.left_top_corner => border.left_top_corner);
//     set_border!(a.left_bottom_corner, b.left_bottom_corner => border.left_bottom_corner);
//     set_border!(a.right, b.right => border.right);
//     set_border!(a.right_top_corner, b.right_top_corner => border.right_top_corner);
//     set_border!(a.right_bottom_corner, b.right_bottom_corner => border.right_bottom_corner);

//     (a, b)
// }

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
