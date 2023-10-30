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

    /// Get a vertical character.
    pub const fn get_split(&self) -> Option<char> {
        self.line.main
    }

    /// Get a vertical intersection character.
    pub const fn get_intersection(&self) -> Option<char> {
        self.line.intersection
    }

    /// Get a top character.
    pub const fn get_top(&self) -> Option<char> {
        self.line.connector1
    }

    /// Get a bottom character.
    pub const fn get_bottom(&self) -> Option<char> {
        self.line.connector2
    }
}

#[cfg(feature = "std")]
impl<R, D> crate::settings::TableOption<R, D, ColoredConfig> for VerticalLine {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        cfg.insert_vertical_line(self.index, VLine::from(self.line));
    }
}

#[cfg(feature = "std")]
impl<R, D> crate::settings::TableOption<R, D, ColoredConfig> for crate::grid::config::VerticalLine {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        let mut borders = *cfg.get_borders();
        borders.vertical = self.main;
        borders.top_intersection = self.top;
        borders.bottom_intersection = self.bottom;
        borders.intersection = self.intersection;

        cfg.set_borders(borders);
    }
}

impl<R, D> crate::settings::TableOption<R, D, crate::grid::config::CompactMultilineConfig>
    for crate::grid::config::VerticalLine
{
    fn change(self, _: &mut R, cfg: &mut crate::grid::config::CompactMultilineConfig, _: &mut D) {
        let mut borders = *cfg.get_borders();
        borders.vertical = self.main;
        borders.top_intersection = self.top;
        borders.bottom_intersection = self.bottom;
        borders.intersection = self.intersection;

        *cfg = cfg.set_borders(borders);
    }
}

impl<R, D> crate::settings::TableOption<R, D, crate::grid::config::CompactConfig>
    for crate::grid::config::VerticalLine
{
    fn change(self, _: &mut R, cfg: &mut crate::grid::config::CompactConfig, _: &mut D) {
        let mut borders = *cfg.get_borders();
        borders.vertical = self.main;
        borders.top_intersection = self.top;
        borders.bottom_intersection = self.bottom;
        borders.intersection = self.intersection;

        *cfg = cfg.set_borders(borders);
    }
}
