#[allow(unused)]
use papergrid::Grid;
use std::{
    collections::BTreeSet,
    ops::{Bound, Range, RangeBounds, RangeFull},
};

/// Object helps to locate a nessesary part of a [Grid].
pub trait Object: Sized {
    /// Cells returns a set of cordinates of cells
    fn cells(&self, count_rows: usize, count_columns: usize) -> Vec<(usize, usize)>;

    /// Combines cells.
    /// It doesn't repeat cells.
    fn and<O: Object>(self, rhs: O) -> Combination<Self, O> {
        Combination {
            lhs: self,
            rhs,
            combinator: combine_cells,
        }
    }

    /// Excludes rhs cells from this cells.
    fn not<O: Object>(self, rhs: O) -> Combination<Self, O> {
        Combination {
            lhs: self,
            rhs,
            combinator: remove_cells,
        }
    }
}

/// Segment represents a sub table of [Table].
pub struct Segment<C, R> {
    columns: C,
    rows: R,
}

impl Segment<RangeFull, RangeFull> {
    /// Returns a full table segment.
    ///
    /// The same as [Full].
    pub fn all() -> Self {
        Self::new(.., ..)
    }
}

impl<C, R> Segment<C, R>
where
    C: RangeBounds<usize>,
    R: RangeBounds<usize>,
{
    /// This function builds a [Segment].
    pub fn new(rows: R, columns: C) -> Self {
        Self { columns, rows }
    }
}

impl<C, R> Object for Segment<C, R>
where
    C: RangeBounds<usize>,
    R: RangeBounds<usize>,
{
    fn cells(&self, count_rows: usize, count_columns: usize) -> Vec<(usize, usize)> {
        let (rows_start, rows_end) =
            bounds_to_usize(self.rows.start_bound(), self.rows.end_bound(), count_rows);

        let (columns_start, columns_end) = bounds_to_usize(
            self.columns.start_bound(),
            self.columns.end_bound(),
            count_columns,
        );

        let mut cells = Vec::new();
        (rows_start..rows_end)
            .for_each(|row| (columns_start..columns_end).for_each(|col| cells.push((row, col))));

        cells
    }
}

/// Frame includes cells which are on the edges of each side.
/// Therefore it's [Object] implementation returns a subset of cells which are present in frame.
pub struct Frame;

impl Object for Frame {
    fn cells(&self, count_rows: usize, count_columns: usize) -> Vec<(usize, usize)> {
        let mut cells = Vec::new();

        if count_rows > 0 {
            (0..count_columns).for_each(|col| {
                cells.push((0, col));
            });

            (0..count_columns).for_each(|col| {
                cells.push((count_rows - 1, col));
            });
        }

        if count_columns > 0 {
            (0..count_rows).for_each(|row| {
                cells.push((row, 0));
            });

            (0..count_rows).for_each(|row| {
                cells.push((row, count_columns - 1));
            });
        }

        cells
    }
}

/// FirstRow represents the first row of a [Table].
/// It's often contains headers data.
pub struct FirstRow;

impl Object for FirstRow {
    fn cells(&self, _: usize, count_columns: usize) -> Vec<(usize, usize)> {
        (0..count_columns).map(|column| (0, column)).collect()
    }
}

/// LastRow represents the last row of a [Table].
pub struct LastRow;

impl Object for LastRow {
    fn cells(&self, count_rows: usize, count_columns: usize) -> Vec<(usize, usize)> {
        let row = if count_rows == 0 { 0 } else { count_rows - 1 };
        (0..count_columns).map(|column| (row, column)).collect()
    }
}

/// Full represents all cells on a [Grid]
pub struct Full;

impl Object for Full {
    fn cells(&self, count_rows: usize, count_columns: usize) -> Vec<(usize, usize)> {
        Segment::new(.., ..).cells(count_rows, count_columns)
    }
}

/// Row denotes a set of cells on given rows on a [Grid].
pub struct Rows<R> {
    range: R,
}

