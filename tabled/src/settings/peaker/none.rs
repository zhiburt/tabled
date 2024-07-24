use super::Peaker;

/// A Peaker which goes over column 1 by 1.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Hash)]
pub struct PriorityNone {
    i: usize,
}

impl PriorityNone {
    /// Creates a new priority which does not target anything.
    pub const fn new() -> Self {
        Self { i: 0 }
    }
}

impl Peaker for PriorityNone {
    fn peak(&mut self, _: &[usize], widths: &[usize]) -> Option<usize> {
        let mut i = self.i;
        let mut count_empty = 0;
        while widths[i] == 0 {
            i += 1;
            if i >= widths.len() {
                i = 0;
            }

            count_empty += 1;
            if count_empty == widths.len() {
                return None;
            }
        }

        let col = i;

        i += 1;
        if i >= widths.len() {
            i = 0;
        }

        self.i = i;

        Some(col)
    }
}
