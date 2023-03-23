use std::ops::{Add, RangeBounds, Sub};

use crate::{
    grid::config::Entity,
    grid::records::{ExactRecords, Records},
    settings::object::{cell::EntityOnce, Object},
};

use super::util::bounds_to_usize;

/// Row denotes a set of cells on given rows on a [`Table`].
///
/// [`Table`]: crate::Table
#[derive(Debug)]
pub struct Rows<R> {
    range: R,
}

impl<R> Rows<R> {
    /// Returns a new instance of [`Rows`] for a range of rows.
    ///
    /// If the boundaries are exceeded it may panic.
    pub fn new(range: R) -> Self
    where
        R: RangeBounds<usize>,
    {
        Self { range }
    }

    pub(crate) const fn get_range(&self) -> &R {
        &self.range
    }
}

impl Rows<()> {
    /// Returns a new instance of [`Rows`] with a single row.
    ///
    /// If the boundaries are exceeded it may panic.
    pub const fn single(index: usize) -> Row {
        Row { index }
    }

    /// Returns a first row [`Object`].
    ///
    /// If the table has 0 rows returns an empty set of cells.
    pub const fn first() -> FirstRow {
        FirstRow
    }

    /// Returns a last row [`Object`].
    ///
    /// If the table has 0 rows returns an empty set of cells.
    pub const fn last() -> LastRow {
        LastRow
    }
}

impl<I, R> Object<I> for Rows<R>
where
    R: RangeBounds<usize>,
    I: ExactRecords,
{
    type Iter = RowsIter;

    fn cells(&self, records: &I) -> Self::Iter {
        let start = self.range.start_bound();
        let end = self.range.end_bound();
        let max = records.count_rows();
        let (x, y) = bounds_to_usize(start, end, max);

        RowsIter::new(x, y)
    }
}

/// A row which is located by an offset from the first row.
#[derive(Debug, Clone, Copy)]
pub struct Row {
    index: usize,
}

impl<I> Object<I> for Row {
    type Iter = EntityOnce;

    fn cells(&self, _: &I) -> Self::Iter {
        EntityOnce::new(Some(Entity::Row(self.index)))
    }
}

impl From<Row> for usize {
    fn from(val: Row) -> Self {
        val.index
    }
}

/// This structure represents the first row of a [`Table`].
/// It's often contains headers data.
///
/// [`Table`]: crate::Table
#[derive(Debug)]
pub struct FirstRow;

impl<I> Object<I> for FirstRow
where
    I: Records + ExactRecords,
{
    type Iter = EntityOnce;

    fn cells(&self, records: &I) -> Self::Iter {
        if records.count_columns() == 0 || records.count_rows() == 0 {
            return EntityOnce::new(None);
        }

        EntityOnce::new(Some(Entity::Row(0)))
    }
}

impl Add<usize> for FirstRow {
    type Output = Row;

    fn add(self, rhs: usize) -> Self::Output {
        Row { index: rhs }
    }
}

/// This structure represents the last row of a [`Table`].
///
/// [`Table`]: crate::Table
#[derive(Debug)]
pub struct LastRow;

impl<I> Object<I> for LastRow
where
    I: Records + ExactRecords,
{
    type Iter = EntityOnce;

    fn cells(&self, records: &I) -> Self::Iter {
        let count_rows = records.count_rows();
        if records.count_columns() == 0 || count_rows == 0 {
            return EntityOnce::new(None);
        }

        let row = if count_rows == 0 { 0 } else { count_rows - 1 };

        EntityOnce::new(Some(Entity::Row(row)))
    }
}

impl Sub<usize> for LastRow {
    type Output = LastRowOffset;

    fn sub(self, rhs: usize) -> Self::Output {
        LastRowOffset { offset: rhs }
    }
}

/// A row which is located by an offset from the last row.
#[derive(Debug)]
pub struct LastRowOffset {
    offset: usize,
}

impl<I> Object<I> for LastRowOffset
where
    I: Records + ExactRecords,
{
    type Iter = EntityOnce;

    fn cells(&self, records: &I) -> Self::Iter {
        let count_rows = records.count_rows();
        if records.count_columns() == 0 || count_rows == 0 {
            return EntityOnce::new(None);
        }

        let row = if count_rows == 0 { 0 } else { count_rows - 1 };
        if self.offset > row {
            return EntityOnce::new(None);
        }

        let row = row - self.offset;
        EntityOnce::new(Some(Entity::Row(row)))
    }
}

/// An [`Iterator`] which goes goes over all rows of a [`Table`].
///
/// [`Table`]: crate::Table
#[derive(Debug)]
pub struct RowsIter {
    start: usize,
    end: usize,
}

impl RowsIter {
    const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

impl Iterator for RowsIter {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }

        let col = self.start;
        self.start += 1;

        Some(Entity::Row(col))
    }
}
