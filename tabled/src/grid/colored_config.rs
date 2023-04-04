use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use papergrid::{
    color::AnsiColor,
    config::{spanned::SpannedConfig, Position},
};

/// A [`Table`] configuration.
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct ColoredConfig {
    config: SpannedConfig,
    colors: HashMap<Position, AnsiColor<'static>>,
}

impl ColoredConfig {
    /// Create a new colored config.
    pub fn new(config: SpannedConfig, colors: HashMap<Position, AnsiColor<'static>>) -> Self {
        Self { config, colors }
    }

    /// Set a color for a given cell.
    ///
    /// The outcome is the same as if you'd use [`Format`] and added a color but it'd work only with `color` feature on.
    /// While this method works in all contexts.
    pub fn set_color(&mut self, pos: Position, color: AnsiColor<'static>) -> &mut Self {
        let _ = self.colors.insert(pos, color);
        self
    }

    /// Returns a list of colors.
    pub fn get_colors(&self) -> &HashMap<Position, AnsiColor<'static>> {
        &self.colors
    }
}

impl Deref for ColoredConfig {
    type Target = SpannedConfig;

    fn deref(&self) -> &Self::Target {
        &self.config
    }
}

impl DerefMut for ColoredConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.config
    }
}

impl From<SpannedConfig> for ColoredConfig {
    fn from(value: SpannedConfig) -> Self {
        Self::new(value, Default::default())
    }
}

impl AsRef<SpannedConfig> for ColoredConfig {
    fn as_ref(&self) -> &SpannedConfig {
        &self.config
    }
}
