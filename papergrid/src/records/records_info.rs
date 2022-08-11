use std::{borrow::Cow, cmp::max, fmt};

use crate::{
    util::get_lines,
    width::{CfgWidthFunction, WidthFunc},
    GridConfig, Position,
};

use super::{Cell, Records, RecordsMut, Resizable, Text};

#[derive(Debug, Default, Clone)]
pub struct RecordsInfo<'a> {
    records: Vec<Vec<CellInfo<'a>>>,
    size: (usize, usize),
}

impl<'a> RecordsInfo<'a> {
    pub fn new<I, T, S>(records: I, mut size: (usize, usize), cfg: &GridConfig) -> Self
    where
        I: IntoIterator<Item = T> + 'a,
        T: IntoIterator<Item = S> + 'a,
        S: AsRef<str> + 'a,
    {
        let width_fn = CfgWidthFunction::from_cfg(cfg);
        let records = create_records(records, &mut size.0, size.1, width_fn);
        Self { records, size }
    }

    pub fn from_vec(records: Vec<Vec<CellInfo<'a>>>, size: (usize, usize)) -> Self {
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
}

impl<'a, 'b> Records for &'a RecordsInfo<'b> {
    type Cell = &'a CellInfo<'b>;

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

impl RecordsMut for RecordsInfo<'_> {
    fn set_text<W>(&mut self, (row, col): Position, text: String, width_fn: W)
    where
        W: WidthFunc,
    {
        let info = &mut self.records[row][col];
        info.lines.clear();
        info.width = 0;
        create_cell_info(info, text, width_fn);
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

impl Resizable for RecordsInfo<'_> {
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
        self.records.push(vec![CellInfo::default(); self.size.1]);
        self.size.0 += 1;
    }

    fn push_column(&mut self) {
        for row in &mut self.records {
            row.push(CellInfo::default());
        }
        self.size.1 += 1;
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
        self.records
            .insert(row, vec![CellInfo::default(); self.size.1]);
    }
}

#[derive(Debug, Clone, Default)]
pub struct CellInfo<'a> {
    text: Cow<'a, str>,
    lines: Vec<StrWithWidth<'a>>,
    width: usize,
}

impl<'a> CellInfo<'a> {
    pub fn new(text: Cow<'a, str>, lines: Vec<StrWithWidth<'a>>, width: usize) -> Self {
        Self { text, lines, width }
    }

    pub fn from_str<W>(text: String, width_ctrl: W) -> Self
    where
        W: WidthFunc,
    {
        let mut info = CellInfo::default();
        create_cell_info(&mut info, text, width_ctrl);
        info
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}

impl<'a, 'b> Cell for &'a CellInfo<'b> {
    type Text = StrWithWidth<'a>;
    type Lines = CellInfoLines<'a>;

    fn lines(&self) -> Self::Lines {
        CellInfoLines::new(self.lines.iter())
    }

    fn get_line(&self, i: usize) -> Option<Self::Text> {
        self.lines
            .get(i)
            .map(|s| StrWithWidth::new(Cow::Borrowed(&s.text), s.width))
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

#[cfg(feature = "color")]
impl crate::Color for CellInfo<'_> {
    fn fmt_prefix(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }

    fn fmt_suffix(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

#[derive(Debug)]
pub struct CellInfoLines<'a> {
    lines: std::slice::Iter<'a, StrWithWidth<'a>>,
}

impl<'a> CellInfoLines<'a> {
    pub fn new(lines: std::slice::Iter<'a, StrWithWidth<'a>>) -> Self {
        Self { lines }
    }
}

impl<'a> Iterator for CellInfoLines<'a> {
    type Item = StrWithWidth<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines
            .next()
            .map(|s| StrWithWidth::new(Cow::Borrowed(s.text.as_ref()), s.width))
    }
}

#[derive(Debug, Clone, Default)]
pub struct StrWithWidth<'a> {
    pub(crate) text: Cow<'a, str>,
    pub(crate) width: usize,
}

impl<'a> StrWithWidth<'a> {
    pub fn new(text: Cow<'a, str>, width: usize) -> Self {
        Self { text, width }
    }

    pub fn set_width(&mut self, width: usize) {
        self.width = width;
    }
}

impl AsRef<str> for StrWithWidth<'_> {
    fn as_ref(&self) -> &str {
        &self.text
    }
}

impl Text for StrWithWidth<'_> {
    fn as_str(&self) -> &str {
        &self.text
    }

    fn width<W>(&self, _: W) -> usize
    where
        W: WidthFunc,
    {
        self.width
    }
}

impl fmt::Display for StrWithWidth<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.text.fmt(f)
    }
}

fn create_records<'a, I, T, S, W>(
    data: I,
    count_rows: &mut usize,
    count_cols: usize,
    width_fn: W,
) -> Vec<Vec<CellInfo<'a>>>
where
    I: IntoIterator<Item = T>,
    T: IntoIterator<Item = S>,
    S: AsRef<str> + 'a,
    W: WidthFunc,
{
    let mut cells = vec![vec![CellInfo::default(); count_cols]; *count_rows];

    for (row, rows) in data.into_iter().enumerate() {
        if row >= *count_rows {
            cells.push(vec![CellInfo::default(); count_cols]);
            *count_rows += 1;
        }

        for (col, text) in rows.into_iter().enumerate().take(count_cols) {
            let text = text.as_ref().to_owned();
            if text.is_empty() {
                continue;
            }

            let info = &mut cells[row][col];
            create_cell_info(info, text, &width_fn);
        }
    }

    cells
}

fn create_cell_info<W>(info: &mut CellInfo<'_>, text: String, width_fn: W)
where
    W: WidthFunc,
{
    info.text = Cow::Owned(text);
    for line in get_lines(info.text.as_ref()) {
        let width = width_fn.width(line.as_ref());
        let line = StrWithWidth::new(line.to_string().into(), width);
        info.width = max(info.width, width);
        info.lines.push(line);
    }
}
