use crate::{
    grid::config::{GridConfig, HorizontalLine as GridLine},
    settings::TableOption,
};

use super::Line;

/// A horizontal split line which can be used to set a border.
#[derive(Debug, Clone)]
pub struct HorizontalLine {
    pub(crate) index: usize,
    pub(crate) line: Option<Line>,
}

impl HorizontalLine {
    /// Creates a new horizontal split line.
    pub const fn new(index: usize, line: Line) -> Self {
        Self {
            index,
            line: Some(line),
        }
    }

    /// Removes an existing split line by index.
    ///
    /// It not present or in case of index bigger than the count of columns it has no affect.
    pub fn empty(index: usize) -> Self {
        Self { index, line: None }
    }

    /// Sets a horizontal character.
    pub const fn main(mut self, c: Option<char>) -> Self {
        let mut line = match self.line {
            Some(line) => line,
            None => Line::empty(),
        };

        line.main = c;
        self.line = Some(line);

        self
    }

    /// Sets a vertical intersection character.
    pub const fn intersection(mut self, c: Option<char>) -> Self {
        let mut line = match self.line {
            Some(line) => line,
            None => Line::empty(),
        };

        line.intersection = c;
        self.line = Some(line);

        self
    }

    /// Sets a left character.
    pub const fn left(mut self, c: Option<char>) -> Self {
        let mut line = match self.line {
            Some(line) => line,
            None => Line::empty(),
        };

        line.connector1 = c;
        self.line = Some(line);

        self
    }

    /// Sets a right character.
    pub const fn right(mut self, c: Option<char>) -> Self {
        let mut line = match self.line {
            Some(line) => line,
            None => Line::empty(),
        };

        line.connector2 = c;
        self.line = Some(line);

        self
    }

    /// Checks if it's an empty line.
    pub const fn is_empty(&self) -> bool {
        match &self.line {
            Some(l) => l.is_empty(),
            None => true,
        }
    }
}

impl<R, D> TableOption<R, D> for HorizontalLine {
    fn change(&mut self, _: &mut R, cfg: &mut GridConfig, _: &mut D) {
        match &self.line {
            Some(line) => cfg.set_horizontal_line(self.index, GridLine::from(*line)),
            None => cfg.remove_horizontal_line(self.index),
        }
    }
}
