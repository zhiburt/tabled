use core::fmt::Display;
use std::{borrow::Cow, cmp::max};

use crate::{
    records::vec_records::Cell,
    util::string::{self, count_lines, get_line_width, get_lines},
};

/// The struct is a [Cell] implementation which keeps width information pre allocated.
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Text<S> {
    text: S,
    width: usize,
    lines: Vec<StrWithWidth<'static>>,
}

impl<S> Text<S> {
    /// Creates a new instance of the structure.
    pub fn new(text: S) -> Self
    where
        S: AsRef<str>,
    {
        create_text(text)
    }

    /// Creates a new instance of the structure with a single line.
    pub fn exact(text: S, width: usize, lines: Vec<StrWithWidth<'static>>) -> Self {
        Self { text, width, lines }
    }

    /// Return a original text value.
    pub fn into_inner(self) -> S {
        self.text
    }
}

impl<S> AsRef<str> for Text<S>
where
    S: AsRef<str>,
{
    fn as_ref(&self) -> &str {
        self.text()
    }
}

impl<S> Display for Text<S>
where
    S: Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.text.fmt(f)
    }
}

impl<S> Cell for Text<S>
where
    S: AsRef<str>,
{
    fn text(&self) -> &str {
        self.text.as_ref()
    }

    fn line(&self, i: usize) -> &str {
        if i == 0 && self.lines.is_empty() {
            return self.text.as_ref();
        }

        &self.lines[i].text
    }

    fn count_lines(&self) -> usize {
        std::cmp::max(1, self.lines.len())
    }

    fn width(&self) -> usize {
        self.width
    }

    fn line_width(&self, i: usize) -> usize {
        if i == 0 && self.lines.is_empty() {
            return self.width;
        }

        self.lines[i].width
    }
}

impl<S> Clone for Text<S>
where
    S: Clone + AsRef<str>,
{
    fn clone(&self) -> Self {
        let mut cell = Self {
            text: self.text.clone(),
            width: self.width,
            lines: vec![StrWithWidth::default(); self.lines.len()],
        };

        for (i, line) in self.lines.iter().enumerate() {
            // We need to redirect pointers to the original string.
            //
            // # Safety
            //
            // It must be safe because the referenced string and the references are dropped at the same time.
            // And the referenced String is guaranteed to not be changed.
            let text = unsafe {
                let text_ptr = self.text.as_ref().as_ptr();
                let line_ptr = line.text.as_ptr();
                let text_shift = line_ptr as isize - text_ptr as isize;

                let new_text_shifted_ptr = cell.text.as_ref().as_ptr().offset(text_shift);

                std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                    new_text_shifted_ptr,
                    line.text.len(),
                ))
            };

            cell.lines[i].width = line.width;
            cell.lines[i].text = Cow::Borrowed(text);
        }

        cell
    }
}

/// StrWithWidth is a structure is responsible for a string and it's width.
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct StrWithWidth<'a> {
    text: Cow<'a, str>,
    width: usize,
}

impl<'a> StrWithWidth<'a> {
    /// Creates a new object.
    pub fn new(text: Cow<'a, str>, width: usize) -> Self {
        Self { text, width }
    }
}

// TODO: compare with 'get_text_dimension'
fn create_text<S: AsRef<str>>(text: S) -> Text<S> {
    let mut info = Text {
        text,
        lines: vec![],
        width: 0,
    };

    // Here we do a small optimization.
    // We check if there's only 1 line in which case we don't allocate lines Vec
    let count_lines = count_lines(info.text.as_ref());
    if count_lines < 2 {
        info.width = string::get_text_width(info.text.as_ref());
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
    // And the referenced String is guaranteed to not be changed.
    let text = unsafe {
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(
            info.text.as_ref().as_ptr(),
            info.text.as_ref().len(),
        ))
    };

    info.lines = vec![StrWithWidth::new(Cow::Borrowed(""), 0); count_lines];
    for (line, i) in get_lines(text).zip(info.lines.iter_mut()) {
        i.width = get_line_width(&line);
        i.text = line;
        info.width = max(info.width, i.width);
    }

    info
}
