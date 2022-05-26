//! This module contains a list of primitives that implement a [Object] trait.
//! They help to locate a necessary segment on a [Table].
//!
//! [Table]: crate::Table

use std::{
    collections::HashSet,
    ops::{Add, Bound, RangeBounds, RangeFull, Sub},
};

/// Object helps to locate a necessary part of a [Table].
///
/// [Table]: crate::Table
pub trait Object: Sized {
    type Iter: Iterator<Item = (usize, usize)>;

    /// Cells returns a set of coordinates of cells
    fn cells(&self, count_rows: usize, count_columns: usize) -> Self::Iter;

    /// Combines cells.
    /// It doesn't repeat cells.
    fn and<O: Object>(self, rhs: O) -> UntionCombination<Self, O> {
        UntionCombination { lhs: self, rhs }
    }

    /// Excludes rhs cells from this cells.
    fn not<O: Object>(self, rhs: O) -> DiffCombination<Self, O> {
        DiffCombination { lhs: self, rhs }
    }
}

/// Segment represents a sub table of [Table].
///
/// [Table]: crate::Table
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
    type Iter = SectorIter;

    fn cells(&self, count_rows: usize, count_columns: usize) -> Self::Iter {
        let (rows_start, rows_end) =
            bounds_to_usize(self.rows.start_bound(), self.rows.end_bound(), count_rows);

        let (cols_start, cols_end) = bounds_to_usize(
            self.columns.start_bound(),
            self.columns.end_bound(),
            count_columns,
        );

        SectorIter::new(rows_start, rows_end, cols_start, cols_end)
    }
}

/// Frame includes cells which are on the edges of each side.
/// Therefore it's [Object] implementation returns a subset of cells which are present in frame.
pub struct Frame;

impl Object for Frame {
    type Iter = FrameIter;

    fn cells(&self, count_rows: usize, count_columns: usize) -> Self::Iter {
        FrameIter::new(count_rows, count_columns)
    }
}

/// FirstRow represents the first row of a [Table].
/// It's often contains headers data.
///
/// [Table]: crate::Table
pub struct FirstRow;

impl Object for FirstRow {
    type Iter = RowIter;

    fn cells(&self, _: usize, count_columns: usize) -> Self::Iter {
        RowIter::new(0, count_columns)
    }
}

impl Add<usize> for FirstRow {
    type Output = Row;

    fn add(self, rhs: usize) -> Self::Output {
        Row { index: rhs }
    }
}

/// LastRow represents the last row of a [Table].
///
/// [Table]: crate::Table
pub struct LastRow;

impl Object for LastRow {
    type Iter = RowIter;

    fn cells(&self, count_rows: usize, count_columns: usize) -> Self::Iter {
        let row = if count_rows == 0 { 0 } else { count_rows - 1 };
        RowIter::new(row, count_columns)
    }
}

impl Sub<usize> for LastRow {
    type Output = LastRowOffset;

    fn sub(self, rhs: usize) -> Self::Output {
        LastRowOffset { offset: rhs }
    }
}

/// A row which is located by an offset from the first row.
pub struct Row {
    index: usize,
}

impl Object for Row {
    type Iter = RowIter;

    fn cells(&self, count_rows: usize, count_columns: usize) -> Self::Iter {
        if self.index >= count_rows {
            return RowIter::new(0, 0);
        }

        RowIter::new(self.index, count_columns)
    }
}

/// A row which is located by an offset from the last row.
pub struct LastRowOffset {
    offset: usize,
}

impl Object for LastRowOffset {
    type Iter = RowIter;

    fn cells(&self, count_rows: usize, count_columns: usize) -> Self::Iter {
        let row = if count_rows == 0 { 0 } else { count_rows - 1 };
        if self.offset > row {
            return RowIter::new(0, 0);
        }

        let row = row - self.offset;
        RowIter::new(row, count_columns)
    }
}

/// Row denotes a set of cells on given rows on a [Table].
///
/// [Table]: crate::Table
pub struct Rows<R> {
    range: R,
}

