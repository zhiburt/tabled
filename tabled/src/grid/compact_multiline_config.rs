use crate::grid::{
    ansi::ANSIStr,
    config::{
        AlignmentHorizontal, AlignmentVertical, Borders, CompactConfig, Formatting, Indent, Sides,
    },
};

/// A [`CompactConfig`] based configuration plus vertical alignment and formatting options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompactMultilineConfig {
    config: CompactConfig,
    alignment_vertical: AlignmentVertical,
    formatting: Formatting,
}

impl CompactMultilineConfig {
    /// Create a new [`CompactMultilineConfig`].
    pub const fn new() -> Self {
        Self {
            config: CompactConfig::new(),
            alignment_vertical: AlignmentVertical::Top,
            formatting: Formatting::new(false, false, false),
        }
    }

    /// Create a new [`CompactMultilineConfig`].
    pub const fn from_compact(config: CompactConfig) -> Self {
        Self {
            config,
            alignment_vertical: AlignmentVertical::Top,
            formatting: Formatting::new(false, false, false),
        }
    }

    /// Set a horizontal alignment.
    pub fn set_alignment_vertical(&mut self, alignment: AlignmentVertical) {
        self.alignment_vertical = alignment
    }

    /// Get a alignment horizontal.
    pub const fn get_alignment_vertical(&self) -> AlignmentVertical {
        self.alignment_vertical
    }

    /// Set grid margin.
    pub fn set_margin(&mut self, margin: Sides<Indent>) {
        self.config = self.config.set_margin(margin);
    }

    /// Returns a grid margin.
    pub const fn get_margin(&self) -> &Sides<Indent> {
        self.config.get_margin()
    }

    /// Set the [`Borders`] value as correct one.
    pub fn set_borders(&mut self, borders: Borders<char>) {
        self.config = self.config.set_borders(borders)
    }

    /// Returns a current [`Borders`] structure.
    pub const fn get_borders(&self) -> &Borders<char> {
        self.config.get_borders()
    }

    /// Returns a current [`Borders`] structure.
    pub const fn get_borders_color(&self) -> &Borders<ANSIStr<'static>> {
        self.config.get_borders_color()
    }

    /// Set a padding to a given cells.
    pub fn set_padding(&mut self, padding: Sides<Indent>) {
        self.config = self.config.set_padding(padding)
    }

    /// Get a padding for a given.
    pub const fn get_padding(&self) -> &Sides<Indent> {
        self.config.get_padding()
    }

    /// Set a horizontal alignment.
    pub fn set_alignment_horizontal(&mut self, alignment: AlignmentHorizontal) {
        self.config = self.config.set_alignment_horizontal(alignment)
    }

    /// Get a alignment horizontal.
    pub const fn get_alignment_horizontal(&self) -> AlignmentHorizontal {
        self.config.get_alignment_horizontal()
    }

    /// Sets colors of border carcass on the grid.
    pub fn set_borders_color(&mut self, borders: Borders<ANSIStr<'static>>) {
        self.config = self.config.set_borders_color(borders)
    }

    /// Set colors for a margin.
    pub fn set_margin_color(&mut self, color: Sides<ANSIStr<'static>>) {
        self.config = self.config.set_margin_color(color)
    }

    /// Returns a margin color.
    pub const fn get_margin_color(&self) -> &Sides<ANSIStr<'static>> {
        self.config.get_margin_color()
    }

    /// Set a padding color to all cells.
    pub fn set_padding_color(&mut self, color: Sides<ANSIStr<'static>>) {
        self.config = self.config.set_padding_color(color)
    }

    /// get a padding color.
    pub const fn get_padding_color(&self) -> &Sides<ANSIStr<'static>> {
        self.config.get_padding_color()
    }

    /// Set formatting.
    pub fn set_formatting(&mut self, formatting: Formatting) {
        self.formatting = formatting
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

impl From<CompactMultilineConfig> for CompactConfig {
    fn from(cfg: CompactMultilineConfig) -> Self {
        cfg.config
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

#[cfg(feature = "std")]
impl From<CompactMultilineConfig> for crate::grid::config::SpannedConfig {
    fn from(compact: CompactMultilineConfig) -> Self {
        use crate::grid::config::Entity::*;
        use crate::grid::config::SpannedConfig;

        let mut cfg = SpannedConfig::from(compact.config);
        cfg.set_alignment_vertical(Global, compact.alignment_vertical);
        cfg.set_formatting(Global, compact.formatting);

        cfg
    }
}
