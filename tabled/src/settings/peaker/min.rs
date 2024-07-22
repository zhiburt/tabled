use super::Peaker;

/// A Peaker which goes over the smallest column first.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Hash)]
pub struct PriorityMin;

impl Peaker for PriorityMin {
    fn peak(&mut self, min_widths: &[usize], widths: &[usize]) -> Option<usize> {
        let col = (0..widths.len())
            .filter(|&i| min_widths.is_empty() || widths[i] > min_widths[i])
            .min_by_key(|&i| widths[i])
            .unwrap();
        if widths[col] == 0 {
            None
        } else {
            Some(col)
        }
    }
}