impl<R> Rows<R>
where
    R: RangeBounds<usize>,
{
    /// Returns a new instance of [Rows] for a range of rows.
    ///
    /// If the boundaries are exeeded it may panic.
    pub fn new(range: R) -> Self {
        Self { range }
    }
}

impl Rows<()> {
    /// Returns a new instance of [Rows] with a single row.
    ///
    /// If the boundaries are exeeded it may panic.
    pub fn single(index: usize) -> Row {
        Row { index }
    }

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
    type Iter = RowsIter;

    fn cells(&self, count_rows: usize, count_columns: usize) -> Self::Iter {
        let (x, y) = bounds_to_usize(self.range.start_bound(), self.range.end_bound(), count_rows);

        RowsIter::new(x, y, count_columns)
    }
}

/// Column denotes a set of cells on given columns on a [Table].
///
/// [Table]: crate::Table
pub struct Columns<R> {
    range: R,
}

impl<R> Columns<R>
where
    R: RangeBounds<usize>,
{
    /// Returns a new instance of [Columns] for a range of columns.
    ///
    /// If the boundaries are exeeded it may panic.
    pub fn new(range: R) -> Self {
        Self { range }
    }
}

impl Columns<()> {
    /// Returns a new instance of [Columns] for a single column.
    ///
    /// If the boundaries are exeeded it may panic.
    pub fn single(index: usize) -> Column {
        Column(index)
    }

    /// Returns a new instance of [Columns] for a first column.
    ///
    /// If the boundaries are exeeded the object will produce no cells.
    pub fn first() -> FirstColumn {
        FirstColumn
    }

    /// Returns a new instance of [Columns] for a last column.
    ///
    /// If the boundaries are exeeded the object will produce no cells.
    pub fn last() -> LastColumn {
        LastColumn
    }
}

impl<R> Object for Columns<R>
where
    R: RangeBounds<usize>,
{
    type Iter = ColumnsIter;

    fn cells(&self, count_rows: usize, count_columns: usize) -> Self::Iter {
        let (x, y) = bounds_to_usize(
            self.range.start_bound(),
            self.range.end_bound(),
            count_columns,
        );

        ColumnsIter::new(x, y, count_rows)
    }
}

/// FirstColumn represents the first column on a grid.
pub struct FirstColumn;

impl Object for FirstColumn {
    type Iter = ColumnIter;

    fn cells(&self, count_rows: usize, _: usize) -> Self::Iter {
        ColumnIter::new(0, count_rows)
    }
}

impl Add<usize> for FirstColumn {
    type Output = Column;

    fn add(self, rhs: usize) -> Self::Output {
        Column(rhs)
    }
}

/// LastColumn represents the last column on a grid.
pub struct LastColumn;

impl Object for LastColumn {
    type Iter = ColumnIter;

    fn cells(&self, count_rows: usize, count_columns: usize) -> Self::Iter {
        let col = count_columns.saturating_sub(1);
        ColumnIter::new(col, count_rows)
    }
}

impl Sub<usize> for LastColumn {
    type Output = LastColumnOffset;

    fn sub(self, rhs: usize) -> Self::Output {
        LastColumnOffset { offset: rhs }
    }
}

/// Column represents a single column on a grid.
pub struct Column(usize);

impl Object for Column {
    type Iter = ColumnIter;

    fn cells(&self, count_rows: usize, count_columns: usize) -> Self::Iter {
        let col = self.0;
        if col >= count_columns {
            return ColumnIter::new(0, 0);
        }

        ColumnIter::new(col, count_rows)
    }
}

/// LastColumnOffset represents a single column on a grid indexed via offset from the last column.
pub struct LastColumnOffset {
    offset: usize,
}

impl Object for LastColumnOffset {
    type Iter = ColumnIter;

    fn cells(&self, count_rows: usize, count_columns: usize) -> Self::Iter {
        let col = count_columns.saturating_sub(1);
        if self.offset > col {
            return ColumnIter::new(0, 0);
        }

        let col = col - self.offset;
        ColumnIter::new(col, count_rows)
    }
}

/// Cell denotes a particular cell on a [Table].
///
/// [Table]: crate::Table
pub struct Cell(pub usize, pub usize);

