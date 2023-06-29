/// The structure represents a table mode for a given entity,
/// either it will be rendered vertically or horizontally.
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub enum Orientation {
    /// Vertical mode (from top to bottom).
    Row,
    /// Horizontal mode (from left to right).
    Column,
}
