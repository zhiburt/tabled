//! Split setting is a work in progress

use crate::records::{ExactRecords, Records, Resizable};

use super::TableOption;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Column,
    Row,
}

#[derive(Debug, Clone, Copy)]
enum Behavior {
    Append,
    Zip,
}

#[derive(Debug, Clone, Copy)]
pub struct Split {
    direction: Direction,
    behavior: Behavior,
    index: usize,
}

impl Split {
    pub fn column(index: usize) -> Self {
        Split {
            direction: Direction::Column,
            behavior: Behavior::Zip,
            index,
        }
    }

    pub fn row(index: usize) -> Self {
        Split {
            direction: Direction::Row,
            behavior: Behavior::Zip,
            index,
        }
    }

    pub fn concat(self) -> Self {
        Self {
            behavior: Behavior::Append,
            ..self
        }
    }

    pub fn zip(self) -> Self {
        Self {
            behavior: Behavior::Zip,
            ..self
        }
    }
}

impl<R, D, Cfg> TableOption<R, D, Cfg> for Split
where
    R: Records + ExactRecords + Resizable,
{
    fn change(&mut self, records: &mut R, _: &mut Cfg, _: &mut D) {
        use Behavior::*;
        use Direction::*;

        let columns = records.count_columns();
        let rows = records.count_rows();

        if columns == 0 || rows == 0 {
            return;
        }

        match (self.direction, self.behavior) {
            (Column, Append) => split_column_append(records, columns, rows, self.index),
            (Column, Zip) => split_column_zip(records, columns, rows, self.index),
            (Row, Append) => split_row_append(records, columns, rows, self.index),
            (Row, Zip) => split_row_zip(records, columns, rows, self.index),
        }
    }
}

fn split_column_append<R>(records: &mut R, columns: usize, rows: usize, index: usize)
where
    R: Resizable,
{
    let sections_per_row = ceil_div(columns, index);

    for section in 1..sections_per_row {
        for ref_row in 0..rows {
            let target_row = ref_row + section * rows;
            records.push_row();

            for target_col in 0..index {
                let ref_col = target_col + section * index;

                if ref_col < columns {
                    records.swap((target_row, target_col), (ref_row, ref_col));
                }
            }
        }
    }

    clean_columns(records, (index..columns).rev())
}

fn split_column_zip<R>(records: &mut R, columns: usize, rows: usize, index: usize)
where
    R: Resizable,
{
    let sections_per_row = ceil_div(columns, index);

    for mut ref_row in 0..rows {
        ref_row *= sections_per_row;

        for section in 1..sections_per_row {
            let target_row = ref_row + section;
            records.insert_row(target_row);

            for target_col in 0..index {
                let ref_col = target_col + (section * index);

                if ref_col < columns {
                    records.swap((target_row, target_col), (ref_row, ref_col));
                }
            }
        }
    }

    clean_columns(records, (index..columns).rev())
}

fn split_row_append<R>(records: &mut R, columns: usize, rows: usize, index: usize)
where
    R: Resizable,
{
    let per_column = ceil_div(rows, index);

    for section in 1..per_column {
        for ref_col in 0..columns {
            let target_col = ref_col + section * columns;
            records.push_column();

            for target_row in 0..index {
                let ref_row = target_row + section * index;

                if ref_row < rows {
                    records.swap((target_row, target_col), (ref_row, ref_col));
                }
            }
        }
    }

    clean_rows(records, (index..rows).rev());
}

fn split_row_zip<R>(records: &mut R, columns: usize, rows: usize, index: usize)
where
    R: Resizable,
{
    let per_column = ceil_div(rows, index);

    for col in 0..columns {
        let ref_col = col * per_column;

        for section in 1..per_column {
            let target_col = ref_col + section;
            records.insert_column(target_col);

            for target_row in 0..index {
                let ref_row = target_row + (section * index);

                if ref_row < rows {
                    records.swap((target_row, target_col), (ref_row, ref_col));
                }
            }
        }
    }

    clean_rows(records, (index..rows).rev());
}

fn clean_rows<R, I>(records: &mut R, range: I)
where
    R: Resizable,
    I: Iterator<Item = usize>,
{
    for row in range {
        records.remove_row(row)
    }
}

fn clean_columns<R, I>(records: &mut R, range: I)
where
    R: Resizable,
    I: Iterator<Item = usize>,
{
    for column in range {
        records.remove_column(column)
    }
}

fn ceil_div(x: usize, y: usize) -> usize {
    debug_assert!(x != 0);
    1 + ((x - 1) / y)
}