impl Object for Cell {
    type Iter = CellIter;

    fn cells(&self, _: usize, _: usize) -> Self::Iter {
        CellIter::new(self.0, self.1)
    }
}

/// Combination struct used for chaining [Object]'s.
///
/// Combines 2 sets of cells into one.
///
/// Duplicates are removed from the output set.
pub struct UntionCombination<L, R> {
    lhs: L,
    rhs: R,
}

impl<L, R> Object for UntionCombination<L, R>
where
    L: Object,
    R: Object,
{
    type Iter = UnionIter<L::Iter, R::Iter>;

    fn cells(&self, count_rows: usize, count_columns: usize) -> Self::Iter {
        let lhs = self.lhs.cells(count_rows, count_columns);
        let rhs = self.rhs.cells(count_rows, count_columns);

        UnionIter::new(lhs, rhs)
    }
}

/// Difference struct used for chaining [Object]'s.
///
/// Returns cells from 1st set with removed ones from the 2nd set.
pub struct DiffCombination<L, R> {
    lhs: L,
    rhs: R,
}

impl<L, R> Object for DiffCombination<L, R>
where
    L: Object,
    R: Object,
{
    type Iter = DiffIter<L::Iter>;

    fn cells(&self, count_rows: usize, count_columns: usize) -> Self::Iter {
        let lhs = self.lhs.cells(count_rows, count_columns);
        let rhs = self.rhs.cells(count_rows, count_columns);

        DiffIter::new(lhs, rhs)
    }
}

pub struct SectorIter {
    rows_end: usize,
    cols_start: usize,
    cols_end: usize,
    row: usize,
    col: usize,
}

impl SectorIter {
    pub const fn new(
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

impl Iterator for SectorIter {
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

pub struct FrameIter {
    rows: usize,
    cols: usize,
    row: usize,
    col: usize,
}

impl FrameIter {
    pub const fn new(count_rows: usize, count_columns: usize) -> Self {
        Self {
            rows: count_rows,
            cols: count_columns,
            row: 0,
            col: 0,
        }
    }
}

impl Iterator for FrameIter {
    type Item = (usize, usize);

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

        Some((row, col))
    }
}

pub struct RowsIter {
    rows: usize,
    cols: usize,
    col: usize,
    row: usize,
}

impl RowsIter {
    pub const fn new(rows_start: usize, rows_end: usize, count_columns: usize) -> Self {
        Self {
            rows: rows_end,
            cols: count_columns,
            row: rows_start,
            col: 0,
        }
    }
}

impl Iterator for RowsIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
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

        Some((row, col))
    }
}

pub struct RowIter {
    cols: usize,
    col: usize,
    row: usize,
}

impl RowIter {
    pub const fn new(row: usize, count_columns: usize) -> Self {
        Self {
            cols: count_columns,
            row,
            col: 0,
        }
    }
}

impl Iterator for RowIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.col == self.cols {
            return None;
        }

        let col = self.col;

        self.col += 1;

        Some((self.row, col))
    }
}

pub struct ColumnsIter {
    rows: usize,
    cols: usize,
    col: usize,
    row: usize,
}

impl ColumnsIter {
    pub const fn new(cols_start: usize, cols_end: usize, count_rows: usize) -> Self {
        Self {
            rows: count_rows,
            cols: cols_end,
            row: 0,
            col: cols_start,
        }
    }
}

impl Iterator for ColumnsIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.col == self.cols {
            return None;
        }

        let row = self.row;
        let col = self.col;

        self.row += 1;

        if self.row == self.rows {
            self.col += 1;
            self.row = 0;
        }

        Some((row, col))
    }
}

pub struct ColumnIter {
    rows: usize,
    col: usize,
    row: usize,
}

impl ColumnIter {
    pub const fn new(col: usize, count_rows: usize) -> Self {
        Self {
            rows: count_rows,
            row: 0,
            col,
        }
    }
}

impl Iterator for ColumnIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row == self.rows {
            return None;
        }

        let row = self.row;
        let col = self.col;

        self.row += 1;

        Some((row, col))
    }
}

pub struct CellIter {
    cell: Option<(usize, usize)>,
}

