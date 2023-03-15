//! The module contains a [`Locator`] trait and implementations for it.

use core::ops::Bound;
use std::{
    iter::{self, Once},
    ops::{Range, RangeBounds},
};

use crate::{
    grid::config::Entity,
    grid::records::{ExactRecords, PeekableRecords, Records},
    settings::object::{
        Column, Columns, FirstColumn, FirstRow, LastColumn, LastRow, Object, Row, Rows,
    },
};

/// Locator is an interface which searches for a particular thing in the [`Records`],
/// and returns coordinate of the foundings if any.
pub trait Locator<Records> {
    /// A coordinate of the finding.
    type Coordinate;
    /// An iterator of the coordinates.
    /// If it's empty it's considered that nothing is found.
    type IntoIter: IntoIterator<Item = Self::Coordinate>;

    /// Search for the thing in [`Records`], returning a list of coordinates.
    fn locate(&mut self, records: Records) -> Self::IntoIter;
}

impl<B, R> Locator<R> for Columns<B>
where
    B: RangeBounds<usize>,
    R: Records,
{
    type Coordinate = usize;
    type IntoIter = Range<usize>;

    fn locate(&mut self, records: R) -> Self::IntoIter {
        let range = self.get_range();
        let max = records.count_columns();
        let (from, to) = bounds_to_usize(range.start_bound(), range.end_bound(), max);

        from..to
    }
}

impl<R> Locator<R> for Column {
    type Coordinate = usize;
    type IntoIter = Once<usize>;

    fn locate(&mut self, _: R) -> Self::IntoIter {
        iter::once((*self).into())
    }
}

impl<R> Locator<R> for FirstColumn {
    type Coordinate = usize;
    type IntoIter = Once<usize>;

    fn locate(&mut self, _: R) -> Self::IntoIter {
        iter::once(0)
    }
}

impl<R> Locator<R> for LastColumn
where
    R: Records,
{
    type Coordinate = usize;
    type IntoIter = Once<usize>;

    fn locate(&mut self, records: R) -> Self::IntoIter {
        if records.count_columns() > 0 {
            iter::once(records.count_columns() - 1)
        } else {
            iter::once(0)
        }
    }
}

impl<B, R> Locator<R> for Rows<B>
where
    R: Records,
    B: RangeBounds<usize>,
{
    type Coordinate = usize;
    type IntoIter = Range<usize>;

    fn locate(&mut self, records: R) -> Self::IntoIter {
        let (from, to) = bounds_to_usize(
            self.get_range().start_bound(),
            self.get_range().end_bound(),
            records.count_columns(),
        );

        from..to
    }
}

impl<R> Locator<R> for Row {
    type Coordinate = usize;
    type IntoIter = Once<usize>;

    fn locate(&mut self, _: R) -> Self::IntoIter {
        iter::once((*self).into())
    }
}

impl<R> Locator<R> for FirstRow {
    type Coordinate = usize;
    type IntoIter = Once<usize>;

    fn locate(&mut self, _: R) -> Self::IntoIter {
        iter::once(0)
    }
}

impl<R> Locator<R> for LastRow
where
    R: ExactRecords,
{
    type Coordinate = usize;
    type IntoIter = Once<usize>;

    fn locate(&mut self, records: R) -> Self::IntoIter {
        if records.count_rows() > 0 {
            iter::once(records.count_rows() - 1)
        } else {
            iter::once(0)
        }
    }
}

/// The structure is an implementation of [`Locator`] to search for a column by it's name.
/// A name is considered be a value in a first row.
///
/// So even if in reality there's no header, the first row will be considered to be one.
#[derive(Debug, Clone, Copy)]
pub struct ByColumnName<S>(S);

impl<S> ByColumnName<S> {
    /// Constructs a new object of the structure.
    pub fn new(text: S) -> Self
    where
        S: AsRef<str>,
    {
        Self(text)
    }
}

impl<R, S> Locator<R> for ByColumnName<S>
where
    S: AsRef<str>,
    R: Records + ExactRecords + PeekableRecords,
{
    type Coordinate = usize;
    type IntoIter = Vec<usize>;

    fn locate(&mut self, records: R) -> Self::IntoIter {
        // todo: can be optimized by creating Iterator
        (0..records.count_columns())
            .filter(|col| records.get_text((0, *col)) == self.0.as_ref())
            .collect::<Vec<_>>()
    }
}

impl<S, R> Object<R> for ByColumnName<S>
where
    S: AsRef<str>,
    R: Records + PeekableRecords + ExactRecords,
{
    type Iter = std::vec::IntoIter<Entity>;

    fn cells(&self, records: &R) -> Self::Iter {
        // todo: can be optimized by creating Iterator
        (0..records.count_columns())
            .filter(|col| records.get_text((0, *col)) == self.0.as_ref())
            .map(Entity::Column)
            .collect::<Vec<_>>()
            .into_iter()
    }
}

fn bounds_to_usize(
    left: Bound<&usize>,
    right: Bound<&usize>,
    count_elements: usize,
) -> (usize, usize) {
    match (left, right) {
        (Bound::Included(x), Bound::Included(y)) => (*x, y + 1),
        (Bound::Included(x), Bound::Excluded(y)) => (*x, *y),
        (Bound::Included(x), Bound::Unbounded) => (*x, count_elements),
        (Bound::Unbounded, Bound::Unbounded) => (0, count_elements),
        (Bound::Unbounded, Bound::Included(y)) => (0, y + 1),
        (Bound::Unbounded, Bound::Excluded(y)) => (0, *y),
        (Bound::Excluded(_), Bound::Unbounded)
        | (Bound::Excluded(_), Bound::Included(_))
        | (Bound::Excluded(_), Bound::Excluded(_)) => {
            unreachable!("A start bound can't be excluded")
        }
    }
}
