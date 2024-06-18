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
pub type Position = (usize, usize);
