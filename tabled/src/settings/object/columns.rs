use std::ops::{Add, RangeBounds, Sub};

use crate::{
    grid::config::Entity,
    grid::records::{ExactRecords, Records},
    settings::object::{cell::EntityOnce, Object},
};

use super::util::bounds_to_usize;

/// Column denotes a set of cells on given columns on a [`Table`].
///
/// [`Table`]: crate::Table
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Columns<R> {
    range: R,
}

impl<R> Columns<R> {
    /// Returns a new instance of [`Columns`] for a range of columns.
    ///
    /// If the boundaries are exceeded it may panic.
    pub fn new(range: R) -> Self
    where
        R: RangeBounds<usize>,
    {
        Self { range }
    }

    pub(crate) fn get_range(&self) -> &R {
        &self.range
    }
}

impl Columns<()> {
    /// Returns a new instance of [`Columns`] for a single column.
    ///
    /// If the boundaries are exceeded it may panic.
    pub fn one(index: usize) -> Column {
        Column(index)
    }

    /// Returns a new instance of [`Columns`] for a first column.
    ///
    /// If the boundaries are exceeded the object will produce no cells.
    pub fn first() -> FirstColumn {
        FirstColumn
    }

    /// Returns a new instance of [`Columns`] for a last column.
    ///
    /// If the boundaries are exceeded the object will produce no cells.
    pub fn last() -> LastColumn {
        LastColumn
    }
}

impl<I, R> Object<I> for Columns<R>
where
    R: RangeBounds<usize>,
    I: Records,
{
    type Iter = ColumnsIter;

    fn cells(&self, records: &I) -> Self::Iter {
        let max = records.count_columns();
        let start = self.range.start_bound();
        let end = self.range.end_bound();
        let (x, y) = bounds_to_usize(start, end, max);

        ColumnsIter::new(x, y)
    }
}

/// `FirstColumn` represents the first column on a grid.
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FirstColumn;

impl<I> Object<I> for FirstColumn
where
    I: Records + ExactRecords,
{
    type Iter = EntityOnce;

    fn cells(&self, records: &I) -> Self::Iter {
        if records.count_rows() == 0 || records.count_columns() == 0 {
            return EntityOnce::new(None);
        }

        EntityOnce::new(Some(Entity::Column(0)))
    }
}

impl Add<usize> for FirstColumn {
    type Output = Column;

    fn add(self, rhs: usize) -> Self::Output {
        Column(rhs)
    }
}

/// `LastColumn` represents the last column on a grid.
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LastColumn;

impl<I> Object<I> for LastColumn
where
    I: Records + ExactRecords,
{
    type Iter = EntityOnce;

    fn cells(&self, records: &I) -> Self::Iter {
        if records.count_rows() == 0 || records.count_columns() == 0 {
            return EntityOnce::new(None);
        }

        let col = records.count_columns().saturating_sub(1);
        EntityOnce::new(Some(Entity::Column(col)))
    }
}

impl Sub<usize> for LastColumn {
    type Output = LastColumnOffset;

    fn sub(self, rhs: usize) -> Self::Output {
        LastColumnOffset::sub(rhs)
    }
}

impl Add<usize> for LastColumn {
    type Output = LastColumnOffset;

    fn add(self, rhs: usize) -> Self::Output {
        LastColumnOffset::add(rhs)
    }
}

/// Column represents a single column on a grid.
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Column(usize);

impl<I> Object<I> for Column {
    type Iter = EntityOnce;

    fn cells(&self, _: &I) -> Self::Iter {
        EntityOnce::new(Some(Entity::Column(self.0)))
    }
}

impl From<usize> for Column {
    fn from(i: usize) -> Self {
        Self(i)
    }
}

impl From<Column> for usize {
    fn from(val: Column) -> Self {
        val.0
    }
}

/// `LastColumnOffset` represents a single column on a grid indexed via offset from the last column.
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LastColumnOffset {
    offset: usize,
    sign: bool,
}

impl LastColumnOffset {
    fn sub(offset: usize) -> Self {
        Self {
            offset,
            sign: false,
        }
    }

    fn add(offset: usize) -> Self {
        Self { offset, sign: true }
    }
}

impl<I> Object<I> for LastColumnOffset
where
    I: Records,
{
    type Iter = EntityOnce;

    fn cells(&self, records: &I) -> Self::Iter {
        let count_cols = records.count_columns();
        if count_cols == 0 {
            return EntityOnce::new(None);
        }

        let last_col = count_cols - 1;

        if self.sign {
            let col = last_col + self.offset;
            EntityOnce::new(Some(Entity::Column(col)))
        } else {
            if self.offset > last_col {
                return EntityOnce::new(None);
            }

            let col = last_col - self.offset;
            EntityOnce::new(Some(Entity::Column(col)))
        }
    }
}

/// An [`Iterator`] which goes goes over columns of a [`Table`].
///
/// [`Table`]: crate::Table
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ColumnsIter {
    start: usize,
    end: usize,
}

impl ColumnsIter {
    const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

impl Iterator for ColumnsIter {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }

        let col = self.start;
        self.start += 1;

        Some(Entity::Column(col))
    }
}
