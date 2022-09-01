//! A [`Cell`] implementation for [`VecRecords`].
//!
//! [`VecRecords`]: crate::records::vec_records::VecRecords

use std::{borrow::Cow, cmp::max};

use crate::{
    records::vec_records::{Cell, CellMut},
    util::{count_lines, get_lines},
    width::WidthFunc,
};

/// The struct is a [Cell] implementation which keeps width information pre allocated.
#[derive(Debug, Default)]
pub struct CellInfo<'a> {
    text: Cow<'a, str>,
    width: usize,
    lines: Vec<StrWithWidth<'a>>,
    count_lines: usize,
}

impl<'a> CellInfo<'a> {
    /// Creates a new instance of the structure.
    pub fn new<S, W>(text: S, width_ctrl: W) -> Self
    where
        S: Into<Cow<'a, str>>,
        W: WidthFunc,
    {
        create_cell_info(text.into(), width_ctrl)
    }

    /// Checks if the containing string is empty.
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
        self.count_lines
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
        let text = text.into();
        *self = create_cell_info(text, width_ctrl);
    }
}

impl AsRef<str> for CellInfo<'_> {
    fn as_ref(&self) -> &str {
        &self.text
    }
}

impl Clone for CellInfo<'_> {
    fn clone(&self) -> Self {
        let mut cell = Self {
            text: self.text.clone(),
            width: self.width,
            lines: vec![StrWithWidth::default(); self.lines.len()],
            count_lines: self.count_lines,
        };

        for (i, line) in self.lines.iter().enumerate() {
            cell.lines[i].width = line.width;

            cell.lines[i].text = match &line.text {
                Cow::Owned(line) => Cow::Owned(line.clone()),
                Cow::Borrowed(s) => {
                    // We need to redirect pointers to the original string.
                    //
                    // # Safety
                    //
                    // It must be safe because the referenced string and the references are dropped at the same time.
                    // And the referenced String is guaranted to not be changed.
                    let text = unsafe {
                        let text_ptr = self.text.as_ptr();
                        let line_ptr = s.as_ptr();
                        let text_shift = line_ptr as isize - text_ptr as isize;

                        let new_text_shifted_ptr = cell.text.as_ptr().offset(text_shift);

                        std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                            new_text_shifted_ptr,
                            s.len(),
                        ))
                    };

                    Cow::Borrowed(text)
                }
            }
        }

        cell
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
        count_lines: 1,
        ..Default::default()
    };

    // Here we do a small optimization.
    // We check if there's only 1 line in which case we don't allocate lines Vec
    let count_lines = count_lines(&info.text);
    if count_lines < 2 {
        info.width = width_fn.width(&info.text);
        return info;
    }

    // In case `Cow::Borrowed` we want to not allocate a String.
    // It's currerently not possible due to a lifetime issues. (It's known as self-referential struct)
    //
    // Here we change the lifetime of text.
    //
    // # Safety
    //
    // It must be safe because the referenced string and the references are dropped at the same time.
    // And the referenced String is guaranted to not be changed.
    let text = unsafe {
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(
            info.text.as_ptr(),
            info.text.len(),
        ))
    };

    info.count_lines = count_lines;
    info.lines = vec![StrWithWidth::new(Cow::Borrowed(""), 0); count_lines];
    for (line, i) in get_lines(text).zip(info.lines.iter_mut()) {
        let width = width_fn.width(line.as_ref());
        info.width = max(info.width, width);
        i.text = line;
        i.width = width;
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

    for line in &mut info.lines {
        line.width = width_fn.width(&line.text);
        info.width = max(info.width, line.width);
    }
}
