/// Indent represent a filled space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Indent {
    /// A fill character.
    pub fill: char,
    /// A number of repeats of a fill character.
    pub size: usize,
}

impl Indent {
    /// Creates a new Indent structure.
    pub const fn new(size: usize, fill: char) -> Self {
        Self { fill, size }
    }

    /// Creates a new Indent structure with space (`' '`) as a fill character.
    pub const fn spaced(size: usize) -> Self {
        Self { size, fill: ' ' }
    }

    /// Creates a new Indent structure with space (`' '`) as a fill character.
    pub const fn zero() -> Self {
        Self::new(0, ' ')
    }
}

impl Default for Indent {
    fn default() -> Self {
        Self { size: 0, fill: ' ' }
    }
}
