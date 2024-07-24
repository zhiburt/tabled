use crate::{
    grid::records::{ExactRecords, Records, Resizable},
    settings::TableOption,
};

/// Reverse data on the table.
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Reverse {
    columns: bool,
    skip: usize,
    skip_from_end: usize,
}

impl Reverse {
    /// Reverse columns.
    pub const fn columns(start: usize, end: usize) -> Self {
        Self::new(true, start, end)
    }

    /// Reverse rows.
    pub const fn rows(start: usize, end: usize) -> Self {
        Self::new(false, start, end)
    }

    const fn new(columns: bool, skip: usize, skip_from_end: usize) -> Self {
        Self {
            columns,
            skip,
            skip_from_end,
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

        let skip = self.skip_from_end + self.skip;

        if self.columns {
            if count_columns <= skip {
                return;
            }

            let start = self.skip;
            let end = count_columns - self.skip_from_end;

            reverse_columns(records, start, end);
        } else {
            if count_rows <= skip {
                return;
            }

            let start = self.skip;
            let end = count_rows - self.skip_from_end;

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

    for (i, row) in (start..end / 2).enumerate() {
        data.swap_row(row, end - i - 1);
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

    for (i, col) in (start..end / 2).enumerate() {
        data.swap_column(col, end - i - 1);
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
