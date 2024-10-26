/// A structure which represents 4 box sides.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

    /// Creates a new object.
    pub const fn filled(value: T) -> Self
    where
        T: Copy,
    {
        Self::new(value, value, value, value)
    }

    /// Creates a new object.
    pub fn convert_into<T1>(self) -> Sides<T1>
    where
        T: Into<T1>,
    {
        Sides::new(
            self.left.into(),
            self.right.into(),
            self.top.into(),
            self.bottom.into(),
        )
    }

    /// Converts all sides with a given function.
    pub fn map<F, T1>(self, f: F) -> Sides<T1>
    where
        F: Fn(T) -> T1,
    {
        Sides::new(
            (f)(self.left),
            (f)(self.right),
            (f)(self.top),
            (f)(self.bottom),
        )
    }
}
