/// Formatting represent a logic of formatting of a cell.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Formatting {
    /// An setting to allow horizontal trim.
    pub horizontal_trim: bool,
    /// An setting to allow vertical trim.
    pub vertical_trim: bool,
    /// An setting to allow alignment per line.
    pub allow_lines_alignment: bool,
}

impl Formatting {
    /// Creates a new [`Formatting`] structure.
    pub fn new(horizontal_trim: bool, vertical_trim: bool, allow_lines_alignment: bool) -> Self {
        Self {
            horizontal_trim,
            vertical_trim,
            allow_lines_alignment,
        }
    }
}
