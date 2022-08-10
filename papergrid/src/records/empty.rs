use std::iter;

use crate::{width::WidthFunc, Position};

use super::{Cell, Records, Text};

#[derive(Debug, Default, Clone)]
pub struct EmptyRecords {
    rows: usize,
    cols: usize,
}

impl EmptyRecords {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { rows, cols }
    }
}

impl Records for EmptyRecords {
    type Cell = ();

    fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    fn get(&self, _: Position) -> Self::Cell {}

    fn get_text(&self, _: Position) -> &str {
        ""
    }
}

impl Cell for () {
    type Text = ();
    type Lines = iter::Empty<()>;

    fn lines(&self) -> Self::Lines {
        iter::empty()
    }

    fn get_line(&self, _: usize) -> Option<Self::Text> {
        Some(())
    }

    fn count_lines(&self) -> usize {
        1
    }

    fn width<W>(&self, _: W) -> usize {
        0
    }
}

impl Text for () {
    fn as_str(&self) -> &str {
        ""
    }

    fn width<W>(&self, _: W) -> usize
    where
        W: WidthFunc,
    {
        0
    }
}
