use super::Peaker;

/// A Peaker which goes over column 1 by 1, but truncates as much as possible left side.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Hash)]
pub struct PriorityLeft {
    i: usize,
}

impl PriorityLeft {
    /// Creates a new priority which does not target anything.
    pub const fn new() -> Self {
        Self { i: 0 }
    }
}

impl Peaker for PriorityLeft {
    fn peak(&mut self, min: &[usize], widths: &[usize]) -> Option<usize> {
        let col = self.i;
        if widths[col] > min[col] {
            return Some(col);
        }

        if col + 1 == widths.len() {
            return None;
        }

        let mut col = col + 1;
        while widths[col] == min[col] {
            if col + 1 == widths.len() {
                return None;
            }

            col += 1;
        }

        Some(col)
    }
}
