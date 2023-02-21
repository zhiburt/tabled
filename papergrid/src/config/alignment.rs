/// [`AlignmentHorizontal`] represents an horizontal alignment of a cell content.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlignmentHorizontal {
    /// Align to the center.
    Center,
    /// Align on the left.
    Left,
    /// Align on the right.
    Right,
}

/// [`AlignmentVertical`] represents an vertical alignment of a cell content.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlignmentVertical {
    /// Align to the center.
    Center,
    /// Align to the top.
    Top,
    /// Align to the bottom.
    Bottom,
}
