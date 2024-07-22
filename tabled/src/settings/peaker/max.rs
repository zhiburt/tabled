use super::Peaker;

/// A Peaker which goes over the biggest column first.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Hash)]
pub struct PriorityMax;

impl Peaker for PriorityMax {
    fn peak(&mut self, _: &[usize], widths: &[usize]) -> Option<usize> {
        let col = (0..widths.len()).max_by_key(|&i| widths[i]).unwrap();
        if widths[col] == 0 {
            None
        } else {
            Some(col)
        }
    }
}