impl CellIter {
    fn new(row: usize, col: usize) -> Self {
        Self {
            cell: Some((row, col)),
        }
    }
}

impl Iterator for CellIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.cell.take()
    }
}

pub struct UnionIter<L, R> {
    lhs: L,
    rhs: R,
    seen: HashSet<(usize, usize)>,
}

impl<L, R> UnionIter<L, R>
where
    L: Iterator<Item = (usize, usize)>,
    R: Iterator<Item = (usize, usize)>,
{
    fn new(lhs: L, rhs: R) -> Self {
        let size = match lhs.size_hint() {
            (s1, Some(s2)) if s1 == s2 => s1,
            _ => 0,
        };

        Self {
            lhs,
            rhs,
            seen: HashSet::with_capacity(size),
        }
    }
}

impl<L, R> Iterator for UnionIter<L, R>
where
    L: Iterator<Item = (usize, usize)>,
    R: Iterator<Item = (usize, usize)>,
{
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(p) = self.lhs.next() {
            self.seen.insert(p);
            return Some(p);
        }

        for p in self.rhs.by_ref() {
            if !self.seen.contains(&p) {
                self.seen.insert(p);
                return Some(p);
            }
        }

        None
    }
}

pub struct DiffIter<L> {
    lhs: L,
    seen: HashSet<(usize, usize)>,
}

impl<L> DiffIter<L>
where
    L: Iterator<Item = (usize, usize)>,
{
    fn new<R>(lhs: L, rhs: R) -> Self
    where
        R: Iterator<Item = (usize, usize)>,
    {
        let size = match rhs.size_hint() {
            (s1, Some(s2)) if s1 == s2 => s1,
            _ => 0,
        };

        let mut seen = HashSet::with_capacity(size);
        for p in rhs {
            seen.insert(p);
        }

        Self { lhs, seen }
    }
}

