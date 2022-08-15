use std::collections::{HashMap, HashSet};

use super::{Border, Position};

#[derive(Debug, Clone, Default)]
pub(crate) struct BordersConfig<T> {
    global: Option<T>,
    borders: Borders<T>,
    cells: BordersMap<T>,
    lines: HashMap<usize, Line<T>>,
    layout: BordersLayout,
}

impl<T: std::fmt::Debug> BordersConfig<T> {
    pub(crate) fn insert_border(&mut self, pos: Position, border: Border<T>) {
        if let Some(c) = border.top {
            self.cells.horizontal.insert(pos, c);
            self.layout.horizontal.insert(pos.0);
        }

        if let Some(c) = border.bottom {
            self.cells.horizontal.insert((pos.0 + 1, pos.1), c);
            self.layout.horizontal.insert(pos.0 + 1);
        }

        if let Some(c) = border.left {
            self.cells.vertical.insert(pos, c);
            self.layout.vertical.insert(pos.1);
        }

        if let Some(c) = border.right {
            self.cells.vertical.insert((pos.0, pos.1 + 1), c);
            self.layout.vertical.insert(pos.1 + 1);
        }

        if let Some(c) = border.left_top_corner {
            self.cells.intersection.insert((pos.0, pos.1), c);
            self.layout.horizontal.insert(pos.0);
            self.layout.vertical.insert(pos.1);
        }

        if let Some(c) = border.right_top_corner {
            self.cells.intersection.insert((pos.0, pos.1 + 1), c);
            self.layout.horizontal.insert(pos.0);
            self.layout.vertical.insert(pos.1 + 1);
        }

        if let Some(c) = border.left_bottom_corner {
            self.cells.intersection.insert((pos.0 + 1, pos.1), c);
            self.layout.horizontal.insert(pos.0 + 1);
            self.layout.vertical.insert(pos.1);
        }

        if let Some(c) = border.right_bottom_corner {
            self.cells.intersection.insert((pos.0 + 1, pos.1 + 1), c);
            self.layout.horizontal.insert(pos.0 + 1);
            self.layout.vertical.insert(pos.1 + 1);
        }
    }

    pub(crate) fn remove_border(&mut self, pos: Position, count_cols: usize) {
        self.cells.horizontal.remove(&pos);
        self.cells.horizontal.remove(&(pos.0 + 1, pos.1));
        self.cells.vertical.remove(&pos);
        self.cells.vertical.remove(&(pos.0, pos.1 + 1));
        self.cells.intersection.remove(&pos);
        self.cells.intersection.remove(&(pos.0 + 1, pos.1));
        self.cells.intersection.remove(&(pos.0, pos.1 + 1));
        self.cells.intersection.remove(&(pos.0 + 1, pos.1 + 1));

        // clean up the layout.

        if !self.check_is_horizontal_set(pos.0) {
            self.layout.horizontal.remove(&pos.0);
        }

        if !self.check_is_horizontal_set(pos.0 + 1) {
            self.layout.horizontal.remove(&(pos.0 + 1));
        }

        if !self.check_is_vertical_set(pos.1, count_cols) {
            self.layout.vertical.remove(&pos.1);
        }

        if !self.check_is_vertical_set(pos.1 + 1, count_cols) {
            self.layout.vertical.remove(&(pos.1 + 1));
        }
    }

    pub(crate) fn get_border(
        &self,
        pos: Position,
        count_rows: usize,
        count_cols: usize,
    ) -> Border<&T> {
        Border {
            top: self.get_horizontal(pos, count_rows),
            bottom: self.get_horizontal((pos.0 + 1, pos.1), count_rows),
            left: self.get_vertical(pos, count_cols),
            left_top_corner: self.get_intersection(pos, count_rows, count_cols),
            left_bottom_corner: self.get_intersection((pos.0 + 1, pos.1), count_rows, count_cols),
            right: self.get_vertical((pos.0, pos.1 + 1), count_cols),
            right_top_corner: self.get_intersection((pos.0, pos.1 + 1), count_rows, count_cols),
            right_bottom_corner: self.get_intersection(
                (pos.0 + 1, pos.1 + 1),
                count_rows,
                count_cols,
            ),
        }
    }

    pub(crate) fn insert_line(&mut self, row: usize, line: Line<T>) {
        if line.left.is_some() {
            self.layout.vertical_left = true;
        }

        if line.right.is_some() {
            self.layout.vertical_right = true;
        }

        if line.intersection.is_some() {
            self.layout.vertical_mid = true;
        }

        self.lines.insert(row, line);
        self.layout.horizontal.insert(row);
    }

