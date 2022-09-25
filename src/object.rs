//! This module contains a list of primitives that implement a [`Object`] trait.
//! They help to locate a necessary segment on a [`Table`].
//!
//! [`Table`]: crate::Table

use std::{
    collections::HashSet,
    ops::{Add, Bound, RangeBounds, RangeFull, Sub},
};

use papergrid::records::Records;
pub use papergrid::{Entity, EntityIterator};

use crate::Table;

/// Object helps to locate a necessary part of a [`Table`].
///
/// [`Table`]: crate::Table
pub trait Object: Sized {
    /// An [`Iterator`] which returns a list of cells.
    type Iter: Iterator<Item = Entity>;

    /// Cells returns a set of coordinates of cells
    fn cells<R>(&self, table: &Table<R>) -> Self::Iter
    where
        R: Records;

    /// Combines cells.
    /// It doesn't repeat cells.
    fn and<O>(self, rhs: O) -> UnionCombination<Self, O>
    where
        O: Object,
    {
        UnionCombination { lhs: self, rhs }
    }

    /// Excludes rhs cells from this cells.
    fn not<O>(self, rhs: O) -> DiffCombination<Self, O>
    where
        O: Object,
    {
        DiffCombination { lhs: self, rhs }
    }

    /// Returns cells which are present in both [`Object`]s only.
    fn intersect<O>(self, rhs: O) -> IntersectionCombination<Self, O>
    where
        O: Object,
    {
        IntersectionCombination { lhs: self, rhs }
    }

    /// Returns cells which are not present in target [`Object`].
    fn inverse(self) -> InversionCombination<Self> {
        InversionCombination { obj: self }
    }
}

/// Combination struct used for chaining [`Object`]'s.
///
/// Combines 2 sets of cells into one.
///
/// Duplicates are removed from the output set.
#[derive(Debug)]
pub struct UnionCombination<L, R> {
    lhs: L,
    rhs: R,
}

impl<L, R> Object for UnionCombination<L, R>
where
    L: Object,
    R: Object,
{
    type Iter = UnionIter<L::Iter, R::Iter>;

    fn cells<T>(&self, table: &Table<T>) -> Self::Iter
    where
        T: Records,
    {
        let lhs = self.lhs.cells(table);
        let rhs = self.rhs.cells(table);

        UnionIter::new(lhs, rhs, table.count_rows(), table.count_columns())
    }
}

/// Difference struct used for chaining [`Object`]'s.
///
/// Returns cells from 1st set with removed ones from the 2nd set.
#[derive(Debug)]
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

    fn cells<T>(&self, table: &Table<T>) -> Self::Iter
    where
        T: Records,
    {
        let lhs = self.lhs.cells(table);
        let rhs = self.rhs.cells(table);

        DiffIter::new(lhs, rhs, table.count_rows(), table.count_columns())
    }
}

/// Intersection struct used for chaining [`Object`]'s.
///
/// Returns cells which are present in 2 sets.
/// But not in one of them
#[derive(Debug)]
pub struct IntersectionCombination<L, R> {
    lhs: L,
    rhs: R,
}

impl<L, R> Object for IntersectionCombination<L, R>
where
    L: Object,
    R: Object,
{
    type Iter = IntersectIter<L::Iter>;
    fn cells<T>(&self, table: &Table<T>) -> Self::Iter
    where
        T: Records,
    {
        let lhs = self.lhs.cells(table);
        let rhs = self.rhs.cells(table);

        IntersectIter::new(lhs, rhs, table.count_rows(), table.count_columns())
    }
}

/// Inversion struct used for chaining [`Object`]'s.
///
/// Returns cells which are present in 2 sets.
/// But not in one of them
#[derive(Debug)]
pub struct InversionCombination<O> {
    obj: O,
}

impl<O> Object for InversionCombination<O>
where
    O: Object,
{
    type Iter = InversionIter;

    fn cells<T>(&self, table: &Table<T>) -> Self::Iter
    where
        T: Records,
    {
        let obj = self.obj.cells(table);

        InversionIter::new(obj, table.count_rows(), table.count_columns())
    }
}

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

