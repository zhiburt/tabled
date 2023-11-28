/// A structure for a vertical line.
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VerticalLine<T> {
    /// Line character.
    pub main: Option<T>,
    /// Line intersection character.
    pub intersection: Option<T>,
    /// Left intersection character.
    pub top: Option<T>,
    /// Right intersection character.
    pub bottom: Option<T>,
}

impl<T> VerticalLine<T> {
    /// Creates a new line.
    pub const fn new(
        main: Option<T>,
        intersection: Option<T>,
        top: Option<T>,
        bottom: Option<T>,
    ) -> Self {
        Self {
            main,
            intersection,
            top,
            bottom,
        }
    }

    /// Creates a new line.
    pub const fn full(main: T, intersection: T, left: T, right: T) -> Self {
        Self::new(Some(main), Some(intersection), Some(left), Some(right))
    }

    /// Creates a new line.
    pub const fn filled(val: T) -> Self
    where
        T: Copy,
    {
        Self {
            main: Some(val),
            intersection: Some(val),
            top: Some(val),
            bottom: Some(val),
        }
    }

    /// Creates a new line.
    pub const fn empty() -> Self {
        Self::new(None, None, None, None)
    }

    /// Verifies if the line has any setting set.
    pub const fn is_empty(&self) -> bool {
        self.main.is_none()
            && self.intersection.is_none()
            && self.top.is_none()
            && self.bottom.is_none()
    }
}
