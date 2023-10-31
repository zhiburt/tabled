//! The module contains a [`Locator`] trait and implementations for it.

mod by_column_name;
mod by_content;

pub use by_column_name::ByColumnName;
pub use by_content::ByContent;

use core::ops::Bound;
use std::{
    iter::{self, Once},
    ops::{Range, RangeBounds},
};

use crate::{
    grid::records::{ExactRecords, Records},
    settings::object::{Column, Columns, FirstColumn, FirstRow, LastColumn, LastRow, Row, Rows},
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

#[cfg(test)]
mod tests {
    use crate::{
        grid::config::Entity,
        grid::records::vec_records::CellInfo,
        grid::records::vec_records::VecRecords,
        settings::locator::{ByColumnName, ByContent},
        settings::object::Object,
    };

    use Entity::*;

    #[test]
    fn object_by_column_name_test() {
        let data = [
            vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]],
            vec![vec![1, 2, 3], vec![1, 1, 3], vec![1, 2, 1]],
            vec![vec![1, 1, 3], vec![1, 1, 3], vec![1, 1, 1]],
            vec![vec![1, 1, 1], vec![1, 1, 3], vec![1, 1, 1]],
            vec![vec![0, 1, 1], vec![1, 1, 3], vec![1, 1, 1]],
            vec![vec![0, 0, 0], vec![1, 1, 3], vec![1, 1, 1]],
        ];

        assert_eq!(cells(by_colname("1"), &data[0]), [Column(0)]);
        assert_eq!(cells(by_colname("1"), &data[1]), [Column(0)]);
        assert_eq!(cells(by_colname("1"), &data[2]), [Column(0), Column(1)]);
        assert_eq!(
            cells(by_colname("1"), &data[3]),
            [Column(0), Column(1), Column(2)]
        );
        assert_eq!(cells(by_colname("1"), &data[4]), [Column(1), Column(2)]);
        assert_eq!(cells(by_colname("1"), &data[5]), []);
    }

    #[test]
    fn object_by_content_test() {
        let data = [
            vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]],
            vec![vec![1, 2, 3], vec![1, 1, 3], vec![1, 2, 1]],
            vec![vec![1, 1, 3], vec![1, 1, 3], vec![1, 1, 1]],
            vec![vec![1, 1, 1], vec![1, 1, 3], vec![1, 1, 1]],
            vec![vec![0, 1, 1], vec![1, 1, 3], vec![1, 1, 1]],
            vec![vec![0, 0, 0], vec![1, 1, 3], vec![1, 1, 1]],
        ];

        assert_eq!(cells(by_content("1"), &[]), []);
        assert_eq!(cells(by_content("1"), &[vec![], vec![], vec![]]), []);
        assert_eq!(
            cells(by_content("1"), &data[0]),
            [Cell(0, 0), Cell(1, 0), Cell(2, 0)]
        );
        assert_eq!(
            cells(by_content("1"), &data[1]),
            [Cell(0, 0), Cell(1, 0), Cell(1, 1), Cell(2, 0), Cell(2, 2)]
        );
        assert_eq!(
            cells(by_content("1"), &data[2]),
            [
                Cell(0, 0),
                Cell(0, 1),
                Cell(1, 0),
                Cell(1, 1),
                Cell(2, 0),
                Cell(2, 1),
                Cell(2, 2)
            ]
        );
        assert_eq!(
            cells(by_content("1"), &data[3]),
            [
                Cell(0, 0),
                Cell(0, 1),
                Cell(0, 2),
                Cell(1, 0),
                Cell(1, 1),
                Cell(2, 0),
                Cell(2, 1),
                Cell(2, 2)
            ]
        );
        assert_eq!(
            cells(by_content("1"), &data[4]),
            [
                Cell(0, 1),
                Cell(0, 2),
                Cell(1, 0),
                Cell(1, 1),
                Cell(2, 0),
                Cell(2, 1),
                Cell(2, 2)
            ]
        );
        assert_eq!(
            cells(by_content("1"), &data[5]),
            [Cell(1, 0), Cell(1, 1), Cell(2, 0), Cell(2, 1), Cell(2, 2)]
        );
    }

    fn by_colname(text: &str) -> ByColumnName<&str> {
        ByColumnName::new(text)
    }

    fn by_content(text: &str) -> ByContent<&str> {
        ByContent::new(text)
    }

    fn cells<O>(o: O, data: &[Vec<usize>]) -> Vec<Entity>
    where
        O: Object<VecRecords<CellInfo<String>>>,
    {
        let data = data
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|n| n.to_string())
                    .map(CellInfo::new)
                    .collect()
            })
            .collect();

        let records = VecRecords::new(data);
        o.cells(&records).collect::<Vec<_>>()
    }
}