impl<C, R> Object for Segment<C, R>
where
    C: RangeBounds<usize>,
    R: RangeBounds<usize>,
{
    type Iter = SectorIter;

    fn cells<T>(&self, table: &Table<T>) -> Self::Iter
    where
        T: Records,
    {
        let (rows_start, rows_end) = bounds_to_usize(
            self.rows.start_bound(),
            self.rows.end_bound(),
            table.count_rows(),
        );

        let (cols_start, cols_end) = bounds_to_usize(
            self.columns.start_bound(),
            self.columns.end_bound(),
            table.count_columns(),
        );

        SectorIter::new(rows_start, rows_end, cols_start, cols_end)
    }
}

/// This is a segment which cantains all cells on the table.
///
/// Can be crated from [`Segment::all`].
#[derive(Debug)]
pub struct SegmentAll;

impl Object for SegmentAll {
    type Iter = EntityOnce;
    fn cells<T>(&self, _: &Table<T>) -> Self::Iter
    where
        T: Records,
    {
        EntityOnce::new(Some(Entity::Global))
    }
}

/// Frame includes cells which are on the edges of each side.
/// Therefore it's [`Object`] implementation returns a subset of cells which are present in frame.
#[derive(Debug)]
pub struct Frame;

impl Object for Frame {
    type Iter = FrameIter;

    fn cells<T>(&self, table: &Table<T>) -> Self::Iter
    where
        T: Records,
    {
        FrameIter::new(table.count_rows(), table.count_columns())
    }
}

/// This structure represents the first row of a [`Table`].
/// It's often contains headers data.
///
/// [`Table`]: crate::Table
#[derive(Debug)]
pub struct FirstRow;

impl Object for FirstRow {
    type Iter = EntityOnce;

