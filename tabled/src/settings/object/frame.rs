use crate::{
    grid::config::Entity,
    grid::records::{ExactRecords, Records},
    settings::object::Object,
};

/// Frame includes cells which are on the edges of each side.
/// Therefore it's [`Object`] implementation returns a subset of cells which are present in frame.
#[derive(Debug)]
pub struct Frame;

impl<I> Object<I> for Frame
where
    I: Records + ExactRecords,
{
    type Iter = FrameIter;

    fn cells(&self, records: &I) -> Self::Iter {
        FrameIter::new(records.count_rows(), records.count_columns())
    }
}

/// An [`Iterator`] which goes goes over all cell on a frame of a [`Table`].
///
/// [`Table`]: crate::Table
#[derive(Debug)]
pub struct FrameIter {
    rows: usize,
    cols: usize,
    row: usize,
    col: usize,
}

impl FrameIter {
    const fn new(count_rows: usize, count_columns: usize) -> Self {
        Self {
            rows: count_rows,
            cols: count_columns,
            row: 0,
            col: 0,
        }
    }
}

impl Iterator for FrameIter {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cols == 0 || self.rows == 0 {
            return None;
        }

        if self.row == self.rows {
            return None;
        }

        let row = self.row;
        let col = self.col;

        self.col += 1;

        if self.col == self.cols {
            self.row += 1;
            self.col = 0;
        }

        Some(Entity::Cell(row, col))
    }
}
