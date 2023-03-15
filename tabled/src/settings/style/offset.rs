use crate::grid::config::spanned;

/// The structure represents an offset in a text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Offset {
    /// An offset from the start.
    Begin(usize),
    /// An offset from the end.
    End(usize),
}

impl From<Offset> for spanned::Offset {
    fn from(o: Offset) -> Self {
        match o {
            Offset::Begin(i) => spanned::Offset::Begin(i),
            Offset::End(i) => spanned::Offset::End(i),
        }
    }
}
