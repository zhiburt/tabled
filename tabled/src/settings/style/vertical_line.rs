#[cfg(feature = "std")]
use crate::grid::config::{ColoredConfig, VerticalLine as VLine};

use super::Line;

/// A horizontal split line which can be used to set a border.
#[cfg_attr(not(feature = "std"), allow(dead_code))]
#[derive(Debug, Clone)]
pub struct VerticalLine {
    pub(crate) index: usize,
    pub(crate) line: Line,
}

impl VerticalLine {
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

    /// Sets a top character.
    pub const fn top(mut self, c: Option<char>) -> Self {
        self.line.connector1 = c;
        self
    }

    /// Sets a bottom character.
    pub const fn bottom(mut self, c: Option<char>) -> Self {
        self.line.connector2 = c;
        self
    }
}

#[cfg(feature = "std")]
impl<R, D> crate::settings::TableOption<R, D, ColoredConfig> for VerticalLine {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        cfg.insert_vertical_line(self.index, VLine::from(self.line));
    }
}
