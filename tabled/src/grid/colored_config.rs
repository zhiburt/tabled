use std::ops::{Deref, DerefMut};

use crate::grid::{
    color::AnsiColor,
    config::{Entity, EntityMap, SpannedConfig},
};

/// A spanned configuration plus colors for cells.
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct ColoredConfig {
    config: SpannedConfig,
    colors: ColorMap,
}

impl ColoredConfig {
    /// Create a new colored config.
    pub fn new(config: SpannedConfig) -> Self {
        Self {
            config,
            colors: ColorMap::default(),
        }
    }

    /// Set a color for a given cell.
    ///
    /// The outcome is the same as if you'd use [`Format`] and added a color but it'd work only with `color` feature on.
    /// While this method works in all contexts.
    ///
    /// [`Format`]: crate::settings::Format
    pub fn set_color(&mut self, pos: Entity, color: AnsiColor<'static>) -> &mut Self {
        match self.colors.0.as_mut() {
            Some(map) => map.insert(pos, color),
            None => {
                let mut colors = EntityMap::default();
                colors.insert(pos, color);
                self.colors = ColorMap(Some(colors));
            }
        }

        self
    }

    /// Set a list of colors.
    pub fn set_colors(&mut self, colors: EntityMap<AnsiColor<'static>>) -> &mut Self {
        self.colors = ColorMap(Some(colors));
        self
    }

    /// Remove a color for a given cell.
    pub fn remove_color(&mut self, pos: Entity) -> &mut Self {
        if let Some(colors) = self.colors.0.as_mut() {
            colors.remove(pos);
        }

        self
    }

    /// Returns a list of colors.
    pub fn get_colors(&self) -> &ColorMap {
        &self.colors
    }

    /// Returns an inner config.
    pub fn into_inner(self) -> SpannedConfig {
        self.config
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
        Self::new(value)
    }
}

impl AsRef<SpannedConfig> for ColoredConfig {
    fn as_ref(&self) -> &SpannedConfig {
        &self.config
    }
}

/// A colors structure for [`ColoredConfig`].
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct ColorMap(Option<EntityMap<AnsiColor<'static>>>);

impl ColorMap {
    /// Checks if any colors is set on.
    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }
}

impl crate::grid::colors::Colors for ColorMap {
    type Color = AnsiColor<'static>;

    fn get_color(&self, (row, col): (usize, usize)) -> Option<&Self::Color> {
        self.0.as_ref().map(|map| map.get(Entity::Cell(row, col)))
    }
}
