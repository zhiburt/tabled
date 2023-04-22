use crate::grid::color::StaticColor;
use crate::grid::config::{
    AlignmentHorizontal, AlignmentVertical, Borders, CompactConfig, Indent, Line, Sides,
};

/// A [`CompactConfig`] configuration plus vertical alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompactMultilineConfig {
    config: CompactConfig,
    alignment_vertical: AlignmentVertical,
    formatting: Formatting,
}

impl CompactMultilineConfig {
    /// Create a new colored config.
    pub fn new(config: CompactConfig) -> Self {
        Self::from(config)
    }

    /// Set a horizontal alignment.
    pub const fn set_alignment_vertical(mut self, alignment: AlignmentVertical) -> Self {
        self.alignment_vertical = alignment;
        self
    }

    /// Get a alignment horizontal.
    pub const fn get_alignment_vertical(&self) -> AlignmentVertical {
        self.alignment_vertical
    }

    /// Set grid margin.
    pub const fn set_margin(mut self, margin: Sides<Indent>) -> Self {
        self.config = self.config.set_margin(margin);
        self
    }

    /// Returns a grid margin.
    pub const fn get_margin(&self) -> &Sides<Indent> {
        self.config.get_margin()
    }

    /// Set the [`Borders`] value as correct one.
    pub const fn set_borders(mut self, borders: Borders<char>) -> Self {
        self.config = self.config.set_borders(borders);
        self
    }

    /// Set the first horizontal line.
    ///
    /// It ignores the [`Borders`] horizontal value if set for 1st row.
    pub const fn set_first_horizontal_line(mut self, line: Line<char>) -> Self {
        self.config = self.config.set_first_horizontal_line(line);
        self
    }

    /// Set the first horizontal line.
    ///
    /// It ignores the [`Borders`] horizontal value if set for 1st row.
    pub const fn get_first_horizontal_line(&self) -> Option<Line<char>> {
        self.config.get_first_horizontal_line()
    }

    /// Returns a current [`Borders`] structure.
    pub const fn get_borders(&self) -> &Borders<char> {
        self.config.get_borders()
    }

    /// Returns a current [`Borders`] structure.
    pub const fn get_borders_color(&self) -> &Borders<StaticColor> {
        self.config.get_borders_color()
    }

    /// Set a padding to a given cells.
    pub const fn set_padding(mut self, padding: Sides<Indent>) -> Self {
        self.config = self.config.set_padding(padding);
        self
    }

    /// Get a padding for a given.
    pub const fn get_padding(&self) -> &Sides<Indent> {
        self.config.get_padding()
    }

    /// Set a horizontal alignment.
    pub const fn set_alignment_horizontal(mut self, alignment: AlignmentHorizontal) -> Self {
        self.config = self.config.set_alignment_horizontal(alignment);
        self
    }

    /// Get a alignment horizontal.
    pub const fn get_alignment_horizontal(&self) -> AlignmentHorizontal {
        self.config.get_alignment_horizontal()
    }

    /// Sets colors of border carcass on the grid.
    pub const fn set_borders_color(mut self, borders: Borders<StaticColor>) -> Self {
        self.config = self.config.set_borders_color(borders);
        self
    }

    /// Set colors for a margin.
    pub const fn set_margin_color(mut self, color: Sides<StaticColor>) -> Self {
        self.config = self.config.set_margin_color(color);
        self
    }

    /// Returns a margin color.
    pub const fn get_margin_color(&self) -> Sides<StaticColor> {
        self.config.get_margin_color()
    }

    /// Set a padding color to all cells.
    pub const fn set_padding_color(mut self, color: Sides<StaticColor>) -> Self {
        self.config = self.config.set_padding_color(color);
        self
    }

    /// get a padding color.
    pub const fn get_padding_color(&self) -> Sides<StaticColor> {
        self.config.get_padding_color()
    }

    /// Set formatting.
    pub const fn set_formatting(mut self, formatting: Formatting) -> Self {
        self.formatting = formatting;
        self
    }

    /// Get formatting.
    pub const fn get_formatting(&self) -> Formatting {
        self.formatting
    }
}

impl Default for CompactMultilineConfig {
    fn default() -> Self {
        Self {
            config: Default::default(),
            alignment_vertical: AlignmentVertical::Top,
            formatting: Formatting::default(),
        }
    }
}

impl From<CompactConfig> for CompactMultilineConfig {
    fn from(config: CompactConfig) -> Self {
        Self {
            config,
            alignment_vertical: AlignmentVertical::Top,
            formatting: Formatting::default(),
        }
    }
}

impl AsRef<CompactConfig> for CompactMultilineConfig {
    fn as_ref(&self) -> &CompactConfig {
        &self.config
    }
}

impl AsMut<CompactConfig> for CompactMultilineConfig {
    fn as_mut(&mut self) -> &mut CompactConfig {
        &mut self.config
    }
}

#[cfg(feature = "std")]
impl From<CompactMultilineConfig> for crate::grid::config::SpannedConfig {
    fn from(compact: CompactMultilineConfig) -> Self {
        use crate::grid::config::Entity;

        let mut cfg = crate::grid::config::SpannedConfig::from(compact.config);
        cfg.set_alignment_vertical(Entity::Global, compact.alignment_vertical);
        cfg.set_formatting(Entity::Global, compact.formatting.into());

        cfg
    }
}

/// Formatting represent a logic of formatting of a cell.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Formatting {
    /// An setting to allow horizontal trim.
    pub horizontal_trim: bool,
    /// An setting to allow vertical trim.
    pub vertical_trim: bool,
    /// An setting to allow alignment per line.
    pub allow_lines_alignment: bool,
}

impl Formatting {
    /// Creates a new [`Formatting`] structure.
    pub fn new(horizontal_trim: bool, vertical_trim: bool, allow_lines_alignment: bool) -> Self {
        Self {
            horizontal_trim,
            vertical_trim,
            allow_lines_alignment,
        }
    }
}

#[cfg(feature = "std")]
impl From<Formatting> for crate::grid::config::Formatting {
    fn from(val: Formatting) -> Self {
        crate::grid::config::Formatting {
            allow_lines_alignment: val.allow_lines_alignment,
            horizontal_trim: val.horizontal_trim,
            vertical_trim: val.vertical_trim,
        }
    }
}
