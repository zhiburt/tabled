use super::Peaker;

/// A Peaker which goes over the smallest column first.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Hash)]
pub struct PriorityMin {
    side: bool,
}

impl PriorityMin {
    /// Creates a [`PriorityMin`] object with a side set to right or left,
    /// It's crusial in cases where both columns has equal widths and we need to peak a left or right.
    ///
    /// Passing true means a right side.
    /// Passing false means a left side.
    pub fn new(priorities_right: bool) -> Self {
        Self {
            side: priorities_right,
        }
    }

    /// Creates a [`PriorityMin`] object with left side prioritized,
    /// See [`PriorityMin::new`].
    pub fn left() -> Self {
        Self::new(false)
    }

    /// Creates a [`PriorityMin`] object with right side prioritized,
    /// See [`PriorityMin::new`].
    pub fn right() -> Self {
        Self::new(true)
    }
}

impl Peaker for PriorityMin {
    fn peak(&mut self, mins: &[usize], widths: &[usize]) -> Option<usize> {
        match self.side {
            true => (0..widths.len())
                .filter(|&i| mins.is_empty() || widths[i] > mins[i])
                .min_by_key(|&i| widths[i])
                .filter(|&col| widths[col] != 0),
            false => (0..widths.len())
                .rev()
                .filter(|&i| mins.is_empty() || widths[i] > mins[i])
                .min_by_key(|&i| widths[i])
                .filter(|&col| widths[col] != 0),
        }
    }
}
