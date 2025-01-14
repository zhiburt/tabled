//! This module contains a [`Highlight`] primitive, which helps
//! changing a [`Border`] style of any segment on a [`Table`].
//!
//! [`Table`]: crate::Table

use std::collections::HashSet;

use crate::{
    grid::{
        ansi::ANSIBuf,
        config::{Border, ColoredConfig, Entity, Position, SpannedConfig},
        records::{ExactRecords, Records},
    },
    settings::{
        object::Object,
        style::{Border as ConstBorder, BorderColor},
        Color, TableOption,
    },
};

/// Highlight modifies a table style by changing a border of a target [`Table`] segment.
///
/// It basically highlights outer border of a given segment.
///
/// # Example
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
///                .with(Highlight::outline(Segment::all().not((0,0).and((1, 0)).and((0, 1)).and((0, 3))), '*'))
///                .to_string();
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
    border: Option<Border<char>>,
    color: Option<Border<ANSIBuf>>,
}

impl<O> Highlight<O> {
    /// Build a new instance of [`Highlight`] with '*' char being used as changer.
    ///
    /// BE AWARE: if target exceeds boundaries it may panic.
    pub const fn new(target: O) -> Self {
        Self::_new(target, None, None)
    }

    /// Build a new instance of [`Highlight`],
    /// highlighting by a character.
    ///
    /// BE AWARE: if target exceeds boundaries it may panic.
    pub const fn outline(target: O, c: char) -> Self {
        Self::_new(target, Some(Border::filled(c)), None)
    }

    /// Build a new instance of [`Highlight`],
    /// highlighting by a color and a given character for a border.
    ///
    /// BE AWARE: if target exceeds boundaries it may panic.
    pub fn colored(target: O, color: Color) -> Self {
        let color = Border::filled(&color).cloned().convert();
        Self::_new(target, None, Some(color))
    }

    /// Set a border for a [`Highlight`].
    pub fn border<T, B, L, R>(self, border: ConstBorder<T, B, L, R>) -> Self {
        let border = border.into_inner();
        Self {
            target: self.target,
            border: Some(border),
            color: self.color,
        }
    }

    /// Set a border color for a [`Highlight`].
    pub fn color(self, border: BorderColor) -> Self {
        let border = border.into_inner();
        let border = border.convert();

        Self {
            target: self.target,
            border: self.border,
            color: Some(border),
        }
    }

