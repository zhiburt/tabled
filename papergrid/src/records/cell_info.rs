use std::{
    borrow::Cow,
    cmp::max,
    fmt::{Formatter, Result},
};

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
        &self.lines[i].text
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

    fn line_width<W>(&self, i: usize, _: W) -> usize
    where
        W: WidthFunc,
    {
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
        update_cell_info(self, width_ctrl);
    }

    fn set(&mut self, text: T) {
        self.text = text.into();
    }
}

impl AsRef<str> for CellInfo<'_> {
    fn as_ref(&self) -> &str {
        &self.text
    }
}

#[cfg(feature = "color")]
impl crate::Color for CellInfo<'_> {
    fn fmt_prefix(&self, _: &mut Formatter<'_>) -> Result {
        Ok(())
    }

    fn fmt_suffix(&self, _: &mut Formatter<'_>) -> Result {
        Ok(())
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

    for line in get_lines(info.text.as_ref()) {
        let width = width_fn.width(line.as_ref());
        let line = StrWithWidth::new(Cow::Owned(line.to_string()), width);
        info.width = max(info.width, width);
        info.lines.push(line);
    }

    info
}

fn update_cell_info<W>(info: &mut CellInfo<'_>, width_fn: W)
where
    W: WidthFunc,
{
    info.lines.clear();
    info.width = 0;
    for line in get_lines(info.text.as_ref()) {
        let width = width_fn.width(line.as_ref());
        let line = StrWithWidth::new(Cow::Owned(line.to_string()), width);
        info.width = max(info.width, width);
        info.lines.push(line);
    }
}
