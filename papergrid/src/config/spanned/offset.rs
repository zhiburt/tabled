/// The structure represents an offset in a text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Offset {
    /// An offset from the start.
    Begin(usize),
    /// An offset from the end.
    End(usize),
}
