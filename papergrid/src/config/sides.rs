/// A structure which represents 4 box sides.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    /// Converts all sides with a given function.
    pub fn fold<B, F>(self, acc: B, f: F) -> B
    where
        F: FnMut(B, T) -> B,
    {
        let mut f = f;
        let mut acc = acc;

        acc = (f)(acc, self.left);
        acc = (f)(acc, self.right);
        acc = (f)(acc, self.top);
        acc = (f)(acc, self.bottom);

        acc
    }
}

impl<T> Sides<Option<T>> {
    /// Checkes whether any option was set
    pub const fn is_empty(&self) -> bool {
        self.left.is_none() && self.right.is_none() && self.top.is_none() && self.bottom.is_none()
    }
}
