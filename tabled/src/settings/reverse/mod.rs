use crate::{
    grid::records::{ExactRecords, Records, Resizable},
    settings::TableOption,
};

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Reverse {
    columns: bool,
}

impl Reverse {
    pub const fn columns() -> Self {
        Self { columns: true }
    }

    pub const fn rows() -> Self {
        Self { columns: false }
    }
}

impl<R, D, C> TableOption<R, D, C> for Reverse
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
