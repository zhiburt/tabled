//! A module which contains [GridConfig] which is responsible for grid configuration.

use crate::color::StaticColor;

use crate::config::{AlignmentHorizontal, AlignmentVertical, Borders, Indent, Sides};

/// This structure represents a settings of a grid.
///
/// grid: crate::Grid.
#[derive(Debug, Clone, Copy)]
pub struct CommonConfig {
    borders: Borders<char>,
    border_colors: Borders<StaticColor>,
    margin: Sides<Indent>,
    margin_color: Sides<StaticColor>,
    padding: Sides<Indent>,
    padding_color: Sides<StaticColor>,
    halignment: AlignmentHorizontal,
    valignment: AlignmentVertical,
    tab_width: usize,
}

impl Default for CommonConfig {
    fn default() -> Self {
        Self {
            tab_width: 4,
            halignment: AlignmentHorizontal::Left,
            valignment: AlignmentVertical::Top,
            borders: Borders::default(),
            border_colors: Borders::default(),
            margin: Sides::default(),
            margin_color: Sides::default(),
            padding: Sides::default(),
            padding_color: Sides::default(),
        }
    }
}

impl CommonConfig {
    /// Set [`Margin`].
    pub fn set_margin(&mut self, margin: Sides<Indent>) {
        self.margin = margin;
    }

    /// Returns a [`Margin`].
    pub fn get_margin(&self) -> &Sides<Indent> {
        &self.margin
    }

    /// Set the [`Borders`] value as currect one.
    pub fn set_borders(&mut self, borders: Borders<char>) {
        self.borders = borders;
    }

    /// Set tab width in spaces.
    pub fn set_tab_width(&mut self, width: usize) {
        self.tab_width = width;
    }

    /// Get tab width value in spaces.
    pub fn get_tab_width(&self) -> usize {
        self.tab_width
    }

    /// Returns a current [`Borders`] structure.
    pub fn get_borders(&self) -> &Borders<char> {
        &self.borders
    }

    /// Returns a current [`Borders`] structure.
    pub fn get_borders_color(&self) -> &Borders<StaticColor> {
        &self.border_colors
    }

    /// Set a padding to a given cells.
    pub fn set_padding(&mut self, padding: Sides<Indent>) {
        self.padding = padding;
    }

    /// Get a padding for a given [Entity].
    pub fn get_padding(&self) -> &Sides<Indent> {
        &self.padding
    }

    /// Set a horizontal alignment to a given cells.
    pub fn set_alignment_horizontal(&mut self, alignment: AlignmentHorizontal) {
        self.halignment = alignment;
    }

    pub fn get_alignment_horizontal(&self) -> AlignmentHorizontal {
        self.halignment
    }

    /// Set a horizontal alignment to a given cells.
    pub fn set_alignment_vertical(&mut self, alignment: AlignmentVertical) {
        self.valignment = alignment;
    }

    pub fn get_alignment_vertical(&self) -> AlignmentVertical {
        self.valignment
    }

    /// Sets colors of border carcass on the grid.
    pub fn set_borders_color(&mut self, borders: Borders<StaticColor>) {
        self.border_colors = borders;
    }

    /// Set colors for a [`Margin`] value.
    pub fn set_margin_color(&mut self, color: Sides<StaticColor>) {
        self.margin_color = color;
    }

    /// Returns a [`Margin`] offset.
    pub fn get_margin_color(&self) -> &Sides<StaticColor> {
        &self.margin_color
    }

    /// Set a padding to a given cells.
    pub fn set_padding_color(&mut self, color: Sides<StaticColor>) {
        self.padding_color = color;
    }

    /// Set a padding to a given cells.
    pub fn get_padding_color(&self) -> Sides<StaticColor> {
        self.padding_color
    }
}

impl From<&CommonConfig> for CommonConfig {
    fn from(value: &CommonConfig) -> Self {
        *value
    }
}
