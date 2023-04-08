/// A line data structure.
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Line<T> {
    /// A horizontal/vertical character.
    pub main: T,
    /// A horizontal/vertical intersection.
    pub intersection: Option<T>,
    /// A horizontal left / vertical top intersection.
    pub connect1: Option<T>,
    /// A horizontal right / vertical bottom intersection.
    pub connect2: Option<T>,
}

impl<T> Line<T> {
    /// Creates a new line.
    pub const fn new(
        main: T,
        intersection: Option<T>,
        connect1: Option<T>,
        connect2: Option<T>,
    ) -> Self {
        Self {
            main,
            intersection,
            connect1,
            connect2,
        }
    }

    /// Creates a new line.
    pub const fn filled(val: T) -> Self
    where
        T: Copy,
    {
        Self {
            main: val,
            intersection: Some(val),
            connect1: Some(val),
            connect2: Some(val),
        }
    }
}
