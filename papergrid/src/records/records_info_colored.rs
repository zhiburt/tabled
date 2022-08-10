use std::{
    borrow::Cow,
    cmp::max,
    fmt,
    ops::{Index, IndexMut},
};

use crate::{
    records::{
        records_info::{CellInfoLines, StringWithWidth},
        Cell, Records,
    },
    util::get_lines,
    width::{CfgWidthFunction, WidthFunc},
    Color, GridConfig, Position,
};

use super::RecordsMut;

#[derive(Debug, Default, Clone)]
pub struct RecordsInfo<'a, T> {
    records: Vec<Vec<CellInfo<'a, T>>>,
    size: (usize, usize),
}

impl<'a, TT> RecordsInfo<'a, TT> {
    pub fn new<I, T, S>(records: I, mut size: (usize, usize), cfg: &GridConfig) -> Self
    where
        I: IntoIterator<Item = T> + 'a,
        T: IntoIterator<Item = (S, TT)> + 'a,
        S: AsRef<str> + 'a,
        TT: Default + Clone + 'a,
    {
        let width_fn = CfgWidthFunction::new(cfg);
        let records = create_records(records, &mut size.0, size.1, width_fn);
        Self { records, size }
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

impl<'a, TT> RecordsInfo<'a, TT>
where
    TT: Default,
{
    pub fn push(&mut self, text: String, cfg: &GridConfig) {
        let ctrl = CfgWidthFunction::new(cfg);
        for row in &mut self.records {
            let cell = create_cell_info(text.clone(), &ctrl);
            row.push(cell);
        }
    }
}

impl<'a, 'b, T> Records for &'a RecordsInfo<'b, T> {
    type Cell = &'a CellInfo<'b, T>;

    fn size(&self) -> (usize, usize) {
        self.size
    }

    fn get(&self, (row, col): Position) -> Self::Cell {
        &self.records[row][col]
    }

    fn get_text(&self, (row, col): Position) -> &str {
        &self.records[row][col].text
    }
}
impl<'b, T> RecordsMut for RecordsInfo<'b, T>
where
    T: Default,
{
    fn set_text<W>(&mut self, (row, col): Position, text: String, width_fn: W)
    where
        W: WidthFunc,
    {
        self.records[row][col] = create_cell_info(text, width_fn);
    }

    fn update<W>(&mut self, (row, col): Position, width: W)
    where
        W: WidthFunc,
    {
        let info = &mut self.records[row][col];
        info.width = 0;
        for line in &mut info.lines {
            let width = width.width(line.as_ref());
            line.set_width(width);
            info.width = max(info.width, width);
        }
    }
}

impl<T> Index<Position> for RecordsInfo<'_, T> {
    type Output = T;

    fn index(&self, (row, col): Position) -> &Self::Output {
        &self.records[row][col].data
    }
}

impl<T> IndexMut<Position> for RecordsInfo<'_, T> {
    fn index_mut(&mut self, (row, col): Position) -> &mut Self::Output {
        &mut self.records[row][col].data
    }
}

#[derive(Debug, Clone, Default)]
pub struct CellInfo<'a, T> {
    text: Cow<'a, str>,
    lines: Vec<StringWithWidth>,
    width: usize,
    data: T,
}

impl<'a, T> CellInfo<'a, T> {
    pub fn new(text: Cow<'a, str>, lines: Vec<StringWithWidth>, width: usize, data: T) -> Self {
        Self {
            text,
            lines,
            width,
            data,
        }
    }
}

impl<'a, 'b, T> Cell for &'a CellInfo<'b, T> {
    type Text = StringWithWidth;
    type Lines = CellInfoLines<'a>;

    fn lines(&self) -> Self::Lines {
        CellInfoLines::new(self.lines.iter())
    }

    fn get_line(&self, i: usize) -> Option<Self::Text> {
        self.lines.get(i).cloned()
    }

    fn count_lines(&self) -> usize {
        self.lines.len()
    }

    fn width<W>(&self, _: W) -> usize
    where
        W: WidthFunc,
    {
        self.width
    }
}

impl<T> Color for CellInfo<'_, T>
where
    T: Color,
{
    fn fmt_prefix(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.data.fmt_prefix(f)
    }

    fn fmt_suffix(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.data.fmt_suffix(f)
    }
}

impl Color for () {
    fn fmt_prefix(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }

    fn fmt_suffix(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

fn create_records<'a, I, T, S, W, TT>(
    data: I,
    count_rows: &mut usize,
    count_cols: usize,
    width_fn: W,
) -> Vec<Vec<CellInfo<'a, TT>>>
where
    I: IntoIterator<Item = T>,
    T: IntoIterator<Item = (S, TT)>,
    S: AsRef<str> + 'a,
    W: WidthFunc,
    TT: Default + Clone,
{
    let mut cells = vec![vec![CellInfo::default(); count_cols]; *count_rows];

    for (row, rows) in data.into_iter().enumerate() {
        if row >= *count_rows {
            cells.push(vec![CellInfo::default(); count_cols]);
            *count_rows += 1;
        }

        for (col, (text, clr)) in rows.into_iter().enumerate().take(count_cols) {
            let text = text.as_ref().to_owned();
            if text.is_empty() {
                continue;
            }

            cells[row][col] = create_cell_info(text, &width_fn);
            cells[row][col].data = clr;
        }
    }

    cells
}

fn create_cell_info<W: WidthFunc, T: Default>(text: String, width_fn: W) -> CellInfo<'static, T> {
    let mut info = CellInfo {
        text: Cow::Owned(text),
        ..Default::default()
    };

    for line in get_lines(info.text.as_ref()) {
        let width = width_fn.width(line.as_ref());
        let line = StringWithWidth::new(line.to_string(), width);
        info.width = max(info.width, width);
        info.lines.push(line);
    }

    info
}
