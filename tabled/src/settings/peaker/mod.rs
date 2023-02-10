//! The module contains [`Peaker`] trait and its implementations to be used in [`Height`] and [`Width`].
//!
//! [`Width`]: crate::settings::width::Width
//! [`Height`]: crate::settings::height::Height

/// A strategy of width function.
/// It determines the order how the function is applied.
pub trait Peaker {
    /// Creates a new instance.
    fn create() -> Self;
    /// This function returns a column index which will be changed.
    /// Or `None` if no changes are necessary.
    fn peak(&mut self, min_widths: &[usize], widths: &[usize]) -> Option<usize>;
}

/// A Peaker which goes over column 1 by 1.
#[derive(Debug, Default, Clone)]
pub struct PriorityNone {
    i: usize,
}

impl Peaker for PriorityNone {
    fn create() -> Self {
        Self { i: 0 }
    }

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

/// A Peaker which goes over the biggest column first.
#[derive(Debug, Default, Clone)]
pub struct PriorityMax;

impl Peaker for PriorityMax {
    fn create() -> Self {
        Self
    }

    fn peak(&mut self, _: &[usize], widths: &[usize]) -> Option<usize> {
        let col = (0..widths.len()).max_by_key(|&i| widths[i]).unwrap();
        if widths[col] == 0 {
            None
        } else {
            Some(col)
        }
    }
}

/// A Peaker which goes over the smallest column first.
#[derive(Debug, Default, Clone)]
pub struct PriorityMin;

impl Peaker for PriorityMin {
    fn create() -> Self {
        Self
    }

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