impl<R> Rows<R>
where
    R: RangeBounds<usize>,
{
    /// Returns a new instance of [Rows] for a range of rows.
    ///
    /// If the boundries are exeeded it may panic.
    pub fn new(range: R) -> Self {
        Self { range }
    }
}

impl Rows<Range<usize>> {
    /// Returns a new instance of [Rows] with a single row.
    ///
    /// If the boundries are exeeded it may panic.
    pub fn single(index: usize) -> Self {
        Self {
            range: index..index + 1,
        }
    }
}

impl Rows<()> {
    /// Returns a first row [Object].
    ///
    /// If the table has 0 rows returns an empty set of cells.
    pub fn first() -> FirstRow {
        FirstRow
    }

    /// Returns a last row [Object].
    ///
    /// If the table has 0 rows returns an empty set of cells.
    pub fn last() -> LastRow {
        LastRow
    }
}

impl<R> Object for Rows<R>
where
    R: RangeBounds<usize>,
{
    fn cells(&self, count_rows: usize, count_columns: usize) -> Vec<(usize, usize)> {
        let (x, y) = bounds_to_usize(self.range.start_bound(), self.range.end_bound(), count_rows);

        (x..y)
            .map(|row| (0..count_columns).map(|column| (row, column)).collect())
            .collect::<Vec<Vec<_>>>()
            .concat()
    }
}

/// Column denotes a set of cells on given columns on a [Grid].
pub struct Columns<R> {
    range: R,
}

impl<R> Columns<R>
where
    R: RangeBounds<usize>,
{
    /// Returns a new instance of [Columns] for a range of columns.
    ///
    /// If the boundries are exeeded it may panic.
    pub fn new(range: R) -> Self {
        Self { range }
    }
}

impl Columns<Range<usize>> {
    /// Returns a new instance of [Columns] for a single column.
    ///
    /// If the boundries are exeeded it may panic.
    pub fn single(index: usize) -> Self {
        Self {
            range: index..index + 1,
        }
    }
}

impl<R> Object for Columns<R>
where
    R: RangeBounds<usize>,
{
    fn cells(&self, count_rows: usize, count_columns: usize) -> Vec<(usize, usize)> {
        let (x, y) = bounds_to_usize(
            self.range.start_bound(),
            self.range.end_bound(),
            count_columns,
        );

        (x..y)
            .map(|column| (0..count_rows).map(|row| (row, column)).collect())
            .collect::<Vec<Vec<_>>>()
            .concat()
    }
}

/// Cell denotes a particular cell on a [Grid].
pub struct Cell(pub usize, pub usize);

impl Object for Cell {
    fn cells(&self, _: usize, _: usize) -> Vec<(usize, usize)> {
        vec![(self.0, self.1)]
    }
}

/// Combinator is a transformation function
type Combinator = fn(Vec<(usize, usize)>, Vec<(usize, usize)>) -> Vec<(usize, usize)>;

/// Combination struct used for chaning [Object]'s.
pub struct Combination<L, R> {
    lhs: L,
    rhs: R,
    combinator: Combinator,
}

impl<L, R> Object for Combination<L, R>
where
    L: Object,
    R: Object,
{
    fn cells(&self, count_rows: usize, count_columns: usize) -> Vec<(usize, usize)> {
        let l = self.lhs.cells(count_rows, count_columns);
        let r = self.rhs.cells(count_rows, count_columns);
        (self.combinator)(l, r)
    }
}

/// Combines 2 sets of cells into one.
///
/// Dublicates are removed from the output set.
fn combine_cells(lhs: Vec<(usize, usize)>, rhs: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    lhs.into_iter()
        .chain(rhs.into_iter())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

/// Removes cells from fist set which are present in a second set.
fn remove_cells(lhs: Vec<(usize, usize)>, rhs: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    lhs.into_iter().filter(|l| !rhs.contains(l)).collect()
}

/// Converts a range bound to its indexes.
pub(crate) fn bounds_to_usize(
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
