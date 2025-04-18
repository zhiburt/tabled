use core::ops::{Add, AddAssign, Sub, SubAssign};

/// Position is a (row, col) position on a Grid.
///
/// For example such table has 4 cells.
/// Which indexes are (0, 0), (0, 1), (1, 0), (1, 1).
///
/// ```text
/// ┌───┬───┐
/// │ 0 │ 1 │
/// ├───┼───┤
/// │ 1 │ 2 │
/// └───┴───┘
/// ```
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    /// Row.
    pub row: usize,
    /// Column.
    pub col: usize,
}

impl Position {
    /// Creates a new [`Position`] object.
    pub const fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    /// A check whether a given cell has intersection with any other cell.
    ///
    /// # Example
    ///
    /// ```
    /// # use papergrid::config::Position;
    /// let p = Position::new(3, 3);
    ///
    /// assert!(p.has_intersection(p));
    /// assert!(p.has_intersection(Position::new(1, 1)));
    /// assert!(p.has_intersection(Position::new(3, 10)));
    /// assert!(p.has_intersection(Position::new(10, 3)));
    ///
    /// assert!(!p.has_intersection(Position::new(4, 4)));
    /// ```
    pub const fn has_intersection(&self, point: Position) -> bool {
        self.row >= point.row || self.col >= point.col
    }

    /// A check whether a given cell has intersection with any other cell.
    ///
    /// # Example
    ///
    /// ```
    /// # use papergrid::config::Position;
    /// let p = Position::new(3, 3);
    ///
    /// assert!(p.has_coverage(Position::new(1, 1)));
    ///
    /// assert!(!p.has_coverage(Position::new(3, 3)));
    /// assert!(!p.has_coverage(Position::new(1, 10)));
    /// assert!(!p.has_coverage(p));
    /// ```
    pub const fn has_coverage(&self, point: Position) -> bool {
        self.row > point.row && self.col > point.col
    }
}

impl Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.row + rhs.row, self.col + rhs.col)
    }
}

impl Add<(usize, usize)> for Position {
    type Output = Position;

    fn add(self, rhs: (usize, usize)) -> Self::Output {
        Self::new(self.row + rhs.0, self.col + rhs.1)
    }
}

impl Sub<Position> for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.row - rhs.row, self.col - rhs.col)
    }
}

impl Sub<(usize, usize)> for Position {
    type Output = Position;

    fn sub(self, rhs: (usize, usize)) -> Self::Output {
        Self::new(self.row - rhs.0, self.col - rhs.1)
    }
}

impl AddAssign<Position> for Position {
    fn add_assign(&mut self, rhs: Position) {
        self.row += rhs.row;
        self.col += rhs.col;
    }
}

impl AddAssign<(usize, usize)> for Position {
    fn add_assign(&mut self, rhs: (usize, usize)) {
        self.row += rhs.0;
        self.col += rhs.1;
    }
}

impl SubAssign<Position> for Position {
    fn sub_assign(&mut self, rhs: Position) {
        self.row -= rhs.row;
        self.col -= rhs.col;
    }
}

impl SubAssign<(usize, usize)> for Position {
    fn sub_assign(&mut self, rhs: (usize, usize)) {
        self.row -= rhs.0;
        self.col -= rhs.1;
    }
}

impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl From<Position> for (usize, usize) {
    fn from(val: Position) -> Self {
        (val.row, val.col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordering() {
        assert!(Position::new(1, 1) < Position::new(2, 2));
        assert!(Position::new(2, 1) < Position::new(2, 2));
        assert!(Position::new(1, 2) < Position::new(2, 2));

        assert!(Position::new(2, 2) == Position::new(2, 2));

        assert!(Position::new(3, 3) > Position::new(2, 2));
        assert!(Position::new(3, 1) > Position::new(2, 2));

        assert!(Position::new(1, 3) < Position::new(2, 2));

        assert!(Position::new(0, 10000) < Position::new(4, 4));
    }
}
