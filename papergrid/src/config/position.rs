use core::ops::{Add, AddAssign, Sub, SubAssign};

/// Constructs a [`Position`] in a convinient way.
/// Basically a short hand for `(row, col).into()` or `Position::new(row, col)`.
pub fn pos(row: usize, col: usize) -> Position {
    Position::new(row, col)
}

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
    row: usize,
    col: usize,
}

impl Position {
    /// Creates a new [`Position`] object.
    pub const fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    /// Returns a row value.
    pub const fn row(&self) -> usize {
        self.row
    }

    /// Returns a column value.
    pub const fn col(&self) -> usize {
        self.col
    }

    /// A check whether a cell is not beyond the maximum point.
    /// Meaning it's less then a maximum point.
    /// So it must be located left and bottom on XY axis.
    pub const fn is_covered(&self, max: Position) -> bool {
        self.row < max.row && self.col < max.col
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
        (val.row(), val.col())
    }
}
