use std::{borrow::Cow, cmp::max};

use crate::{
    records::vec_records::{Cell, CellMut},
    util::get_lines,
    width::WidthFunc,
};

#[derive(Debug, Clone, Default)]
pub struct CellInfo<'a> {
    text: Cow<'a, str>,
    lines: Vec<StrWithWidth<'a>>,
    width: usize,
}

impl<'a> CellInfo<'a> {
    pub fn new<S, W>(text: S, width_ctrl: W) -> Self
    where
        S: Into<Cow<'a, str>>,
        W: WidthFunc,
    {
        create_cell_info(text.into(), width_ctrl)
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}

impl Cell for CellInfo<'_> {
    fn get_line(&self, i: usize) -> &str {
        if i == 0 && self.lines.is_empty() {
            return &self.text;
        }

        &self.lines[i].text
    }

    fn count_lines(&self) -> usize {
        max(self.lines.len(), 1)
    }

    fn width<W>(&self, _: W) -> usize
    where
        W: WidthFunc,
    {
        self.width
    }

    fn line_width<W>(&self, i: usize, _: W) -> usize
    where
        W: WidthFunc,
    {
        if i == 0 && self.lines.is_empty() {
            return self.width;
        }

        self.lines[i].width
    }
}

impl<'a, T> CellMut<T> for CellInfo<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn update<W>(&mut self, width_ctrl: W)
    where
        W: WidthFunc,
    {
        self.width = 0;
        update_cell_info(self, width_ctrl);
    }

    fn set<W>(&mut self, text: T, width_ctrl: W)
    where
        W: WidthFunc,
    {
        *self = create_cell_info(text.into(), width_ctrl);
    }
}

impl AsRef<str> for CellInfo<'_> {
    fn as_ref(&self) -> &str {
        &self.text
    }
}

#[derive(Debug, Clone, Default)]
struct StrWithWidth<'a> {
    text: Cow<'a, str>,
    width: usize,
}

impl<'a> StrWithWidth<'a> {
    fn new(text: Cow<'a, str>, width: usize) -> Self {
        Self { text, width }
    }
}

fn create_cell_info<W>(text: Cow<'_, str>, width_fn: W) -> CellInfo<'_>
where
    W: WidthFunc,
{
    let mut info = CellInfo {
        text,
        ..Default::default()
    };

    let text = info.text.as_ref();

    // We want to mitigate additional allocations whereas possible.
    //
    // # Safety
    //
    // It must be save
    let text = unsafe {
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(text.as_ptr(), text.len()))
    };

    let mut lines = get_lines(text);

    // optimize for a general case where we have only 1 line.
    // to not make any allocations
    let first_line = lines.next();
    if first_line.is_none() {
        return info;
    }

    let first_line = first_line.unwrap();
    let first_width = width_fn.width(first_line.as_ref());
    info.width = first_width;

    let second_line = lines.next();
    if second_line.is_none() {
        return info;
    }

    info.lines.push(StrWithWidth::new(first_line, first_width));

    let second_line = second_line.unwrap();
    let second_width = width_fn.width(second_line.as_ref());
    info.lines
        .push(StrWithWidth::new(second_line, second_width));

    info.width = max(info.width, second_width);

    for line in lines {
        let width = width_fn.width(line.as_ref());
        info.width = max(info.width, width);

        let line = StrWithWidth::new(Cow::Owned(line.to_string()), width);
        info.lines.push(line);
    }

    info
}

fn update_cell_info<W>(info: &mut CellInfo<'_>, width_fn: W)
where
    W: WidthFunc,
{
    if info.text.is_empty() {
        return;
    }

    if info.lines.is_empty() && !info.text.is_empty() {
        info.width = width_fn.width(&info.text);
        return;
    }

    for line in info.lines.iter_mut() {
        line.width = width_fn.width(&line.text);
        info.width = max(info.width, line.width);
    }
}
