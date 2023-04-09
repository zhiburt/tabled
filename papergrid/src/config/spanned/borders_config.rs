use std::collections::{HashMap, HashSet};

use crate::config::{Border, Borders, Position};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct BordersConfig<T> {
    global: Option<T>,
    borders: Borders<T>,
    cells: BordersMap<T>,
    horizontals: HashMap<usize, HorizontalLine<T>>,
    verticals: HashMap<usize, VerticalLine<T>>,
    layout: BordersLayout,
}

impl<T: std::fmt::Debug> BordersConfig<T> {
    pub(crate) fn insert_border(&mut self, pos: Position, border: Border<T>) {
        if let Some(c) = border.top {
            self.cells.horizontal.insert(pos, c);
            self.layout.horizontals.insert(pos.0);
        }

        if let Some(c) = border.bottom {
            self.cells.horizontal.insert((pos.0 + 1, pos.1), c);
            self.layout.horizontals.insert(pos.0 + 1);
        }

        if let Some(c) = border.left {
            self.cells.vertical.insert(pos, c);
            self.layout.verticals.insert(pos.1);
        }

        if let Some(c) = border.right {
            self.cells.vertical.insert((pos.0, pos.1 + 1), c);
            self.layout.verticals.insert(pos.1 + 1);
        }

        if let Some(c) = border.left_top_corner {
            self.cells.intersection.insert((pos.0, pos.1), c);
            self.layout.horizontals.insert(pos.0);
            self.layout.verticals.insert(pos.1);
        }

        if let Some(c) = border.right_top_corner {
            self.cells.intersection.insert((pos.0, pos.1 + 1), c);
            self.layout.horizontals.insert(pos.0);
            self.layout.verticals.insert(pos.1 + 1);
        }

        if let Some(c) = border.left_bottom_corner {
            self.cells.intersection.insert((pos.0 + 1, pos.1), c);
            self.layout.horizontals.insert(pos.0 + 1);
            self.layout.verticals.insert(pos.1);
        }

        if let Some(c) = border.right_bottom_corner {
            self.cells.intersection.insert((pos.0 + 1, pos.1 + 1), c);
            self.layout.horizontals.insert(pos.0 + 1);
            self.layout.verticals.insert(pos.1 + 1);
        }
    }

    pub(crate) fn remove_border(&mut self, pos: Position, shape: (usize, usize)) {
        let (count_rows, count_cols) = shape;

        self.cells.horizontal.remove(&pos);
        self.cells.horizontal.remove(&(pos.0 + 1, pos.1));
        self.cells.vertical.remove(&pos);
        self.cells.vertical.remove(&(pos.0, pos.1 + 1));
        self.cells.intersection.remove(&pos);
        self.cells.intersection.remove(&(pos.0 + 1, pos.1));
        self.cells.intersection.remove(&(pos.0, pos.1 + 1));
        self.cells.intersection.remove(&(pos.0 + 1, pos.1 + 1));

        // clean up the layout.

        if !self.check_is_horizontal_set(pos.0, count_rows) {
            self.layout.horizontals.remove(&pos.0);
        }

        if !self.check_is_horizontal_set(pos.0 + 1, count_rows) {
            self.layout.horizontals.remove(&(pos.0 + 1));
        }

        if !self.check_is_vertical_set(pos.1, count_cols) {
            self.layout.verticals.remove(&pos.1);
        }

        if !self.check_is_vertical_set(pos.1 + 1, count_cols) {
            self.layout.verticals.remove(&(pos.1 + 1));
        }
    }

    pub(crate) fn get_border(&self, pos: Position, shape: (usize, usize)) -> Border<&T> {
        Border {
            top: self.get_horizontal(pos, shape.0),
            bottom: self.get_horizontal((pos.0 + 1, pos.1), shape.0),
            left: self.get_vertical(pos, shape.1),
            left_top_corner: self.get_intersection(pos, shape),
            left_bottom_corner: self.get_intersection((pos.0 + 1, pos.1), shape),
            right: self.get_vertical((pos.0, pos.1 + 1), shape.1),
            right_top_corner: self.get_intersection((pos.0, pos.1 + 1), shape),
            right_bottom_corner: self.get_intersection((pos.0 + 1, pos.1 + 1), shape),
        }
    }

    pub(crate) fn insert_horizontal_line(&mut self, row: usize, line: HorizontalLine<T>) {
        if line.left.is_some() {
            self.layout.left = true;
        }

        // todo: when we delete lines these are still left set; so has_horizontal/vertical return true in some cases;
        // it shall be fixed, but maybe we can improve the logic as it got a bit complicated.
        if line.right.is_some() {
            self.layout.right = true;
        }

        if line.intersection.is_some() {
            self.layout.inner_verticals = true;
        }

        self.horizontals.insert(row, line);
        self.layout.horizontals.insert(row);
    }

    pub(crate) fn get_horizontal_line(&self, row: usize) -> Option<&HorizontalLine<T>> {
        self.horizontals.get(&row)
    }