    fn cells<T>(&self, table: &Table<T>) -> Self::Iter
    where
        T: Records,
    {
        if table.is_empty() {
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

impl Object for LastRow {
    type Iter = EntityOnce;

    fn cells<T>(&self, table: &Table<T>) -> Self::Iter
    where
        T: Records,
    {
        if table.is_empty() {
            return EntityOnce::new(None);
        }

        let count_rows = table.count_rows();
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

/// A row which is located by an offset from the first row.
#[derive(Debug, Clone, Copy)]
pub struct Row {
    index: usize,
}

impl Object for Row {
    type Iter = EntityOnce;

    fn cells<T>(&self, table: &Table<T>) -> Self::Iter
    where
        T: Records,
    {
        if table.is_empty() {
            return EntityOnce::new(None);
        }

        if self.index >= table.count_rows() {
            return EntityOnce::new(None);
        }

        EntityOnce::new(Some(Entity::Row(self.index)))
    }
}

impl From<Row> for usize {
    fn from(val: Row) -> Self {
        val.index
    }
}

/// A row which is located by an offset from the last row.
#[derive(Debug)]
pub struct LastRowOffset {
    offset: usize,
}

impl Object for LastRowOffset {
    type Iter = EntityOnce;

    fn cells<T>(&self, table: &Table<T>) -> Self::Iter
    where
        T: Records,
    {
        if table.is_empty() {
            return EntityOnce::new(None);
        }

        let count_rows = table.count_rows();
        let row = if count_rows == 0 { 0 } else { count_rows - 1 };
        if self.offset > row {
            return EntityOnce::new(None);
        }

        let row = row - self.offset;
        EntityOnce::new(Some(Entity::Row(row)))
    }
}

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

    pub(crate) fn get_range(&self) -> &R {
        &self.range
    }
}

impl Rows<()> {
    /// Returns a new instance of [`Rows`] with a single row.
    ///
    /// If the boundaries are exceeded it may panic.
    pub fn single(index: usize) -> Row {
        Row { index }
    }

    /// Returns a first row [`Object`].
    ///
    /// If the table has 0 rows returns an empty set of cells.
    pub fn first() -> FirstRow {
        FirstRow
    }

    /// Returns a last row [`Object`].
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

    fn cells<T>(&self, table: &Table<T>) -> Self::Iter
    where
        T: Records,
    {
        let (x, y) = bounds_to_usize(
            self.range.start_bound(),
            self.range.end_bound(),
            table.count_rows(),
        );

        RowsIter::new(x, y)
    }
}

/// Column denotes a set of cells on given columns on a [`Table`].
///
/// [`Table`]: crate::Table
#[derive(Debug)]
pub struct Columns<R> {
    range: R,
}

impl<R> Columns<R>
where
    R: RangeBounds<usize>,
{
    /// Returns a new instance of [`Columns`] for a range of columns.
    ///
    /// If the boundaries are exceeded it may panic.
    pub fn new(range: R) -> Self {
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
    pub fn single(index: usize) -> Column {
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

impl<R> Object for Columns<R>
where
    R: RangeBounds<usize>,
{
    type Iter = ColumnsIter;

    fn cells<T>(&self, table: &Table<T>) -> Self::Iter
    where
        T: Records,
    {
        let (x, y) = bounds_to_usize(
            self.range.start_bound(),
            self.range.end_bound(),
            table.count_columns(),
        );
        ColumnsIter::new(x, y)
    }
}

/// `FirstColumn` represents the first column on a grid.
#[derive(Debug)]
pub struct FirstColumn;

impl Object for FirstColumn {
    type Iter = EntityOnce;

    fn cells<T>(&self, table: &Table<T>) -> Self::Iter
    where
        T: Records,
    {
        if table.is_empty() {
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
#[derive(Debug)]
pub struct LastColumn;

impl Object for LastColumn {
    type Iter = EntityOnce;

    fn cells<T>(&self, table: &Table<T>) -> Self::Iter
    where
        T: Records,
    {
        if table.is_empty() {
            return EntityOnce::new(None);
        }

        let col = table.count_columns().saturating_sub(1);
        EntityOnce::new(Some(Entity::Column(col)))
    }
}

impl Sub<usize> for LastColumn {
    type Output = LastColumnOffset;

    fn sub(self, rhs: usize) -> Self::Output {
        LastColumnOffset { offset: rhs }
    }
}

/// Column represents a single column on a grid.
#[derive(Debug, Clone, Copy)]
pub struct Column(usize);

impl Object for Column {
    type Iter = EntityOnce;

    fn cells<T>(&self, table: &Table<T>) -> Self::Iter
    where
        T: Records,
    {
        if table.is_empty() {
            return EntityOnce::new(None);
        }

        let col = self.0;
        if col >= table.count_columns() {
            return EntityOnce::new(None);
        }

        EntityOnce::new(Some(Entity::Column(col)))
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
#[derive(Debug)]
pub struct LastColumnOffset {
    offset: usize,
}

impl Object for LastColumnOffset {
    type Iter = EntityOnce;

    fn cells<T>(&self, table: &Table<T>) -> Self::Iter
    where
        T: Records,
    {
        if table.is_empty() {
            return EntityOnce::new(None);
        }

        let col = table.count_columns().saturating_sub(1);
        if self.offset > col {
            return EntityOnce::new(None);
        }

        let col = col - self.offset;
        EntityOnce::new(Some(Entity::Column(col)))
    }
}

/// Cell denotes a particular cell on a [`Table`].
///
/// [`Table`]: crate::Table
#[derive(Debug)]
pub struct Cell(pub usize, pub usize);

impl Object for Cell {
    type Iter = EntityOnce;

    fn cells<T>(&self, _: &Table<T>) -> Self::Iter
    where
        T: Records,
    {
        EntityOnce::new(Some(Entity::Cell(self.0, self.1)))
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
struct SectorCellsIter {
    rows_end: usize,
    cols_start: usize,
    cols_end: usize,
    row: usize,
    col: usize,
}

impl SectorCellsIter {
    const fn new(rows_start: usize, rows_end: usize, cols_start: usize, cols_end: usize) -> Self {
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
        if self.start == self.end {
            return None;
        }

        let col = self.start;
        self.start += 1;

        Some(Entity::Row(col))
    }
}

/// An [`Iterator`] which goes goes over columns of a [`Table`].
///
/// [`Table`]: crate::Table
#[derive(Debug)]
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
        if self.start == self.end {
            return None;
        }

        let col = self.start;
        self.start += 1;

        Some(Entity::Column(col))
    }
}

/// An [`Iterator`] which returns an entity once.
#[derive(Debug)]
pub struct EntityOnce {
    entity: Option<Entity>,
}

impl EntityOnce {
    const fn new(entity: Option<Entity>) -> Self {
        Self { entity }
    }
}

impl Iterator for EntityOnce {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        self.entity.take()
    }
}

/// An [`Iterator`] which goes over a combination [`Object::Iter`].
#[derive(Debug)]
pub struct UnionIter<L, R> {
    lhs: Option<L>,
    rhs: R,
    seen: HashSet<(usize, usize)>,
    current: Option<EntityIterator>,
    count_rows: usize,
    count_cols: usize,
}

impl<L, R> UnionIter<L, R>
where
    L: Iterator<Item = Entity>,
    R: Iterator<Item = Entity>,
{
    fn new(lhs: L, rhs: R, count_rows: usize, count_cols: usize) -> Self {
        let size = match lhs.size_hint() {
            (s1, Some(s2)) if s1 == s2 => s1,
            _ => 0,
        };

        Self {
            lhs: Some(lhs),
            rhs,
            seen: HashSet::with_capacity(size),
            current: None,
            count_rows,
            count_cols,
        }
    }
}

impl<L, R> Iterator for UnionIter<L, R>
where
    L: Iterator<Item = Entity>,
    R: Iterator<Item = Entity>,
{
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(iter) = self.current.as_mut() {
            for p in iter.by_ref() {
                if self.lhs.is_none() && self.seen.contains(&p) {
                    continue;
                }

                self.seen.insert(p);
                return Some(Entity::Cell(p.0, p.1));
            }
        }

        if let Some(lhs) = self.lhs.as_mut() {
            for entity in lhs.by_ref() {
                let mut iter = entity.iter(self.count_rows, self.count_cols);
                if let Some(p) = iter.by_ref().next() {
                    self.seen.insert(p);
                    self.current = Some(iter);
                    return Some(Entity::Cell(p.0, p.1));
                }
            }

            self.lhs = None;
        }

        for entity in self.rhs.by_ref() {
            let mut iter = entity.iter(self.count_rows, self.count_cols);

            for p in iter.by_ref() {
                if !self.seen.contains(&p) {
                    self.seen.insert(p);
                    self.current = Some(iter);
                    return Some(Entity::Cell(p.0, p.1));
                }
            }
        }

        None
    }
}

/// An [`Iterator`] which goes over only cells which are present in first [`Object::Iter`] but not second.
#[derive(Debug)]
pub struct DiffIter<L> {
    lhs: L,
    seen: HashSet<(usize, usize)>,
    count_rows: usize,
    count_cols: usize,
    current: Option<EntityIterator>,
}

impl<L> DiffIter<L>
where
    L: Iterator<Item = Entity>,
{
    fn new<R>(lhs: L, rhs: R, count_rows: usize, count_cols: usize) -> Self
    where
        R: Iterator<Item = Entity>,
    {
        let size = match rhs.size_hint() {
            (s1, Some(s2)) if s1 == s2 => s1,
            _ => 0,
        };

        let mut seen = HashSet::with_capacity(size);
        for entity in rhs {
            seen.extend(entity.iter(count_rows, count_cols));
        }

        Self {
            lhs,
            seen,
            count_rows,
            count_cols,
            current: None,
        }
    }
}

impl<L> Iterator for DiffIter<L>
where
    L: Iterator<Item = Entity>,
{
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(iter) = self.current.as_mut() {
            for p in iter.by_ref() {
                if !self.seen.contains(&p) {
                    return Some(Entity::Cell(p.0, p.1));
                }
            }
        }

        for entity in self.lhs.by_ref() {
            let mut iter = entity.iter(self.count_rows, self.count_cols);

            for p in iter.by_ref() {
                if !self.seen.contains(&p) {
                    self.current = Some(iter);
                    return Some(Entity::Cell(p.0, p.1));
                }
            }
        }

        None
    }
}

/// An [`Iterator`] which goes goes over cells which are present in both [`Object::Iter`]ators.
#[derive(Debug)]
pub struct IntersectIter<L> {
    lhs: L,
    seen: HashSet<(usize, usize)>,
    count_rows: usize,
    count_cols: usize,
    current: Option<EntityIterator>,
}

impl<L> IntersectIter<L>
where
    L: Iterator<Item = Entity>,
{
    fn new<R>(lhs: L, rhs: R, count_rows: usize, count_cols: usize) -> Self
    where
        R: Iterator<Item = Entity>,
    {
        let size = match rhs.size_hint() {
            (s1, Some(s2)) if s1 == s2 => s1,
            _ => 0,
        };

        let mut seen = HashSet::with_capacity(size);
        for entity in rhs {
            seen.extend(entity.iter(count_rows, count_cols));
        }

        Self {
            lhs,
            seen,
            count_rows,
            count_cols,
            current: None,
        }
    }
}

impl<L> Iterator for IntersectIter<L>
where
    L: Iterator<Item = Entity>,
{
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(iter) = self.current.as_mut() {
            for p in iter.by_ref() {
                if self.seen.contains(&p) {
                    return Some(Entity::Cell(p.0, p.1));
                }
            }
        }

        for entity in self.lhs.by_ref() {
            let mut iter = entity.iter(self.count_rows, self.count_cols);

            for p in iter.by_ref() {
                if self.seen.contains(&p) {
                    self.current = Some(iter);
                    return Some(Entity::Cell(p.0, p.1));
                }
            }
        }

        None
    }
}

/// An [`Iterator`] which goes goes over cells which are not present an [`Object::Iter`]ator.
#[derive(Debug)]
pub struct InversionIter {
    all: SectorCellsIter,
    seen: HashSet<(usize, usize)>,
}

impl InversionIter {
    fn new<O>(obj: O, count_rows: usize, count_columns: usize) -> Self
    where
        O: Iterator<Item = Entity>,
    {
        let size = match obj.size_hint() {
            (s1, Some(s2)) if s1 == s2 => s1,
            _ => 0,
        };

        let mut seen = HashSet::with_capacity(size);
        for entity in obj {
            seen.extend(entity.iter(count_rows, count_columns));
        }

        let all = SectorCellsIter::new(0, count_rows, 0, count_columns);

        Self { all, seen }
    }
}

impl Iterator for InversionIter {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        for p in self.all.by_ref() {
            if !self.seen.contains(&p) {
                return Some(Entity::Cell(p.0, p.1));
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
    fn columns_test() {
        assert_eq!(
            vec_cells(Columns::new(..), 2, 3),
            [Entity::Column(0), Entity::Column(1), Entity::Column(2)]
        );
        assert_eq!(
            vec_cells(Columns::new(1..), 2, 3),
            [Entity::Column(1), Entity::Column(2)]
        );
        assert_eq!(vec_cells(Columns::new(2..), 2, 3), [Entity::Column(2)]);
        assert_eq!(vec_cells(Columns::new(3..), 2, 3), []);
        assert_eq!(vec_cells(Columns::new(0..1), 2, 3), [Entity::Column(0)]);
        assert_eq!(vec_cells(Columns::new(1..2), 2, 3), [Entity::Column(1)]);
        assert_eq!(vec_cells(Columns::new(2..3), 2, 3), [Entity::Column(2)]);
        assert_eq!(vec_cells(Columns::new(..), 0, 0), []);
        assert_eq!(vec_cells(Columns::new(..), 2, 0), []);
        assert_eq!(vec_cells(Columns::new(..), 0, 3), []);
    }

    #[test]
    fn first_column_test() {
        assert_eq!(vec_cells(Columns::first(), 5, 2), [Entity::Column(0)]);
        assert_eq!(vec_cells(Columns::first(), 0, 0), []);
        assert_eq!(vec_cells(Columns::first(), 10, 0), []);
        assert_eq!(vec_cells(Columns::first(), 0, 10), []);
    }

    #[test]
    fn last_column_test() {
        assert_eq!(vec_cells(Columns::last(), 5, 2), [Entity::Column(1)]);
        assert_eq!(vec_cells(Columns::last(), 5, 29), [Entity::Column(28)]);
        assert_eq!(vec_cells(Columns::last(), 0, 0), []);
        assert_eq!(vec_cells(Columns::last(), 10, 0), []);
        assert_eq!(vec_cells(Columns::last(), 0, 10), []);
    }

    #[test]
    fn last_column_sub_test() {
        assert_eq!(vec_cells(Columns::last(), 5, 2), [Entity::Column(1)]);
        assert_eq!(vec_cells(Columns::last() - 0, 5, 2), [Entity::Column(1)]);
        assert_eq!(vec_cells(Columns::last() - 1, 5, 2), [Entity::Column(0)]);
        assert_eq!(vec_cells(Columns::last() - 2, 5, 2), []);
        assert_eq!(vec_cells(Columns::last() - 100, 5, 2), []);
    }

    #[test]
    fn first_column_add_test() {
        assert_eq!(vec_cells(Columns::first(), 5, 2), [Entity::Column(0)]);
        assert_eq!(vec_cells(Columns::first() + 0, 5, 2), [Entity::Column(0)]);
        assert_eq!(vec_cells(Columns::first() + 1, 5, 2), [Entity::Column(1)]);
        assert_eq!(vec_cells(Columns::first() + 2, 5, 2), []);
        assert_eq!(vec_cells(Columns::first() + 100, 5, 2), []);
    }

    #[test]
    fn rows_test() {
        assert_eq!(
            vec_cells(Rows::new(..), 2, 3),
            [Entity::Row(0), Entity::Row(1)]
        );
        assert_eq!(vec_cells(Rows::new(1..), 2, 3), [Entity::Row(1)]);
        assert_eq!(vec_cells(Rows::new(2..), 2, 3), []);
        assert_eq!(vec_cells(Rows::new(0..1), 2, 3), [Entity::Row(0)],);
        assert_eq!(vec_cells(Rows::new(1..2), 2, 3), [Entity::Row(1)],);
        assert_eq!(vec_cells(Rows::new(..), 0, 0), []);
        assert_eq!(vec_cells(Rows::new(..), 0, 3), []);
        assert_eq!(vec_cells(Rows::new(..), 2, 0), []);
    }

    #[test]
    fn last_row_test() {
        assert_eq!(vec_cells(Rows::last(), 5, 2), [Entity::Row(4)]);
        assert_eq!(vec_cells(Rows::last(), 100, 2), [Entity::Row(99)]);
        assert_eq!(vec_cells(Rows::last(), 0, 0), []);
        assert_eq!(vec_cells(Rows::last(), 5, 0), []);
        assert_eq!(vec_cells(Rows::last(), 0, 2), []);
    }

    #[test]
    fn first_row_test() {
        assert_eq!(vec_cells(Rows::first(), 5, 2), [Entity::Row(0)]);
        assert_eq!(vec_cells(Rows::first(), 100, 2), [Entity::Row(0)]);
        assert_eq!(vec_cells(Rows::first(), 0, 0), []);
        assert_eq!(vec_cells(Rows::first(), 5, 0), []);
        assert_eq!(vec_cells(Rows::first(), 0, 2), []);
    }

    #[test]
    fn last_row_sub_test() {
        assert_eq!(vec_cells(Rows::last(), 5, 2), [Entity::Row(4)]);
        assert_eq!(vec_cells(Rows::last() - 0, 5, 2), [Entity::Row(4)]);
        assert_eq!(vec_cells(Rows::last() - 1, 5, 2), [Entity::Row(3)]);
        assert_eq!(vec_cells(Rows::last() - 2, 5, 2), [Entity::Row(2)]);
        assert_eq!(vec_cells(Rows::last() - 3, 5, 2), [Entity::Row(1)]);
        assert_eq!(vec_cells(Rows::last() - 4, 5, 2), [Entity::Row(0)]);
        assert_eq!(vec_cells(Rows::last() - 5, 5, 2), []);
        assert_eq!(vec_cells(Rows::last() - 100, 5, 2), []);
        assert_eq!(vec_cells(Rows::last() - 1, 0, 0), []);
        assert_eq!(vec_cells(Rows::last() - 1, 5, 0), []);
        assert_eq!(vec_cells(Rows::last() - 1, 0, 2), []);
    }

    #[test]
    fn first_row_add_test() {
        assert_eq!(vec_cells(Rows::first(), 5, 2), [Entity::Row(0)]);
        assert_eq!(vec_cells(Rows::first() + 0, 5, 2), [Entity::Row(0)]);
        assert_eq!(vec_cells(Rows::first() + 1, 5, 2), [Entity::Row(1)]);
        assert_eq!(vec_cells(Rows::first() + 2, 5, 2), [Entity::Row(2)]);
        assert_eq!(vec_cells(Rows::first() + 3, 5, 2), [Entity::Row(3)]);
        assert_eq!(vec_cells(Rows::first() + 4, 5, 2), [Entity::Row(4)]);
        assert_eq!(vec_cells(Rows::first() + 5, 5, 2), []);
        assert_eq!(vec_cells(Rows::first() + 100, 5, 2), []);
        assert_eq!(vec_cells(Rows::first() + 1, 0, 0), []);
        assert_eq!(vec_cells(Rows::first() + 1, 5, 0), []);
        assert_eq!(vec_cells(Rows::first() + 1, 0, 2), []);
    }

    #[test]
    fn frame_test() {
        assert_eq!(
            vec_cells(Frame, 2, 3),
            [
                Entity::Cell(0, 0),
                Entity::Cell(0, 1),
                Entity::Cell(0, 2),
                Entity::Cell(1, 0),
                Entity::Cell(1, 1),
                Entity::Cell(1, 2)
            ]
        );
        assert_eq!(vec_cells(Frame, 0, 0), []);
        assert_eq!(vec_cells(Frame, 2, 0), []);
        assert_eq!(vec_cells(Frame, 0, 2), []);
    }

    #[test]
    fn segment_test() {
        assert_eq!(
            vec_cells(Segment::new(.., ..), 2, 3),
            [
                Entity::Cell(0, 0),
                Entity::Cell(0, 1),
                Entity::Cell(0, 2),
                Entity::Cell(1, 0),
                Entity::Cell(1, 1),
                Entity::Cell(1, 2)
            ]
        );
        assert_eq!(
            vec_cells(Segment::new(1.., ..), 2, 3),
            [Entity::Cell(1, 0), Entity::Cell(1, 1), Entity::Cell(1, 2)]
        );
        assert_eq!(vec_cells(Segment::new(2.., ..), 2, 3), []);

        assert_eq!(
            vec_cells(Segment::new(.., 1..), 2, 3),
            [
                Entity::Cell(0, 1),
                Entity::Cell(0, 2),
                Entity::Cell(1, 1),
                Entity::Cell(1, 2)
            ]
        );
        assert_eq!(
            vec_cells(Segment::new(.., 2..), 2, 3),
            [Entity::Cell(0, 2), Entity::Cell(1, 2)]
        );
        assert_eq!(vec_cells(Segment::new(.., 3..), 2, 3), []);

        assert_eq!(
            vec_cells(Segment::new(1.., 1..), 2, 3),
            [Entity::Cell(1, 1), Entity::Cell(1, 2)]
        );
        assert_eq!(
            vec_cells(Segment::new(1..2, 1..2), 2, 3),
            [Entity::Cell(1, 1)]
        );

        assert_eq!(vec_cells(Segment::new(5.., 5..), 2, 3), []);
    }

    #[test]
    fn object_and_test() {
        assert_eq!(
            vec_cells(Cell(0, 0).and(Cell(0, 0)), 2, 3),
            [Entity::Cell(0, 0)]
        );
        assert_eq!(
            vec_cells(Cell(0, 0).and(Cell(1, 2)), 2, 3),
            [Entity::Cell(0, 0), Entity::Cell(1, 2)]
        );
        assert_eq!(vec_cells(Cell(0, 0).and(Cell(1, 2)), 0, 0), []);
    }

    #[test]
    fn object_not_test() {
        assert_eq!(vec_cells(Cell(0, 0).not(Cell(0, 0)), 2, 3), []);
        assert_eq!(
            vec_cells(Rows::first().not(Cell(0, 0)), 2, 3),
            [Entity::Cell(0, 1), Entity::Cell(0, 2)]
        );
        assert_eq!(vec_cells(Rows::first().not(Cell(0, 0)), 0, 0), []);
    }

    #[test]
    fn object_intersect_test() {
        assert_eq!(
            vec_cells(Segment::all().intersect(Rows::single(1)), 2, 3),
            [Entity::Cell(1, 0), Entity::Cell(1, 1), Entity::Cell(1, 2)]
        );
        assert_eq!(
            vec_cells(Cell(0, 0).intersect(Cell(0, 0)), 2, 3),
            [Entity::Cell(0, 0)]
        );
        assert_eq!(
            vec_cells(Rows::first().intersect(Cell(0, 0)), 2, 3),
            [Entity::Cell(0, 0)]
        );
        assert_eq!(vec_cells(Rows::first().intersect(Cell(0, 0)), 0, 0), []);
    }

    #[test]
    fn object_inverse_test() {
        assert_eq!(vec_cells(Segment::all().inverse(), 2, 3), []);
        assert_eq!(
            vec_cells(Cell(0, 0).inverse(), 2, 3),
            [
                Entity::Cell(0, 1),
                Entity::Cell(0, 2),
                Entity::Cell(1, 0),
                Entity::Cell(1, 1),
                Entity::Cell(1, 2)
            ]
        );
        assert_eq!(
            vec_cells(Rows::first().inverse(), 2, 3),
            [Entity::Cell(1, 0), Entity::Cell(1, 1), Entity::Cell(1, 2)]
        );
        assert_eq!(vec_cells(Rows::first().inverse(), 0, 0), []);
    }

    fn vec_cells<O: Object>(o: O, count_rows: usize, count_cols: usize) -> Vec<Entity> {
        let data = vec![vec![String::default(); count_cols]; count_rows];
        let table = crate::builder::Builder::from(data).build();
        o.cells(&table).collect::<Vec<_>>()
    }
}
