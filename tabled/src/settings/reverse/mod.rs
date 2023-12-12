use crate::{
    grid::records::{ExactRecords, Records, Resizable},
    settings::TableOption,
};

/// Reverse data on the table.
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Reverse {
    columns: bool,
}

impl Reverse {
    /// Reverse columns.
    pub const fn columns() -> Self {
        Self { columns: true }
    }

    /// Reverse rows.
    pub const fn rows() -> Self {
        Self { columns: false }
    }
}

impl<R, D, C> TableOption<R, C, D> for Reverse
where
    R: Resizable + Records + ExactRecords,
{
    fn change(self, records: &mut R, _: &mut C, _: &mut D) {
        match self.columns {
            true => reverse_columns(records),
            false => reverse_rows(records),
        }
    }
}

fn reverse_rows<R>(data: &mut R)
where
    R: Resizable + ExactRecords,
{
    let count_rows = data.count_rows();
    if count_rows < 2 {
        return;
    }

    for row in 0..count_rows / 2 {
        data.swap_row(row, count_rows - row - 1);
    }
}

fn reverse_columns<R>(data: &mut R)
where
    R: Resizable + Records,
{
    let count_columns = data.count_columns();
    if count_columns < 2 {
        return;
    }

    for col in 0..count_columns / 2 {
        data.swap_column(col, count_columns - col - 1);
    }
}

#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
    use crate::grid::records::vec_records::VecRecords;

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
        reverse_rows(&mut data);
        data
    }

    fn rev_cols(data: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let mut records = VecRecords::new(data);
        reverse_columns(&mut records);

        records.into()
    }
}
