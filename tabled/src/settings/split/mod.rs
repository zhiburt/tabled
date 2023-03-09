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

    pub fn append(self) -> Self {
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

impl<RR, D, Cfg> TableOption<RR, D, Cfg> for Split
where
    RR: Records + ExactRecords + Resizable,
{
    fn change(&mut self, records: &mut RR, _: &mut Cfg, _: &mut D) {
        let columns = records.count_columns();
        let rows = records.count_rows();
        match (self.direction, self.behavior) {
            (Direction::Column, Behavior::Append) => {
                split_column_append(records, columns, rows, self.index)
            }
            (Direction::Column, Behavior::Zip) => {
                split_column_zip(records, columns, rows, self.index)
            }
            (Direction::Row, Behavior::Append) => {
                split_row_append(records, columns, rows, self.index)
            }
            (Direction::Row, Behavior::Zip) => split_row_zip(records, columns, rows, self.index),
        }
    }
}

fn split_column_append<R>(records: &mut R, columns: usize, rows: usize, index: usize)
where
    R: Resizable,
{
    let sections_per_row = columns / index + { usize::from(columns % index != 0) };

    for section in 1..sections_per_row {
        for reference_row in 0..rows {
            let target_row = reference_row + section * rows;
            records.push_row();

            for target_column in 0..index {
                let reference_column = target_column + section * index;

                if reference_column < columns {
                    records.swap(
                        (target_row, target_column),
                        (reference_row, reference_column),
                    );
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
    let sections_per_row = columns / index + { usize::from(columns % index != 0) };

    for mut reference_row in 0..rows {
        reference_row *= sections_per_row;

        for section in 1..sections_per_row {
            let target_row = reference_row + section;

            records.insert_row(target_row);

            for target_column in 0..index {
                let reference_column = target_column + (section * index);

                if reference_column < columns {
                    records.swap(
                        (target_row, target_column),
                        (reference_row, reference_column),
                    );
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
    let sections_per_column = rows / index + { usize::from(rows % index != 0) };

    for section in 1..sections_per_column {
        for reference_column in 0..columns {
            let target_column = reference_column + section * columns;
            records.push_column();

            for target_row in 0..index {
                let reference_row = target_row + section * index;

                if reference_row < rows {
                    records.swap(
                        (target_row, target_column),
                        (reference_row, reference_column),
                    );
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
    let sections_per_column = rows / index + { usize::from(rows % index != 0) };

    for mut reference_column in 0..columns {
        reference_column *= sections_per_column;

        for section in 1..sections_per_column {
            let target_column = reference_column + section;

            records.insert_column(target_column);

            for target_row in 0..index {
                let reference_row = target_row + (section * index);

                if reference_row < rows {
                    records.swap(
                        (target_row, target_column),
                        (reference_row, reference_column),
                    );
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