    /// Build a new instance of [`Highlight`]
    ///
    /// BE AWARE: if target exceeds boundaries it may panic.
    const fn _new(target: O, border: Option<Border<char>>, color: Option<Border<ANSIBuf>>) -> Self {
        Self {
            target,
            border,
            color,
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

        match (self.border, self.color) {
            (None, Some(color)) => {
                for sector in segments {
                    set_border_color(cfg, &sector, &color);
                }
            }
            (Some(border), None) => {
                for sector in segments {
                    set_border(cfg, &sector, border);
                }
            }
            (Some(border), Some(color)) => {
                for sector in segments {
                    set_border(cfg, &sector, border);
                    set_border_color(cfg, &sector, &color);
                }
            }
            (None, None) => {
                // noop
            }
        }
    }

    fn hint_change(&self) -> Option<Entity> {
        None
    }
}

fn set_border_color(cfg: &mut SpannedConfig, sector: &HashSet<Position>, border: &Border<ANSIBuf>) {
    if sector.is_empty() {
        return;
    }
    let color = border.clone();
    for &p in sector {
        let border = build_cell_border(sector, p, &color);
        cfg.set_border_color(p, border);
    }
}

fn split_segments(
    cells: impl Iterator<Item = Entity>,
    count_rows: usize,
    count_cols: usize,
) -> Vec<HashSet<Position>> {
    let mut segments: Vec<HashSet<Position>> = Vec::new();
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

    let mut squashed_segments: Vec<HashSet<Position>> = Vec::new();
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

fn is_cell_connected(p1: Position, p2: Position) -> bool {
    if p1.col() == p2.col() {
        let up = p1.row() == p2.row() + 1;
        let down = p2.row() > 0 && p1.row() == p2.row() - 1;

        if up || down {
            return true;
        }
    }

    if p1.row() == p2.row() {
        let left = p2.col() > 0 && p1.col() == p2.col() - 1;
        let right = p1.col() == p2.col() + 1;

        if left || right {
            return true;
        }
    }

    false
}

fn is_segment_connected(segment1: &HashSet<Position>, segment2: &HashSet<Position>) -> bool {
    for &cell1 in segment1.iter() {
        for &cell2 in segment2.iter() {
            if is_cell_connected(cell1, cell2) {
                return true;
            }
        }
    }

    false
}

fn set_border(cfg: &mut SpannedConfig, sector: &HashSet<Position>, border: Border<char>) {
    if sector.is_empty() {
        return;
    }

    for &pos in sector {
        let border = build_cell_border(sector, pos, &border);
        cfg.set_border(pos, border);
    }
}

fn build_cell_border<T>(sector: &HashSet<Position>, p: Position, border: &Border<T>) -> Border<T>
where
    T: Default + Clone,
{
    let has_top_neighbor = has_top_neighbor(sector, p);
    let has_bottom_neighbor = has_bottom_neighbor(sector, p);
    let has_left_neighbor = has_left_neighbor(sector, p);
    let has_right_neighbor = has_right_neighbor(sector, p);
    let has_left_top_neighbor = has_left_top_neighbor(sector, p);
    let has_right_top_neighbor = has_right_top_neighbor(sector, p);
    let has_left_bottom_neighbor = has_left_bottom_neighbor(sector, p);
    let has_right_bottom_neighbor = has_right_bottom_neighbor(sector, p);

    let mut b = Border::default();

    if let Some(c) = border.top.clone() {
        if !has_top_neighbor {
            b.top = Some(c.clone());

            if has_right_neighbor && !has_right_top_neighbor {
                b.right_top_corner = Some(c);
            }
        }
    }

    if let Some(c) = border.bottom.clone() {
        if !has_bottom_neighbor {
            b.bottom = Some(c.clone());

            if has_right_neighbor && !has_right_bottom_neighbor {
                b.right_bottom_corner = Some(c);
            }
        }
    }

    if let Some(c) = border.left.clone() {
        if !has_left_neighbor {
            b.left = Some(c.clone());

            if has_bottom_neighbor && !has_left_bottom_neighbor {
                b.left_bottom_corner = Some(c);
            }
        }
    }

    if let Some(c) = border.right.clone() {
        if !has_right_neighbor {
            b.right = Some(c.clone());

            if has_bottom_neighbor && !has_right_bottom_neighbor {
                b.right_bottom_corner = Some(c);
            }
        }
    }

    if let Some(c) = border.left_top_corner.clone() {
        if !has_left_neighbor && !has_top_neighbor {
            b.left_top_corner = Some(c);
        }
    }

    if let Some(c) = border.left_bottom_corner.clone() {
        if !has_left_neighbor && !has_bottom_neighbor {
            b.left_bottom_corner = Some(c);
        }
    }

    if let Some(c) = border.right_top_corner.clone() {
        if !has_right_neighbor && !has_top_neighbor {
            b.right_top_corner = Some(c);
        }
    }

    if let Some(c) = border.right_bottom_corner.clone() {
        if !has_right_neighbor && !has_bottom_neighbor {
            b.right_bottom_corner = Some(c);
        }
    }

    {
        if !has_bottom_neighbor {
            if !has_left_neighbor && has_left_top_neighbor {
                if let Some(c) = border.right_top_corner.clone() {
                    b.left_top_corner = Some(c);
                }
            }

            if has_left_neighbor && has_left_bottom_neighbor {
                if let Some(c) = border.left_top_corner.clone() {
                    b.left_bottom_corner = Some(c);
                }
            }

            if !has_right_neighbor && has_right_top_neighbor {
                if let Some(c) = border.left_top_corner.clone() {
                    b.right_top_corner = Some(c);
                }
            }

            if has_right_neighbor && has_right_bottom_neighbor {
                if let Some(c) = border.right_top_corner.clone() {
                    b.right_bottom_corner = Some(c);
                }
            }
        }

        if !has_top_neighbor {
            if !has_left_neighbor && has_left_bottom_neighbor {
                if let Some(c) = border.right_bottom_corner.clone() {
                    b.left_bottom_corner = Some(c);
                }
            }

            if has_left_neighbor && has_left_top_neighbor {
                if let Some(c) = border.left_bottom_corner.clone() {
                    b.left_top_corner = Some(c);
                }
            }

            if !has_right_neighbor && has_right_bottom_neighbor {
                if let Some(c) = border.left_bottom_corner.clone() {
                    b.right_bottom_corner = Some(c);
                }
            }

            if has_right_neighbor && has_right_top_neighbor {
                if let Some(c) = border.right_bottom_corner.clone() {
                    b.right_top_corner = Some(c);
                }
            }
        }
    }

    b
}

fn has_top_neighbor(sector: &HashSet<Position>, p: Position) -> bool {
    p.row() > 0 && sector.contains(&(p - (1, 0)))
}

fn has_bottom_neighbor(sector: &HashSet<Position>, p: Position) -> bool {
    sector.contains(&(p + (1, 0)))
}

fn has_left_neighbor(sector: &HashSet<Position>, p: Position) -> bool {
    p.col() > 0 && sector.contains(&(p - (0, 1)))
}

fn has_right_neighbor(sector: &HashSet<Position>, p: Position) -> bool {
    sector.contains(&(p + (0, 1)))
}

fn has_left_top_neighbor(sector: &HashSet<Position>, p: Position) -> bool {
    p.row() > 0 && p.col() > 0 && sector.contains(&(p - (1, 1)))
}

fn has_right_top_neighbor(sector: &HashSet<Position>, p: Position) -> bool {
    p.row() > 0 && sector.contains(&(p - (1, 0) + (0, 1)))
}

fn has_left_bottom_neighbor(sector: &HashSet<Position>, p: Position) -> bool {
    p.col() > 0 && sector.contains(&(p + (1, 0) - (0, 1)))
}

fn has_right_bottom_neighbor(sector: &HashSet<Position>, p: Position) -> bool {
    sector.contains(&(p + (1, 1)))
}

#[cfg(test)]
mod tests {
    use papergrid::config::pos;

    use super::*;

    #[test]
    fn test_is_connected() {
        assert!(is_cell_connected(pos(0, 0), pos(0, 1)));
        assert!(is_cell_connected(pos(0, 0), pos(1, 0)));
        assert!(!is_cell_connected(pos(0, 0), pos(1, 1)));

        assert!(is_cell_connected(pos(0, 1), pos(0, 0)));
        assert!(is_cell_connected(pos(1, 0), pos(0, 0)));
        assert!(!is_cell_connected(pos(1, 1), pos(0, 0)));

        assert!(is_cell_connected(pos(1, 1), pos(0, 1)));
        assert!(is_cell_connected(pos(1, 1), pos(1, 0)));
        assert!(is_cell_connected(pos(1, 1), pos(2, 1)));
        assert!(is_cell_connected(pos(1, 1), pos(1, 2)));
        assert!(!is_cell_connected(pos(1, 1), pos(1, 1)));
    }
}
