use std::ops::{RangeBounds, RangeFull};

use crate::{
    grid::config::Entity,
    grid::records::{ExactRecords, Records},
    settings::object::{cell::EntityOnce, Object},
};

use super::util::bounds_to_usize;

/// This structure represents a sub table of [`Table`].
///
/// [`Table`]: crate::Table
#[derive(Debug)]
pub struct Segment<C, R> {
    columns: C,
    rows: R,
}

impl Segment<RangeFull, RangeFull> {
    /// Returns a table segment on which are present all cells.
    pub fn all() -> SegmentAll {
        SegmentAll
    }
}

impl<C, R> Segment<C, R>
where
    C: RangeBounds<usize>,
    R: RangeBounds<usize>,
{
    /// This function builds a [`Segment`].
    pub fn new(rows: R, columns: C) -> Self {
        Self { columns, rows }
    }
}

impl<I, C, R> Object<I> for Segment<C, R>
where
    C: RangeBounds<usize>,
    R: RangeBounds<usize>,
    I: Records + ExactRecords,
{
    type Iter = SectorIter;

    fn cells(&self, records: &I) -> Self::Iter {
        let start = self.rows.start_bound();
        let end = self.rows.end_bound();
        let max = records.count_rows();
        let (rows_start, rows_end) = bounds_to_usize(start, end, max);

        let start = self.columns.start_bound();
        let end = self.columns.end_bound();
        let max = records.count_columns();
        let (cols_start, cols_end) = bounds_to_usize(start, end, max);

        SectorIter::new(rows_start, rows_end, cols_start, cols_end)
    }
}

/// This is a segment which contains all cells on the table.
///
/// Can be created from [`Segment::all`].
#[derive(Debug)]
pub struct SegmentAll;

impl<I> Object<I> for SegmentAll {
    type Iter = EntityOnce;

    fn cells(&self, _: &I) -> Self::Iter {
        EntityOnce::new(Some(Entity::Global))
    }
}

/// An [`Iterator`] which goes goes over all cell in a sector in a [`Table`].
///
/// [`Table`]: crate::Table
#[derive(Debug)]
pub struct SectorIter {
    iter: SectorCellsIter,
}

impl SectorIter {
    const fn new(rows_start: usize, rows_end: usize, cols_start: usize, cols_end: usize) -> Self {
        Self {
            iter: SectorCellsIter::new(rows_start, rows_end, cols_start, cols_end),
        }
    }
}

impl Iterator for SectorIter {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        let (row, col) = self.iter.next()?;
        Some(Entity::Cell(row, col))
    }
}

#[derive(Debug)]
pub(crate) struct SectorCellsIter {
    rows_end: usize,
    cols_start: usize,
    cols_end: usize,
    row: usize,
    col: usize,
}

impl SectorCellsIter {
    /// Create an iterator from 1st row to last from 1st col to last.
    pub(crate) const fn new(
        rows_start: usize,
        rows_end: usize,
        cols_start: usize,
        cols_end: usize,
    ) -> Self {
        Self {
            rows_end,
            cols_start,
            cols_end,
            row: rows_start,
            col: cols_start,
        }
    }
}

impl Iterator for SectorCellsIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.rows_end {
            return None;
        }

        if self.col >= self.cols_end {
            return None;
        }

        let row = self.row;
        let col = self.col;

        self.col += 1;

        if self.col == self.cols_end {
            self.row += 1;
            self.col = self.cols_start;
        }

        Some((row, col))
    }
}
