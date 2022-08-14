use crate::{records::Records, Position};

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
    fn count_rows(&self) -> usize {
        self.rows
    }

    fn count_columns(&self) -> usize {
        self.cols
    }

    fn get_text(&self, _: Position) -> &str {
        ""
    }

    fn get_line(&self, _: Position, _: usize) -> &str {
        ""
    }

    fn get_width<W>(&self, _: Position, _: W) -> usize {
        0
    }

    fn get_line_width<W>(&self, _: Position, _: usize, _: W) -> usize {
        0
    }

    fn count_lines(&self, _: Position) -> usize {
        1
    }

    fn fmt_text_prefix(&self, _: &mut std::fmt::Formatter<'_>, _: Position) -> std::fmt::Result {
        Ok(())
    }

    fn fmt_text_suffix(&self, _: &mut std::fmt::Formatter<'_>, _: Position) -> std::fmt::Result {
        Ok(())
    }
}