    pub(crate) fn get_line(&self, row: usize) -> Option<&Line<T>> {
        self.lines.get(&row)
    }

    pub(crate) fn remove_line(&mut self, row: usize) {
        self.lines.remove(&row);
    }

    pub(crate) fn set_borders(&mut self, borders: Borders<T>) {
        self.borders = borders;
    }

    pub(crate) fn get_borders(&self) -> &Borders<T> {
        &self.borders
    }

    pub(crate) fn get_global(&self) -> Option<&T> {
        self.global.as_ref()
    }

    pub(crate) fn set_global(&mut self, value: T) {
        self.global = Some(value);
    }

    pub(crate) fn get_vertical(&self, pos: Position, count_cols: usize) -> Option<&T> {
        self.cells
            .vertical
            .get(&pos)
            .or({
                if pos.1 == count_cols {
                    self.borders.vertical_right.as_ref()
                } else if pos.1 == 0 {
                    self.borders.vertical_left.as_ref()
                } else {
                    self.borders.vertical.as_ref()
                }
            })
            .or(self.global.as_ref())
    }

    pub(crate) fn get_horizontal(&self, pos: Position, count_rows: usize) -> Option<&T> {
        self.cells
            .horizontal
            .get(&pos)
            .or_else(|| self.lines.get(&pos.0).and_then(|l| l.horizontal.as_ref()))
            .or({
                if pos.0 == 0 {
                    self.borders.top.as_ref()
                } else if pos.0 == count_rows {
                    self.borders.bottom.as_ref()
                } else {
                    self.borders.horizontal.as_ref()
                }
            })
            .or(self.global.as_ref())
    }

    pub(crate) fn get_intersection(
        &self,
        pos: Position,
        count_rows: usize,
        count_cols: usize,
    ) -> Option<&T> {
        let use_top = pos.0 == 0;
        let use_bottom = pos.0 == count_rows;
        let use_left = pos.1 == 0;
        let use_right = pos.1 == count_cols;

        self.cells
            .intersection
            .get(&pos)
            .or_else(|| {
                self.lines.get(&pos.0).and_then(|l| {
                    if use_left && l.left.is_some() {
                        l.left.as_ref()
                    } else if use_right && l.right.is_some() {
                        l.right.as_ref()
                    } else if !use_right && !use_left && l.intersection.is_some() {
                        l.intersection.as_ref()
                    } else {
                        None
                    }
                })
            })
            .or({
                if use_top && use_left {
                    self.borders.top_left.as_ref()
                } else if use_top && use_right {
                    self.borders.top_right.as_ref()
                } else if use_bottom && use_left {
                    self.borders.bottom_left.as_ref()
                } else if use_bottom && use_right {
                    self.borders.bottom_right.as_ref()
                } else if use_top {
                    self.borders.top_intersection.as_ref()
                } else if use_bottom {
                    self.borders.bottom_intersection.as_ref()
                } else if use_left {
                    self.borders.horizontal_left.as_ref()
                } else if use_right {
                    self.borders.horizontal_right.as_ref()
                } else {
                    self.borders.intersection.as_ref()
                }
            })
            .or(self.global.as_ref())
    }

    pub(crate) fn has_horizontal(&self, row: usize, count_rows: usize) -> bool {
        self.global.is_some()
            || (row == 0 && self.borders.has_top())
            || (row == count_rows && self.borders.has_bottom())
            || (row > 0 && row < count_rows && self.borders.has_horizontal())
            || self.is_horizontal_set(row)
    }

    pub(crate) fn has_vertical(&self, col: usize, count_cols: usize) -> bool {
        self.global.is_some()
            || (col == 0 && self.borders.has_left())
            || (col == count_cols && self.borders.has_right())
            || (col > 0 && col < count_cols && self.borders.has_vertical())
            || self.is_vertical_set(col, count_cols)
    }

    fn is_line_defined(&self, row: usize) -> bool {
        self.lines.get(&row).map_or(false, |l| {
            l.left.is_some()
                || l.right.is_some()
                || l.intersection.is_some()
                || l.horizontal.is_some()
        })
    }

    fn is_horizontal_set(&self, row: usize) -> bool {
        self.layout.horizontal.contains(&row)
    }

    fn is_vertical_set(&self, col: usize, count_cols: usize) -> bool {
        (col == 0 && self.layout.vertical_left)
            || (col == count_cols && self.layout.vertical_right)
            || (col > 0 && col < count_cols && self.layout.vertical_mid)
            || self.layout.vertical.contains(&col)
    }

