//! A module which contains configuration of a [`CompactGrid`] which is responsible for grid configuration.
//!
//! [`CompactGrid`]: crate::grid::compact::CompactGrid

use crate::color::Color;
use crate::config::{AlignmentHorizontal, Borders, Indent, Sides};

/// This structure represents a settings of a grid.
///
/// grid: crate::Grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompactConfig {
    borders: Borders<char>,
    border_colors: Borders<Color<'static>>,
    margin: Sides<Indent>,
    margin_color: Sides<Color<'static>>,
    padding: Sides<Indent>,
    padding_color: Sides<Color<'static>>,
    halignment: AlignmentHorizontal,
}

impl Default for CompactConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl CompactConfig {
    /// Returns an standard config.
    pub const fn new() -> Self {
        Self {
            halignment: AlignmentHorizontal::Left,
            borders: Borders::empty(),
            border_colors: Borders::empty(),
            margin: Sides::filled(Indent::zero()),
            margin_color: Sides::filled(Color::new("", "")),
            padding: Sides::new(
                Indent::spaced(1),
                Indent::spaced(1),
                Indent::zero(),
                Indent::zero(),
            ),
            padding_color: Sides::filled(Color::new("", "")),
        }
    }

    /// Set grid margin.
    pub const fn set_margin(mut self, margin: Sides<Indent>) -> Self {
        self.margin = margin;
        self
    }

    /// Returns a grid margin.
    pub const fn get_margin(&self) -> &Sides<Indent> {
        &self.margin
    }

    /// Set the [`Borders`] value as correct one.
    pub const fn set_borders(mut self, borders: Borders<char>) -> Self {
        self.borders = borders;
        self
    }

    /// Returns a current [`Borders`] structure.
    pub const fn get_borders(&self) -> &Borders<char> {
        &self.borders
    }

    /// Returns a current [`Borders`] structure.
    pub const fn get_borders_color(&self) -> &Borders<Color<'static>> {
        &self.border_colors
    }

    /// Set a padding to a given cells.
    pub const fn set_padding(mut self, padding: Sides<Indent>) -> Self {
        self.padding = padding;
        self
    }

    /// Get a padding for a given.
    pub const fn get_padding(&self) -> &Sides<Indent> {
        &self.padding
    }

    /// Set a horizontal alignment.
    pub const fn set_alignment_horizontal(mut self, alignment: AlignmentHorizontal) -> Self {
        self.halignment = alignment;
        self
    }

    /// Get a alignment horizontal.
    pub const fn get_alignment_horizontal(&self) -> AlignmentHorizontal {
        self.halignment
    }

    /// Sets colors of border carcass on the grid.
    pub const fn set_borders_color(mut self, borders: Borders<Color<'static>>) -> Self {
        self.border_colors = borders;
        self
    }

    /// Set colors for a margin.
    pub const fn set_margin_color(mut self, color: Sides<Color<'static>>) -> Self {
        self.margin_color = color;
        self
    }

    /// Returns a margin color.
    pub const fn get_margin_color(&self) -> &Sides<Color<'static>> {
        &self.margin_color
    }

    /// Set a padding to a given cells.
    pub const fn set_padding_color(mut self, color: Sides<Color<'static>>) -> Self {
        self.padding_color = color;
        self
    }

    /// Set a padding to a given cells.
    pub const fn get_padding_color(&self) -> &Sides<Color<'static>> {
        &self.padding_color
    }
}
