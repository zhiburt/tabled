//! An empty [`Records`] implementation.

use core::iter::{repeat, Repeat, Take};

use super::Records;

/// Empty representation of [`Records`].
#[derive(Debug, Default, Clone)]
pub struct EmptyRecords {
    rows: usize,
    cols: usize,
}

impl EmptyRecords {
    /// Constructs an empty representation of [`Records`] with a given shape.
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { rows, cols }
    }
}

impl From<(usize, usize)> for EmptyRecords {
    fn from((count_rows, count_columns): (usize, usize)) -> Self {
        Self::new(count_rows, count_columns)
    }
}

impl Records for EmptyRecords {
    type Iter = Take<Repeat<Take<Repeat<&'static str>>>>;

    fn iter_rows(self) -> Self::Iter {
        repeat(repeat("").take(self.cols)).take(self.rows)
    }

    fn count_columns(&self) -> usize {
        self.cols
    }

    fn hint_count_rows(&self) -> Option<usize> {
        Some(self.rows)
    }
}
