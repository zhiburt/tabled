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
    pub const fn new(left: T, right: T, top: T, bottom: T) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
        }
    }
}
