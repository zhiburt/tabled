use crate::{
    grid::config::{CompactConfig, CompactMultilineConfig},
    settings::TableOption,
};

#[cfg(feature = "std")]
use crate::grid::config::{ColoredConfig, HorizontalLine as GridLine};

use super::Line;

/// A horizontal split line which can be used to set a border.
#[cfg_attr(not(feature = "std"), allow(dead_code))]
#[derive(Debug, Clone)]
pub struct HorizontalLine {
    pub(super) index: usize,
    pub(super) line: Line,
}

impl HorizontalLine {
    /// Creates a new horizontal split line.
    pub const fn new(index: usize, line: Line) -> Self {
        Self { index, line }
    }

    /// Sets a horizontal character.
    pub const fn main(mut self, c: Option<char>) -> Self {
        self.line.main = c;
        self
    }

    /// Sets a vertical intersection character.
    pub const fn intersection(mut self, c: Option<char>) -> Self {
        self.line.intersection = c;
        self
    }

    /// Sets a left character.
    pub const fn left(mut self, c: Option<char>) -> Self {
        self.line.connector1 = c;
        self
    }

    /// Sets a right character.
    pub const fn right(mut self, c: Option<char>) -> Self {
        self.line.connector2 = c;
        self
    }
}

#[cfg(feature = "std")]
impl<R, D> TableOption<R, D, ColoredConfig> for HorizontalLine {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        cfg.insert_horizontal_line(self.index, GridLine::from(self.line))
    }
}

impl<R, D> TableOption<R, D, CompactConfig> for HorizontalLine {
    fn change(self, _: &mut R, cfg: &mut CompactConfig, _: &mut D) {
        if self.index == 1 {
            *cfg = cfg.set_first_horizontal_line(papergrid::config::Line::from(self.line));
        }
    }
}

impl<R, D> TableOption<R, D, CompactMultilineConfig> for HorizontalLine {
    fn change(self, records: &mut R, cfg: &mut CompactMultilineConfig, dimension: &mut D) {
        self.change(records, cfg.as_mut(), dimension)
    }
}
