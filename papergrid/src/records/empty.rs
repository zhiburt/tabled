//! An empty [`Records`] implementation.

use std::iter;

use crate::{records::Records, Position};

use super::{ExactRecords, RecordCell};

/// Empty representation of [`Records`].
#[derive(Debug, Default, Clone)]
pub struct EmptyRecords {
    rows: usize,
    cols: usize,
}

impl EmptyRecords {
    /// Constructs an empty representation of [`Records`] with a given shape.
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { rows, cols }
    }
}

impl Records for EmptyRecords {
    type Cell = EmptyCell;
    type Cells = iter::Empty<Self::Cell>;
    type IntoRecords = iter::Empty<Self::Cells>;

    fn count_columns(&self) -> usize {
        self.cols
    }

    fn iter_rows(&self) -> Self::IntoRecords {
        iter::empty()
    }

    fn hint_rows(&self) -> Option<usize> {
        Some(self.rows)
    }
}

impl ExactRecords for EmptyRecords {
    fn count_rows(&self) -> usize {
        self.rows
    }

    fn get(&self, _: Position) -> Option<Self::Cell> {
        Some(EmptyCell)
    }
}

pub struct EmptyCell;

impl RecordCell for EmptyCell {
    type Text = String;
    type Line = String;
    type Lines = Vec<String>;

    fn get_text(&self) -> Self::Text {
        String::new()
    }

    fn get_line(&self, _: usize) -> Self::Line {
        String::new()
    }

    fn get_lines(&self) -> Self::Lines {
        vec![]
    }

    fn count_lines(&self) -> usize {
        1
    }
}
