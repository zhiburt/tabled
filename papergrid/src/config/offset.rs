/// The structure represents an offset in a text.
///
/// # Example
///
/// ```
/// # use papergrid::config::Offset;
/// assert_eq!(Offset::from(1), Offset::Start(1));
/// assert_eq!(Offset::from(-1), Offset::End(1));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Offset {
    /// An offset from the start.
    Start(usize),
    /// An offset from the end.
    End(usize),
}

/// If negative value is given consider it an offset from end.
impl From<isize> for Offset {
    fn from(value: isize) -> Self {
        if value > 0 {
            Offset::Start(value as usize)
        } else {
            Offset::End((-value) as usize)
        }
    }
}

/// If negative value is given consider it an offset from end.
impl From<i32> for Offset {
    fn from(value: i32) -> Self {
        if value > 0 {
            Offset::Start(value as usize)
        } else {
            Offset::End((-value) as usize)
        }
    }
}

/// If negative value is given consider it an offset from end.
impl From<i64> for Offset {
    fn from(value: i64) -> Self {
        if value > 0 {
            Offset::Start(value as usize)
        } else {
            Offset::End((-value) as usize)
        }
    }
}

impl From<usize> for Offset {
    fn from(value: usize) -> Self {
        Offset::Start(value)
    }
}
