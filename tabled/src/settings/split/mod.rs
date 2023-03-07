//! Split setting is a work in progress

use crate::records::{ExactRecords, Records, Resizable};

use super::TableOption;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Column,
    Row,
}

#[derive(Debug, Clone, Copy)]
pub enum Behavior {
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

    pub fn set_behavior(self, behavior: Behavior) -> Self {
        Self { behavior, ..self }
    }
}

impl<RR, D, Cfg> TableOption<RR, D, Cfg> for Split
where
    RR: Records + ExactRecords + Resizable,
{
    fn change(&mut self, records: &mut RR, _: &mut Cfg, _: &mut D) {
        let Split {
            direction,
            behavior,
            index,
        } = &self;
        let before_rows = records.count_rows();
        let before_columns = records.count_columns();

        match (direction, behavior) {
            (Direction::Column, Behavior::Append) => {
                let sections_per_row =
                    before_columns / index + { usize::from(before_columns % index != 0) };

                for section in 1..sections_per_row {
                    for reference_row in 0..before_rows {
                        let target_row = reference_row + section * before_rows;
                        records.push_row();

                        for target_column in 0..*index {
                            let reference_column = target_column + section * index;

                            if reference_column < before_columns {
                                records.swap(
                                    (target_row, target_column),
                                    (reference_row, reference_column),
                                );
                            }
                        }
                    }
                }

                for column in (*index..before_columns).rev() {
                    records.remove_column(column)
                }
            }
            (Direction::Column, Behavior::Zip) => {
                let sections_per_row =
                    before_columns / index + { usize::from(before_columns % index != 0) };

                for reference_row in 0..before_rows {
                    let reference_row = reference_row * sections_per_row;

                    for section in 1..sections_per_row {
                        let target_row = reference_row + section;

                        records.insert_row(target_row);

                        for target_column in 0..*index {
                            let reference_column = target_column + (section * index);

                            if reference_column < before_columns {
                                records.swap(
                                    (target_row, target_column),
                                    (reference_row, reference_column),
                                );
                            }
                        }
                    }
                }

                for column in (*index..before_columns).rev() {
                    records.remove_column(column)
                }
            }
            (Direction::Row, Behavior::Append) => {
                let sections_per_column =
                    before_rows / index + { usize::from(before_rows % index != 0) };

                for section in 1..sections_per_column {
                    for reference_column in 0..before_columns {
                        let target_column = reference_column + section * before_columns;
                        records.push_column();

                        for target_row in 0..*index {
                            let reference_row = target_row + section * index;

                            if reference_row < before_rows {
                                records.swap(
                                    (target_row, target_column),
                                    (reference_row, reference_column),
                                );
                            }
                        }
                    }
                }

                for row in (*index..before_rows).rev() {
                    records.remove_row(row)
                }
            }
            (Direction::Row, Behavior::Zip) => {
                let sections_per_column =
                    before_rows / index + { usize::from(before_rows % index != 0) };

                for mut reference_column in 0..before_columns {
                    reference_column = reference_column * sections_per_column;

                    for section in 1..sections_per_column {
                        let target_column = reference_column + section;

                        records.insert_column(target_column);

                        for target_row in 0..*index {
                            let reference_row = target_row + (section * index);

                            if reference_row < before_rows {
                                records.swap(
                                    (target_row, target_column),
                                    (reference_row, reference_column),
                                );
                            }
                        }
                    }
                }

                for row in (*index..before_rows).rev() {
                    records.remove_row(row)
                }
            }
        }
    }
}
