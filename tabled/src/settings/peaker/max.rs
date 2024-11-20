use super::Peaker;

/// A Peaker which goes over the biggest column first.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Hash)]
pub struct PriorityMax {
    side: bool,
}

impl PriorityMax {
    /// Creates a [`PriorityMax`] object with a side set to right or left,
    /// It's crusial in cases where both columns has equal widths and we need to peak a left or right.
    ///
    /// Passing true means a right side.
    /// Passing false means a left side.
    pub fn new(priorities_right: bool) -> Self {
        Self {
            side: priorities_right,
        }
    }

    /// Creates a [`PriorityMax`] object with left side prioritized,
    /// See [`PriorityMax::new`].
    pub fn left() -> Self {
        Self::new(false)
    }

    /// Creates a [`PriorityMax`] object with right side prioritized,
    /// See [`PriorityMax::new`].
    pub fn right() -> Self {
        Self::new(true)
    }
}

impl Peaker for PriorityMax {
    fn peak(&mut self, mins: &[usize], widths: &[usize]) -> Option<usize> {
        if self.side {
            (0..widths.len())
                .filter(|&i| mins.is_empty() || widths[i] > mins[i])
                .max_by_key(|&i| widths[i])
                .filter(|&col| widths[col] != 0)
        } else {
            (0..widths.len())
                .rev()
                .filter(|&i| mins.is_empty() || widths[i] > mins[i])
                .max_by_key(|&i| widths[i])
                .filter(|&col| widths[col] != 0)
        }
    }
}