    fn check_is_horizontal_set(&self, row: usize) -> bool {
        self.is_line_defined(row)
            || self.cells.horizontal.keys().any(|&p| p.0 == row)
            || self.cells.intersection.keys().any(|&p| p.0 == row)
    }

    fn check_is_vertical_set(&self, col: usize, count_cols: usize) -> bool {
        (col == 0 && self.layout.vertical_left)
            || (col == count_cols && self.layout.vertical_right)
            || (col > 0 && col < count_cols && self.layout.vertical_mid)
            || self.cells.vertical.keys().any(|&p| p.1 == col)
            || self.cells.intersection.keys().any(|&p| p.1 == col)
    }
}

/// Borders represents a Table frame with horizontal and vertical split lines.
#[derive(Debug, Clone, Default)]
pub struct Borders<T = char> {
    /// A top horizontal on the frame.
    pub top: Option<T>,
    /// A top left on the frame.
    pub top_left: Option<T>,
    /// A top right on the frame.
    pub top_right: Option<T>,
    /// A top horizontal intersection on the frame.
    pub top_intersection: Option<T>,

    /// A bottom horizontal on the frame.
    pub bottom: Option<T>,
    /// A bottom left on the frame.
    pub bottom_left: Option<T>,
    /// A bottom right on the frame.
    pub bottom_right: Option<T>,
    /// A bottom horizontal intersection on the frame.
    pub bottom_intersection: Option<T>,

    /// A horizontal split.
    pub horizontal: Option<T>,
    /// A horizontal split on the left frame line.
    pub horizontal_left: Option<T>,
    /// A horizontal split on the right frame line.
    pub horizontal_right: Option<T>,

    /// A vertical split.
    pub vertical: Option<T>,
    /// A vertical split on the left frame line.
    pub vertical_left: Option<T>,
    /// A vertical split on the right frame line.
    pub vertical_right: Option<T>,

    /// A top left charcter on the frame.
    pub intersection: Option<T>,
}

impl<T> Borders<T> {
    /// Verifies if borders has left line set on the frame.
    pub const fn has_left(&self) -> bool {
        self.vertical_left.is_some()
            || self.horizontal_left.is_some()
            || self.top_left.is_some()
            || self.bottom_left.is_some()
    }

    /// Verifies if borders has right line set on the frame.
    pub const fn has_right(&self) -> bool {
        self.vertical_right.is_some()
            || self.horizontal_right.is_some()
            || self.top_right.is_some()
            || self.bottom_right.is_some()
    }

    /// Verifies if borders has top line set on the frame.
    pub const fn has_top(&self) -> bool {
        self.top.is_some()
            || self.top_intersection.is_some()
            || self.top_left.is_some()
            || self.top_right.is_some()
    }

    /// Verifies if borders has bottom line set on the frame.
    pub const fn has_bottom(&self) -> bool {
        self.bottom.is_some()
            || self.bottom_intersection.is_some()
            || self.bottom_left.is_some()
            || self.bottom_right.is_some()
    }

    /// Verifies if borders has horizontal lines set.
    pub const fn has_horizontal(&self) -> bool {
        self.horizontal.is_some()
            || self.horizontal_left.is_some()
            || self.horizontal_right.is_some()
            || self.intersection.is_some()
    }

    /// Verifies if borders has vertical lines set.
    pub const fn has_vertical(&self) -> bool {
        self.intersection.is_some()
            || self.vertical.is_some()
            || self.top_intersection.is_some()
            || self.bottom_intersection.is_some()
    }
}

/// A structre for a custom horizontal/vertical line.
#[derive(Debug, Clone, Default)]
pub struct Line<T> {
    /// Line character.
    pub horizontal: Option<T>,
    /// Line intersection character.
    pub intersection: Option<T>,
    /// Left intersection character.
    pub left: Option<T>,
    /// Right intersection character.
    pub right: Option<T>,
}

impl<T> Line<T> {
    /// Verifies if the line has any setting set.
    pub const fn is_empty(&self) -> bool {
        self.horizontal.is_none()
            && self.horizontal.is_none()
            && self.left.is_none()
            && self.right.is_none()
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct BordersMap<T> {
    vertical: HashMap<Position, T>,
    horizontal: HashMap<Position, T>,
    intersection: HashMap<Position, T>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct BordersLayout {
    horizontal: HashSet<usize>,
    vertical: HashSet<usize>,
    vertical_mid: bool,
    vertical_left: bool,
    vertical_right: bool,
}
