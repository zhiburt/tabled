#[cfg(feature = "std")]
use crate::{Table, Tabled};

/// [`LayoutIterator`] is a convient abstraction to iterate over rows/columns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct LayoutIterator {
    from: usize,
    to: usize,
    batch: usize,
    i: usize,
}

impl LayoutIterator {
    /// Creates a custom [`LayoutIterator`] instance.
    pub fn new(from: usize, to: usize, batch: usize) -> Self {
        Self {
            from,
            to,
            batch,
            i: 0,
        }
    }

    /// Creates a record iterator for KV table created by [`Table::kv`].
    /// So it basically skips all rows until next record starts.
    #[cfg(feature = "std")]
    pub fn kv_batches<T>(t: &Table) -> Self
    where
        T: Tabled,
    {
        Self::new(0, t.count_rows(), T::LENGTH)
    }
}

impl Iterator for LayoutIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.batch == 0 {
            return None;
        }

        if self.from >= self.to {
            return None;
        }

        let value = self.i * self.batch;
        self.from += self.batch;
        self.i += 1;

        Some(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::iter::LayoutIterator;

    #[test]
    fn test_layout_iterator() {
        assert_eq!(
            LayoutIterator::new(0, 5, 1).collect::<Vec<_>>(),
            vec![0, 1, 2, 3, 4]
        );
        assert_eq!(
            LayoutIterator::new(0, 5, 2).collect::<Vec<_>>(),
            vec![0, 2, 4]
        );
        assert_eq!(
            LayoutIterator::new(0, 6, 2).collect::<Vec<_>>(),
            vec![0, 2, 4]
        );
        assert_eq!(LayoutIterator::new(0, 0, 2).collect::<Vec<_>>(), vec![]);
        assert_eq!(LayoutIterator::new(0, 5, 0).collect::<Vec<_>>(), vec![]);
        assert_eq!(LayoutIterator::new(0, 0, 0).collect::<Vec<_>>(), vec![]);
    }
}
