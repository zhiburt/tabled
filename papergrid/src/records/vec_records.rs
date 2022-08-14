use std::{
    fmt::{self, Formatter},
    ops::{Index, IndexMut},
};

use super::{cell_info::CellInfo, Records, RecordsMut, Resizable};
use crate::{width::WidthFunc, Position};

#[derive(Debug, Default, Clone)]
pub struct VecRecords<T> {
    records: Vec<Vec<T>>,
    size: (usize, usize),
}

impl<'a> VecRecords<CellInfo<'a>> {
    pub fn new<R, C, T, W>(records: R, size: (usize, usize), width_ctrl: W) -> Self
    where
        R: IntoIterator<Item = C> + 'a,
        C: IntoIterator<Item = T> + 'a,
        T: AsRef<str> + 'a,
        W: WidthFunc,
    {
        let records = create_records(records, width_ctrl, size);
        Self { records, size }
    }
}

impl<T> VecRecords<T> {
    pub fn with_hint(records: Vec<Vec<T>>, count_columns: usize) -> Self {
        let count_rows = records.len();
        let size = (count_rows, count_columns);

        Self { records, size }
    }

    pub fn size(&self) -> (usize, usize) {
        self.size
    }

    pub fn count_rows(&self) -> usize {
        self.size.0
    }

    pub fn count_columns(&self) -> usize {
        self.size.1
    }

    pub fn truncate(&mut self, len: usize) {
        if self.size.1 > len {
            self.size.1 = len;
            for row in &mut self.records {
                row.truncate(len);
            }
        }
    }
}

impl<T> VecRecords<T>
where
    T: Clone,
{
    pub fn push(&mut self, cell: T) {
        for row in &mut self.records {
            row.push(cell.clone());
        }

        self.size.1 += 1;
    }
}

impl<T> From<Vec<Vec<T>>> for VecRecords<T> {
    fn from(records: Vec<Vec<T>>) -> Self {
        let count_rows = records.len();
        let count_cols = records.get(0).map_or(0, |r| r.len());
        let size = (count_rows, count_cols);

        Self { records, size }
    }
}

impl<T> VecRecords<T>
where
    T: Clone,
{
    pub fn duplicate_row(&mut self, row: usize) {
        if row >= self.size.0 {
            return;
        }

        let row = self.records[row].clone();
        self.records.push(row);
        self.size.0 += 1;
    }
}

impl<T> Records for VecRecords<T>
where
    T: Cell,
{
    fn count_rows(&self) -> usize {
        self.size.0
    }

    fn count_columns(&self) -> usize {
        self.size.1
    }

    fn get_text(&self, (row, col): Position) -> &str {
        self.records[row][col].as_ref()
    }

    fn get_line(&self, (row, col): Position, i: usize) -> &str {
        self.records[row][col].get_line(i)
    }

    fn get_width<W>(&self, (row, col): Position, width_ctrl: W) -> usize
    where
        W: WidthFunc,
    {
        self.records[row][col].width(width_ctrl)
    }

    fn get_line_width<W>(&self, (row, col): Position, i: usize, width_ctrl: W) -> usize
    where
        W: WidthFunc,
    {
        self.records[row][col].line_width(i, width_ctrl)
    }

    fn count_lines(&self, (row, col): Position) -> usize {
        self.records[row][col].count_lines()
    }

    fn fmt_text_prefix(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        (row, col): Position,
    ) -> std::fmt::Result {
        self.records[row][col].fmt_prefix(f)
    }

    fn fmt_text_suffix(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        (row, col): Position,
    ) -> std::fmt::Result {
        self.records[row][col].fmt_suffix(f)
    }
}

impl<T, Q> RecordsMut<Q> for VecRecords<T>
where
    T: CellMut<Q>,
{
    fn set<W>(&mut self, (row, col): Position, text: Q, width_ctrl: W)
    where
        W: WidthFunc,
    {
        self.records[row][col].set(text, width_ctrl);
    }

    fn update<W>(&mut self, (row, col): Position, width_ctrl: W)
    where
        W: WidthFunc,
    {
        self.records[row][col].update(width_ctrl);
    }
}

