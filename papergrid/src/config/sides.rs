#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sides<T> {
    pub top: T,
    pub bottom: T,
    pub left: T,
    pub right: T,
}

impl<T> Sides<T> {
    pub fn new(left: T, right: T, top: T, bottom: T) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
        }
    }
}

/// Indent represent a filled space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Indent {
    /// A fill character.
    pub fill: char,
    /// A number of repeats of a fill character.
    pub size: usize,
}

impl Indent {
    /// Creates a new Indent structure.
    pub fn new(size: usize, fill: char) -> Self {
        Self { fill, size }
    }

    /// Creates a new Indent startucture with space (`' '`) as a fill character.
    pub fn spaced(size: usize) -> Self {
        Self { size, fill: ' ' }
    }
}

impl Default for Indent {
    fn default() -> Self {
        Self { size: 0, fill: ' ' }
    }
}
