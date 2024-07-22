use super::Peaker;

/// A Peaker which goes over column 1 by 1, from right side, but truncates as much as possible right side.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Hash)]
pub struct PriorityRight {
    i: Option<usize>,
}

impl PriorityRight {
    /// Creates a new priority which does not target anything.
    pub const fn new() -> Self {
        Self { i: None }
    }
}

impl Peaker for PriorityRight {
    fn peak(&mut self, min: &[usize], widths: &[usize]) -> Option<usize> {
        if widths.is_empty() {
            return None;
        }

        if self.i.is_none() {
            self.i = Some(widths.len() - 1);
        }

        let col = self.i.expect("checked");
        if widths[col] > min[col] {
            return Some(col);
        }

        if col == 0 {
            return None;
        }

        let mut col = col - 1;
        while widths[col] == min[col] {
            if col == 0 {
                return None;
            }

            col -= 1;
        }

        Some(col)
    }
}
