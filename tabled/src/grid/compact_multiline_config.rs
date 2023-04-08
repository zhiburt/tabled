use core::ops::{Deref, DerefMut};

use papergrid::config::compact::CompactConfig;
use papergrid::config::AlignmentVertical;

/// A [`Table`] configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompactMultilineConfig {
    config: CompactConfig,
    alignment_vertical: AlignmentVertical,
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
}

impl From<CompactConfig> for CompactMultilineConfig {
    fn from(config: CompactConfig) -> Self {
        Self {
            config,
            alignment_vertical: AlignmentVertical::Top,
        }
    }
}

impl Deref for CompactMultilineConfig {
    type Target = CompactConfig;

    fn deref(&self) -> &Self::Target {
        &self.config
    }
}

impl DerefMut for CompactMultilineConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.config
    }
}

impl AsRef<CompactConfig> for CompactMultilineConfig {
    fn as_ref(&self) -> &CompactConfig {
        &self.config
    }
}

#[cfg(feature = "std")]
impl From<CompactMultilineConfig> for crate::grid::config::SpannedConfig {
    fn from(compact: CompactMultilineConfig) -> Self {
        use crate::grid::config::Entity;

        let mut cfg = crate::grid::config::SpannedConfig::from(compact.config);
        cfg.set_alignment_vertical(Entity::Global, compact.alignment_vertical);
        cfg
    }
}