    pub(crate) fn remove_horizontal_line(&mut self, row: usize, count_rows: usize) {
        self.horizontals.remove(&row);
        self.layout.horizontals.remove(&row);

        if self.has_horizontal(row, count_rows) {
            self.layout.horizontals.insert(row);
        }
    }

    pub(crate) fn insert_vertical_line(&mut self, row: usize, line: VerticalLine<T>) {
        if line.top.is_some() {
            self.layout.top = true;
        }

        if line.bottom.is_some() {
            self.layout.bottom = true;
        }

        self.verticals.insert(row, line);
        self.layout.verticals.insert(row);
    }

    pub(crate) fn get_vertical_line(&self, row: usize) -> Option<&VerticalLine<T>> {
        self.verticals.get(&row)
    }

    pub(crate) fn remove_vertical_line(&mut self, col: usize, count_columns: usize) {
        self.verticals.remove(&col);
        self.layout.verticals.remove(&col);

        if self.has_vertical(col, count_columns) {
            self.layout.verticals.insert(col);
        }
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
            .or_else(|| self.verticals.get(&pos.1).and_then(|l| l.main.as_ref()))
            .or({
                if pos.1 == count_cols {
                    self.borders.right.as_ref()
                } else if pos.1 == 0 {
                    self.borders.left.as_ref()
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
            .or_else(|| self.horizontals.get(&pos.0).and_then(|l| l.main.as_ref()))
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
        (count_rows, count_cols): (usize, usize),
    ) -> Option<&T> {
        let use_top = pos.0 == 0;
        let use_bottom = pos.0 == count_rows;
        let use_left = pos.1 == 0;
        let use_right = pos.1 == count_cols;

        if let Some(c) = self.cells.intersection.get(&pos) {
            return Some(c);
        }

        let hl_c = self.horizontals.get(&pos.0).and_then(|l| {
            if use_left && l.left.is_some() {
                l.left.as_ref()
            } else if use_right && l.right.is_some() {
                l.right.as_ref()
            } else if !use_right && !use_left && l.intersection.is_some() {
                l.intersection.as_ref()
            } else {
                None
            }
        });

        if let Some(c) = hl_c {
            return Some(c);
        }

        let vl_c = self.verticals.get(&pos.1).and_then(|l| {
            if use_top && l.top.is_some() {
                l.top.as_ref()
            } else if use_bottom && l.bottom.is_some() {
                l.bottom.as_ref()
            } else if !use_top && !use_bottom && l.intersection.is_some() {
                l.intersection.as_ref()
            } else {
                None
            }
        });

        if let Some(c) = vl_c {
            return Some(c);
        }

        let borders_c = {
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
                self.borders.left_intersection.as_ref()
            } else if use_right {
                self.borders.right_intersection.as_ref()
            } else {
                self.borders.intersection.as_ref()
            }
        };

        if let Some(c) = borders_c {
            return Some(c);
        }

        self.global.as_ref()
    }

    pub(crate) fn has_horizontal(&self, row: usize, count_rows: usize) -> bool {
        self.global.is_some()
            || (row == 0 && self.borders.has_top())
            || (row == count_rows && self.borders.has_bottom())
            || (row > 0 && row < count_rows && self.borders.has_horizontal())
            || self.is_horizontal_set(row, count_rows)
    }

    pub(crate) fn has_vertical(&self, col: usize, count_cols: usize) -> bool {
        self.global.is_some()
            || (col == 0 && self.borders.has_left())
            || (col == count_cols && self.borders.has_right())
            || (col > 0 && col < count_cols && self.borders.has_vertical())
            || self.is_vertical_set(col, count_cols)
    }

    fn is_horizontal_set(&self, row: usize, count_rows: usize) -> bool {
        (row == 0 && self.layout.top)
            || (row == count_rows && self.layout.bottom)
            || (row > 0 && row < count_rows && self.layout.inner_horizontals)
            || self.layout.horizontals.contains(&row)
    }

    fn is_vertical_set(&self, col: usize, count_cols: usize) -> bool {
        (col == 0 && self.layout.left)
            || (col == count_cols && self.layout.right)
            || (col > 0 && col < count_cols && self.layout.inner_verticals)
            || self.layout.verticals.contains(&col)
    }

    fn check_is_horizontal_set(&self, row: usize, count_rows: usize) -> bool {
        (row == 0 && self.layout.top)
            || (row == count_rows && self.layout.bottom)
            || (row > 0 && row < count_rows && self.layout.inner_horizontals)
            || self.cells.horizontal.keys().any(|&p| p.0 == row)
            || self.cells.intersection.keys().any(|&p| p.0 == row)
    }

    fn check_is_vertical_set(&self, col: usize, count_cols: usize) -> bool {
        (col == 0 && self.layout.left)
            || (col == count_cols && self.layout.right)
            || (col > 0 && col < count_cols && self.layout.inner_verticals)
            || self.cells.vertical.keys().any(|&p| p.1 == col)
            || self.cells.intersection.keys().any(|&p| p.1 == col)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct BordersMap<T> {
    vertical: HashMap<Position, T>,
    horizontal: HashMap<Position, T>,
    intersection: HashMap<Position, T>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct BordersLayout {
    left: bool,
    right: bool,
    top: bool,
    bottom: bool,
    inner_verticals: bool,
    inner_horizontals: bool,
    horizontals: HashSet<usize>,
    verticals: HashSet<usize>,
}

/// A structure for a custom horizontal line.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct HorizontalLine<T> {
    /// Line character.
    pub main: Option<T>,
    /// Line intersection character.
    pub intersection: Option<T>,
    /// Left intersection character.
    pub left: Option<T>,
    /// Right intersection character.
    pub right: Option<T>,
}

impl<T> HorizontalLine<T> {
    /// Verifies if the line has any setting set.
    pub const fn is_empty(&self) -> bool {
        self.main.is_none()
            && self.intersection.is_none()
            && self.left.is_none()
            && self.right.is_none()
    }
}

/// A structure for a vertical line.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct VerticalLine<T> {
    /// Line character.
    pub main: Option<T>,
    /// Line intersection character.
    pub intersection: Option<T>,
    /// Left intersection character.
    pub top: Option<T>,
    /// Right intersection character.
    pub bottom: Option<T>,
}

impl<T> VerticalLine<T> {
    /// Verifies if the line has any setting set.
    pub const fn is_empty(&self) -> bool {
        self.main.is_none()
            && self.intersection.is_none()
            && self.top.is_none()
            && self.bottom.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_border() {
        let mut borders = BordersConfig::<char>::default();
        borders.insert_border((0, 0), Border::filled('x'));

        assert_eq!(borders.get_border((0, 0), (10, 10)), Border::filled(&'x'));
        assert_eq!(borders.get_border((0, 0), (0, 0)), Border::filled(&'x'));

        assert!(borders.is_horizontal_set(0, 10));
        assert!(borders.is_horizontal_set(1, 10));
        assert!(!borders.is_horizontal_set(2, 10));
        assert!(borders.is_vertical_set(0, 10));
        assert!(borders.is_vertical_set(1, 10));
        assert!(!borders.is_vertical_set(2, 10));

        assert!(borders.is_horizontal_set(0, 0));
        assert!(borders.is_horizontal_set(1, 0));
        assert!(!borders.is_horizontal_set(2, 0));
        assert!(borders.is_vertical_set(0, 0));
        assert!(borders.is_vertical_set(1, 0));
        assert!(!borders.is_vertical_set(2, 0));
    }

    #[test]
    fn test_insert_border_override() {
        let mut borders = BordersConfig::<char>::default();
        borders.insert_border((0, 0), Border::filled('x'));
        borders.insert_border((1, 0), Border::filled('y'));
        borders.insert_border((0, 1), Border::filled('w'));
        borders.insert_border((1, 1), Border::filled('q'));

        assert_eq!(
            borders.get_border((0, 0), (10, 10)).copied(),
            Border::full('x', 'y', 'x', 'w', 'x', 'w', 'y', 'q')
        );
        assert_eq!(
            borders.get_border((0, 1), (10, 10)).copied(),
            Border::full('w', 'q', 'w', 'w', 'w', 'w', 'q', 'q')
        );
        assert_eq!(
            borders.get_border((1, 0), (10, 10)).copied(),
            Border::full('y', 'y', 'y', 'q', 'y', 'q', 'y', 'q')
        );
        assert_eq!(
            borders.get_border((1, 1), (10, 10)).copied(),
            Border::filled('q')
        );

        assert!(borders.is_horizontal_set(0, 10));
        assert!(borders.is_horizontal_set(1, 10));
        assert!(borders.is_horizontal_set(2, 10));
        assert!(!borders.is_horizontal_set(3, 10));
        assert!(borders.is_vertical_set(0, 10));
        assert!(borders.is_vertical_set(1, 10));
        assert!(borders.is_vertical_set(2, 10));
        assert!(!borders.is_vertical_set(3, 10));
    }

    #[test]
    fn test_set_global() {
        let mut borders = BordersConfig::<char>::default();
        borders.insert_border((0, 0), Border::filled('x'));
        borders.set_global('l');

        assert_eq!(borders.get_border((0, 0), (10, 10)), Border::filled(&'x'));
        assert_eq!(borders.get_border((2, 0), (10, 10)), Border::filled(&'l'));

        assert!(borders.is_horizontal_set(0, 10));
        assert!(borders.is_horizontal_set(1, 10));
        assert!(!borders.is_horizontal_set(2, 10));
        assert!(borders.is_vertical_set(0, 10));
        assert!(borders.is_vertical_set(1, 10));
        assert!(!borders.is_vertical_set(2, 10));

        assert!(borders.is_horizontal_set(0, 0));
        assert!(borders.is_horizontal_set(1, 0));
        assert!(!borders.is_horizontal_set(2, 0));
        assert!(borders.is_vertical_set(0, 0));
        assert!(borders.is_vertical_set(1, 0));
        assert!(!borders.is_vertical_set(2, 0));
    }
}
