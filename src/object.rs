use std::ops::{Bound, RangeBounds};

pub trait Object {
    fn cells(&self, count_rows: usize, count_columns: usize) -> Vec<(usize, usize)>;
}

pub struct Head;

impl Object for Head {
    fn cells(&self, _: usize, count_columns: usize) -> Vec<(usize, usize)> {
        (0..count_columns).map(|column| (0, column)).collect()
    }
}

pub struct Full;

impl Object for Full {
    fn cells(&self, count_rows: usize, count_columns: usize) -> Vec<(usize, usize)> {
        (0..count_rows)
            .map(|row| {
                (0..count_columns)
                    .map(|column| (row, column))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
            .concat()
    }
}

pub struct Row<R: RangeBounds<usize>>(pub R);

impl<R: RangeBounds<usize>> Object for Row<R> {
    fn cells(&self, count_rows: usize, count_columns: usize) -> Vec<(usize, usize)> {
        let (x, y) = bounds_to_usize(self.0.start_bound(), self.0.end_bound(), count_rows);

        (x..y)
            .map(|row| (0..count_columns).map(|column| (row, column)).collect())
            .collect::<Vec<Vec<_>>>()
            .concat()
    }
}

pub struct Column<R: RangeBounds<usize>>(pub R);

impl<R: RangeBounds<usize>> Object for Column<R> {
    fn cells(&self, count_rows: usize, count_columns: usize) -> Vec<(usize, usize)> {
        let (x, y) = bounds_to_usize(self.0.start_bound(), self.0.end_bound(), count_columns);

        (x..y)
            .map(|column| (0..count_rows).map(|row| (row, column)).collect())
            .collect::<Vec<Vec<_>>>()
            .concat()
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
