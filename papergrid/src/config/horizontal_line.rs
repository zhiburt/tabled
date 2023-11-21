/// A structure for a custom horizontal line.
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HorizontalLine<T> {
    /// Line character.
    pub main: Option<T>,
    /// Line intersection character.
    pub intersection: Option<T>,
    /// Left intersection character.
    pub left: Option<T>,
    /// Right intersection character.
    pub right: Option<T>,
}

impl<T> HorizontalLine<T> {
    /// Creates a new line.
    pub const fn new(
        main: Option<T>,
        intersection: Option<T>,
        left: Option<T>,
        right: Option<T>,
    ) -> Self {
        Self {
            main,
            intersection,
            left,
            right,
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
            left: Some(val),
            right: Some(val),
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
            && self.left.is_none()
            && self.right.is_none()
    }
}
