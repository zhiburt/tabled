/// A structure which represents 4 box sides.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sides<T> {
    /// Top side.
    pub top: T,
    /// Bottom side.
    pub bottom: T,
    /// Left side.
    pub left: T,
    /// Right side.
    pub right: T,
}

impl<T> Sides<T> {
    /// Creates a new object.
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
    pub const fn new(size: usize, fill: char) -> Self {
        Self { fill, size }
    }

    /// Creates a new Indent startucture with space (`' '`) as a fill character.
    pub const fn spaced(size: usize) -> Self {
        Self { size, fill: ' ' }
    }

    /// Creates a new Indent startucture with space (`' '`) as a fill character.
    pub const fn zero() -> Self {
        Self::new(0, ' ')
    }
}

impl Default for Indent {
    fn default() -> Self {
        Self { size: 0, fill: ' ' }
    }
}
