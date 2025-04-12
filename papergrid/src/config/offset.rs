/// The structure represents an offset in a text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Offset {
    /// An offset from the start.
    Start(usize),
    /// An offset from the end.
    End(usize),
}

// todo: Add an example of usage
impl From<isize> for Offset {
    fn from(value: isize) -> Self {
        if value > 0 {
            Offset::Start(value as usize)
        } else {
            Offset::End((-value) as usize)
        }
    }
}

impl From<i32> for Offset {
    fn from(value: i32) -> Self {
        if value > 0 {
            Offset::Start(value as usize)
        } else {
            Offset::End((-value) as usize)
        }
    }
}

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
