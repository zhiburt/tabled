//! A module which contains [GridConfig] which is responsible for grid configuration.

use crate::color::StaticColor;

use crate::config::{AlignmentHorizontal, AlignmentVertical, Borders, Indent, Sides};

/// This structure represents a settings of a grid.
///
/// grid: crate::Grid.
#[derive(Debug, Clone, Copy)]
pub struct CompactConfig {
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

impl Default for CompactConfig {
    fn default() -> Self {
        Self::empty()
    }
}

impl CompactConfig {
    /// Returns an standart config.
    pub const fn empty() -> Self {
        Self {
            tab_width: 4,
            halignment: AlignmentHorizontal::Left,
            valignment: AlignmentVertical::Top,
            borders: Borders::empty(),
            border_colors: Borders::empty(),
            margin: Sides::filled(Indent::zero()),
            margin_color: Sides::filled(StaticColor::new("", "")),
            padding: Sides::filled(Indent::zero()),
            padding_color: Sides::filled(StaticColor::new("", "")),
        }
    }

    /// Set [`Margin`].
    pub const fn set_margin(mut self, margin: Sides<Indent>) {
        self.margin = margin;
    }

    /// Returns a [`Margin`].
    pub const fn get_margin(&self) -> &Sides<Indent> {
        &self.margin
    }

    /// Set the [`Borders`] value as currect one.
    pub const fn set_borders(mut self, borders: Borders<char>) -> Self {
        self.borders = borders;
        self
    }

    /// Set tab width in spaces.
    pub const fn set_tab_width(mut self, width: usize) -> Self {
        self.tab_width = width;
        self
    }

    /// Get tab width value in spaces.
    pub const fn get_tab_width(&self) -> usize {
        self.tab_width
    }

    /// Returns a current [`Borders`] structure.
    pub const fn get_borders(&self) -> &Borders<char> {
        &self.borders
    }

    /// Returns a current [`Borders`] structure.
    pub const fn get_borders_color(&self) -> &Borders<StaticColor> {
        &self.border_colors
    }

    /// Set a padding to a given cells.
    pub const fn set_padding(mut self, padding: Sides<Indent>) -> Self {
        self.padding = padding;
        self
    }

    /// Get a padding for a given [Entity].
    pub const fn get_padding(&self) -> &Sides<Indent> {
        &self.padding
    }

    /// Set a horizontal alignment to a given cells.
    pub const fn set_alignment_horizontal(mut self, alignment: AlignmentHorizontal) -> Self {
        self.halignment = alignment;
        self
    }

    /// Get a alignment horizontal.
    pub const fn get_alignment_horizontal(&self) -> AlignmentHorizontal {
        self.halignment
    }

    /// Set a horizontal alignment to a given cells.
    pub const fn set_alignment_vertical(mut self, alignment: AlignmentVertical) -> Self {
        self.valignment = alignment;
        self
    }

    /// Get a alignment vertical.
    pub const fn get_alignment_vertical(&self) -> AlignmentVertical {
        self.valignment
    }

    /// Sets colors of border carcass on the grid.
    pub const fn set_borders_color(mut self, borders: Borders<StaticColor>) -> Self {
        self.border_colors = borders;
        self
    }

    /// Set colors for a [`Margin`] value.
    pub const fn set_margin_color(mut self, color: Sides<StaticColor>) -> Self {
        self.margin_color = color;
        self
    }

    /// Returns a [`Margin`] offset.
    pub const fn get_margin_color(&self) -> &Sides<StaticColor> {
        &self.margin_color
    }

    /// Set a padding to a given cells.
    pub const fn set_padding_color(mut self, color: Sides<StaticColor>) -> Self {
        self.padding_color = color;
        self
    }

    /// Set a padding to a given cells.
    pub const fn get_padding_color(&self) -> Sides<StaticColor> {
        self.padding_color
    }
}

impl From<&CompactConfig> for CompactConfig {
    fn from(value: &CompactConfig) -> Self {
        *value
    }
}
