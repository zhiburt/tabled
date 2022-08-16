/// [`AlignmentHorizontal`] represents an horizontal alignment of a cell content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignmentHorizontal {
    /// Align to the center.
    Center,
    /// Align on the left.
    Left,
    /// Align on the right.
    Right,
}

/// [`AlignmentVertical`] represents an vertical alignment of a cell content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignmentVertical {
    /// Align to the center.
    Center,
    /// Align to the top.
    Top,
    /// Align to the bottom.
    Bottom,
}
