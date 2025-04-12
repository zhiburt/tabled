use crate::{
    grid::config::Offset,
    grid::records::{ExactRecords, Records, Resizable},
    settings::TableOption,
};

// TOOD: simplify

/// Reverse data on the table.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Reverse {
    columns: bool,
    start: usize,
    limit: Offset,
}

impl Reverse {
    /// Reverse columns starting from given index.
    pub const fn columns(start: usize) -> Self {
        Self::new(true, start, Offset::End(0))
    }

    /// Reverse rows starting from given index.
    pub const fn rows(start: usize) -> Self {
        Self::new(false, start, Offset::End(0))
    }

    /// Reverse rows starting from given index.
    pub const fn limit(self, limit: Offset) -> Self {
        Self::new(self.columns, self.start, limit)
    }

    const fn new(columns: bool, start: usize, limit: Offset) -> Self {
        Self {
            columns,
            start,
            limit,
        }
    }
}

impl<R, D, C> TableOption<R, C, D> for Reverse
where
    R: Resizable + Records + ExactRecords,
{
    fn change(self, records: &mut R, _: &mut C, _: &mut D) {
        let count_rows = records.count_rows();
        let count_columns = records.count_columns();

        let start = self.start;

        if self.columns {
            let end = match self.limit {
                Offset::Begin(limit) => start + limit,
                Offset::End(limit) => count_columns - limit,
            };

            if start >= end || end > count_columns {
                return;
            }

            reverse_columns(records, start, end);
        } else {
            let end = match self.limit {
                Offset::Begin(limit) => start + limit,
                Offset::End(limit) => count_rows - limit,
            };

            if start >= end || end > count_rows {
                return;
            }

            reverse_rows(records, start, end);
        }
    }
}

fn reverse_rows<R>(data: &mut R, start: usize, end: usize)
where
    R: Resizable + ExactRecords,
{
    let count_rows = end - start;
    if count_rows < 2 {
        return;
    }

    let mut i = start;
    let mut j = end - 1;

    while i < j {
        data.swap_row(i, j);
        i += 1;
        j -= 1;
    }
}

fn reverse_columns<R>(data: &mut R, start: usize, end: usize)
where
    R: Resizable + Records,
{
    let count_columns = end - start;
    if count_columns < 2 {
        return;
    }

    let mut i = start;
    let mut j = end - 1;

    while i < j {
        data.swap_column(i, j);
        i += 1;
        j -= 1;
    }
}

#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
    use crate::grid::records::{vec_records::VecRecords, Records};

    use super::{reverse_columns, reverse_rows};

    #[test]
    fn test_reverse_rows() {
        assert_eq!(
            rev_rows(vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]),
            vec![vec![6, 7, 8], vec![3, 4, 5], vec![0, 1, 2]]
        )
    }

    #[test]
    fn test_reverse_columns() {
        assert_eq!(
            rev_cols(vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]),
            vec![vec![2, 1, 0], vec![5, 4, 3], vec![8, 7, 6]]
        )
    }

    fn rev_rows(mut data: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let end = data.len();
        reverse_rows(&mut data, 0, end);
        data
    }

    fn rev_cols(data: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let mut records = VecRecords::new(data);
        let end = records.count_columns();
        reverse_columns(&mut records, 0, end);

        records.into()
    }
}