impl<L> Iterator for DiffIter<L>
where
    L: Iterator<Item = (usize, usize)>,
{
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        for p in self.lhs.by_ref() {
            if !self.seen.contains(&p) {
                return Some(p);
            }
        }

        None
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_column_test() {
        assert_eq!((Columns::first()).cells(0, 0).collect::<Vec<_>>(), vec![]);
        // is this correct?
        assert_eq!(
            (Columns::first()).cells(10, 0).collect::<Vec<_>>(),
            vec![
                (0, 0),
                (1, 0),
                (2, 0),
                (3, 0),
                (4, 0),
                (5, 0),
                (6, 0),
                (7, 0),
                (8, 0),
                (9, 0)
            ]
        );
        assert_eq!((Columns::first()).cells(0, 10).collect::<Vec<_>>(), vec![]);
        assert_eq!(
            (Columns::first()).cells(5, 2).collect::<Vec<_>>(),
            vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)]
        );
    }

    #[test]
    fn last_column_test() {
        assert_eq!((Columns::last()).cells(0, 0).collect::<Vec<_>>(), vec![]);
        // is this correct?
        assert_eq!(
            (Columns::last()).cells(10, 0).collect::<Vec<_>>(),
            vec![
                (0, 0),
                (1, 0),
                (2, 0),
                (3, 0),
                (4, 0),
                (5, 0),
                (6, 0),
                (7, 0),
                (8, 0),
                (9, 0)
            ]
        );
        assert_eq!((Columns::last()).cells(0, 10).collect::<Vec<_>>(), vec![]);
        assert_eq!(
            (Columns::last()).cells(5, 2).collect::<Vec<_>>(),
            vec![(0, 1), (1, 1), (2, 1), (3, 1), (4, 1)]
        );
    }

    #[test]
    fn last_column_sub_test() {
        assert_eq!(
            (Columns::last()).cells(5, 2).collect::<Vec<_>>(),
            vec![(0, 1), (1, 1), (2, 1), (3, 1), (4, 1)]
        );
        assert_eq!(
            (Columns::last() - 0).cells(5, 2).collect::<Vec<_>>(),
            vec![(0, 1), (1, 1), (2, 1), (3, 1), (4, 1)]
        );
        assert_eq!(
            (Columns::last() - 1).cells(5, 2).collect::<Vec<_>>(),
            vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)]
        );
        assert_eq!(
            (Columns::last() - 2).cells(5, 2).collect::<Vec<_>>(),
            vec![]
        );
        assert_eq!(
            (Columns::last() - 100).cells(5, 2).collect::<Vec<_>>(),
            vec![]
        );
    }

    #[test]
    fn first_column_sub_test() {
        assert_eq!(
            (Columns::first()).cells(5, 2).collect::<Vec<_>>(),
            [(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)]
        );
        assert_eq!(
            (Columns::first() + 0).cells(5, 2).collect::<Vec<_>>(),
            [(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)]
        );
        assert_eq!(
            (Columns::first() + 1).cells(5, 2).collect::<Vec<_>>(),
            [(0, 1), (1, 1), (2, 1), (3, 1), (4, 1)]
        );
        assert_eq!((Columns::first() + 2).cells(5, 2).collect::<Vec<_>>(), []);
        assert_eq!((Columns::first() + 100).cells(5, 2).collect::<Vec<_>>(), []);
    }

    #[test]
    fn last_row_sub_test() {
        assert_eq!(
            (Rows::last()).cells(5, 2).collect::<Vec<_>>(),
            vec![(4, 0), (4, 1)]
        );
        assert_eq!(
            (Rows::last() - 0).cells(5, 2).collect::<Vec<_>>(),
            vec![(4, 0), (4, 1)]
        );
        assert_eq!(
            (Rows::last() - 1).cells(5, 2).collect::<Vec<_>>(),
            vec![(3, 0), (3, 1)]
        );
        assert_eq!(
            (Rows::last() - 2).cells(5, 2).collect::<Vec<_>>(),
            vec![(2, 0), (2, 1)]
        );
        assert_eq!(
            (Rows::last() - 3).cells(5, 2).collect::<Vec<_>>(),
            vec![(1, 0), (1, 1)]
        );
        assert_eq!(
            (Rows::last() - 4).cells(5, 2).collect::<Vec<_>>(),
            vec![(0, 0), (0, 1)]
        );
        assert_eq!((Rows::last() - 5).cells(5, 2).collect::<Vec<_>>(), vec![]);
        assert_eq!((Rows::last() - 100).cells(5, 2).collect::<Vec<_>>(), vec![]);
    }

    #[test]
    fn first_row_sub_test() {
        assert_eq!(
            (Rows::first()).cells(5, 2).collect::<Vec<_>>(),
            vec![(0, 0), (0, 1)]
        );
        assert_eq!(
            (Rows::first() + 0).cells(5, 2).collect::<Vec<_>>(),
            vec![(0, 0), (0, 1)]
        );
        assert_eq!(
            (Rows::first() + 1).cells(5, 2).collect::<Vec<_>>(),
            vec![(1, 0), (1, 1)]
        );
        assert_eq!(
            (Rows::first() + 2).cells(5, 2).collect::<Vec<_>>(),
            vec![(2, 0), (2, 1)]
        );
        assert_eq!(
            (Rows::first() + 3).cells(5, 2).collect::<Vec<_>>(),
            vec![(3, 0), (3, 1)]
        );
        assert_eq!(
            (Rows::first() + 4).cells(5, 2).collect::<Vec<_>>(),
            vec![(4, 0), (4, 1)]
        );
        assert_eq!((Rows::first() + 5).cells(5, 2).collect::<Vec<_>>(), vec![]);
        assert_eq!(
            (Rows::first() + 100).cells(5, 2).collect::<Vec<_>>(),
            vec![]
        );
    }

    #[test]
    fn rows_test() {
        assert_eq!(
            (Rows::new(..)).cells(2, 3).collect::<Vec<_>>(),
            vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2)]
        );
        assert_eq!(
            (Rows::new(1..)).cells(2, 3).collect::<Vec<_>>(),
            vec![(1, 0), (1, 1), (1, 2)]
        );
        assert_eq!((Rows::new(2..)).cells(2, 3).collect::<Vec<_>>(), vec![]);
        assert_eq!(
            (Rows::new(1..2)).cells(2, 3).collect::<Vec<_>>(),
            vec![(1, 0), (1, 1), (1, 2)]
        );
    }

    #[test]
    fn columns_test() {
        assert_eq!(
            (Columns::new(..)).cells(2, 3).collect::<Vec<_>>(),
            vec![(0, 0), (1, 0), (0, 1), (1, 1), (0, 2), (1, 2)]
        );
        assert_eq!(
            (Columns::new(1..)).cells(2, 3).collect::<Vec<_>>(),
            vec![(0, 1), (1, 1), (0, 2), (1, 2)]
        );
        assert_eq!(
            (Columns::new(2..)).cells(2, 3).collect::<Vec<_>>(),
            vec![(0, 2), (1, 2)]
        );
        assert_eq!((Columns::new(3..)).cells(2, 3).collect::<Vec<_>>(), vec![]);
        assert_eq!(
            (Columns::new(1..2)).cells(2, 3).collect::<Vec<_>>(),
            vec![(0, 1), (1, 1)]
        );
    }

    #[test]
    fn frame_test() {
        assert_eq!(
            Frame.cells(2, 3).collect::<Vec<_>>(),
            vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2)]
        );
        assert_eq!(Frame.cells(0, 0).collect::<Vec<_>>(), vec![]);
        assert_eq!(Frame.cells(2, 0).collect::<Vec<_>>(), vec![]);
        assert_eq!(Frame.cells(0, 2).collect::<Vec<_>>(), vec![]);
    }

    #[test]
    fn segment_test() {
        assert_eq!(
            (Segment::new(.., ..)).cells(2, 3).collect::<Vec<_>>(),
            vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2)]
        );
        assert_eq!(
            (Segment::new(1.., ..)).cells(2, 3).collect::<Vec<_>>(),
            vec![(1, 0), (1, 1), (1, 2)]
        );
        assert_eq!(
            (Segment::new(2.., ..)).cells(2, 3).collect::<Vec<_>>(),
            vec![]
        );

        assert_eq!(
            (Segment::new(.., 1..)).cells(2, 3).collect::<Vec<_>>(),
            vec![(0, 1), (0, 2), (1, 1), (1, 2)]
        );
        assert_eq!(
            (Segment::new(.., 2..)).cells(2, 3).collect::<Vec<_>>(),
            vec![(0, 2), (1, 2)]
        );
        assert_eq!(
            (Segment::new(.., 3..)).cells(2, 3).collect::<Vec<_>>(),
            vec![]
        );

        assert_eq!(
            (Segment::new(1.., 1..)).cells(2, 3).collect::<Vec<_>>(),
            vec![(1, 1), (1, 2)]
        );
        assert_eq!(
            (Segment::new(1..2, 1..2)).cells(2, 3).collect::<Vec<_>>(),
            vec![(1, 1)]
        );

        assert_eq!(
            (Segment::new(5.., 5..)).cells(2, 3).collect::<Vec<_>>(),
            vec![]
        );
    }

    #[test]
    fn object_and_test() {
        assert_eq!(
            Cell(0, 0).and(Cell(0, 0)).cells(2, 3).collect::<Vec<_>>(),
            vec![(0, 0)]
        );
        assert_eq!(
            Cell(0, 0).and(Cell(1, 2)).cells(2, 3).collect::<Vec<_>>(),
            vec![(0, 0), (1, 2)]
        );
        assert_eq!(
            Cell(0, 0).and(Cell(1, 2)).cells(0, 0).collect::<Vec<_>>(),
            vec![(0, 0), (1, 2)]
        );
    }

    #[test]
    fn object_not_test() {
        assert_eq!(
            Cell(0, 0).not(Cell(0, 0)).cells(2, 3).collect::<Vec<_>>(),
            vec![]
        );
        assert_eq!(
            Rows::first()
                .not(Cell(0, 0))
                .cells(2, 3)
                .collect::<Vec<_>>(),
            vec![(0, 1), (0, 2)]
        );
        assert_eq!(
            Rows::first()
                .not(Cell(0, 0))
                .cells(0, 0)
                .collect::<Vec<_>>(),
            vec![]
        );
    }
}