impl<T> Resizable for VecRecords<T>
where
    T: Default + Clone,
{
    fn swap(&mut self, lhs: Position, rhs: Position) {
        if lhs.0 >= self.size.0
            || lhs.1 >= self.size.1
            || rhs.0 >= self.size.0
            || rhs.1 >= self.size.1
        {
            return;
        }

        if lhs == rhs {
            return;
        }

        let t = std::mem::take(&mut self.records[lhs.0][lhs.1]);
        let t = std::mem::replace(&mut self.records[rhs.0][rhs.1], t);
        let _ = std::mem::replace(&mut self.records[lhs.0][lhs.1], t);
    }

    fn swap_row(&mut self, lhs: usize, rhs: usize) {
        if lhs >= self.size.0 || rhs >= self.size.0 {
            return;
        }

        let t = std::mem::take(&mut self.records[lhs]);
        let t = std::mem::replace(&mut self.records[rhs], t);
        let _ = std::mem::replace(&mut self.records[lhs], t);
    }

    fn swap_column(&mut self, lhs: usize, rhs: usize) {
        if lhs >= self.size.1 || rhs >= self.size.1 {
            return;
        }

        for row in &mut self.records {
            row.swap(lhs, rhs);
        }
    }

    fn push_row(&mut self) {
        self.size.0 += 1;
        self.records.push(vec![T::default(); self.size.1]);
    }

    fn push_column(&mut self) {
        self.size.1 += 1;
        for row in &mut self.records {
            row.push(T::default());
        }
    }

    fn remove_row(&mut self, row: usize) {
        if row >= self.records.len() {
            return;
        }

        self.records.remove(row);
        self.size.0 -= 1;
    }

    fn remove_column(&mut self, column: usize) {
        if column >= self.size.1 {
            return;
        }

        for row in &mut self.records {
            row.remove(column);
        }
        self.size.1 -= 1;
    }

    fn insert_row(&mut self, row: usize) {
        self.records.insert(row, vec![T::default(); self.size.1]);
    }
}

impl<T> Index<Position> for VecRecords<T> {
    type Output = T;

    fn index(&self, (row, col): Position) -> &Self::Output {
        &self.records[row][col]
    }
}

impl<T> IndexMut<Position> for VecRecords<T> {
    fn index_mut(&mut self, (row, col): Position) -> &mut Self::Output {
        &mut self.records[row][col]
    }
}

fn create_records<'a, I, T, S, W>(
    data: I,
    width_ctrl: W,
    (hint_count_rows, hint_count_cols): (usize, usize),
) -> Vec<Vec<CellInfo<'a>>>
where
    I: IntoIterator<Item = T>,
    T: IntoIterator<Item = S>,
    S: AsRef<str> + 'a,
    W: WidthFunc,
{
    let mut cells = vec![vec![CellInfo::default(); hint_count_cols]; hint_count_rows];
    let mut count_rows = hint_count_rows;

    for (row, rows) in data.into_iter().enumerate() {
        if row >= count_rows {
            cells.push(vec![CellInfo::default(); hint_count_cols]);
            count_rows += 1;
        }

        for (col, text) in rows.into_iter().enumerate().take(hint_count_cols) {
            let text = text.as_ref();
            if text.is_empty() {
                continue;
            }

            cells[row][col] = CellInfo::new(text.to_owned(), &width_ctrl);
        }
    }

    cells
}

pub trait Cell: AsRef<str> {
    fn get_line(&self, i: usize) -> &str;
    fn count_lines(&self) -> usize;
    fn width<W>(&self, width_ctrl: W) -> usize
    where
        W: WidthFunc;
    fn line_width<W>(&self, i: usize, width_ctrl: W) -> usize
    where
        W: WidthFunc;
    fn fmt_prefix(&self, _: &mut Formatter<'_>) -> fmt::Result {
        Ok(())
    }
    fn fmt_suffix(&self, _: &mut Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

pub trait CellMut<T> {
    fn set<W>(&mut self, text: T, width_ctrl: W)
    where
        W: WidthFunc;
    fn update<W>(&mut self, width_ctrl: W)
    where
        W: WidthFunc;
}
