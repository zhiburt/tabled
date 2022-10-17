//! A module which contains a [`WidthFunc`] trait and it's implementation [`CfgWidthFunction`]

use crate::{
    util::{string_width_multiline_tab, string_width_tab},
    GridConfig,
};

/// A width function.
pub trait WidthFunc {
    /// Calculates a width of a string.
    fn width(&self, text: &str) -> usize;
    /// Calculates a width of a multiline string.
    fn width_multiline(&self, text: &str) -> usize;
}

impl<W> WidthFunc for &W
where
    W: WidthFunc,
{
    fn width(&self, text: &str) -> usize {
        W::width(self, text)
    }

    fn width_multiline(&self, text: &str) -> usize {
        W::width_multiline(self, text)
    }
}

/// A [`WidthFunc`] implementation which is used by [`Grid`].
///
/// [`Grid`]: crate::Grid
#[derive(Debug, Default, Clone)]
pub struct CfgWidthFunction {
    tab_width: usize,
}

impl CfgWidthFunction {
    /// Creates a [`CfgWidthFunction`] from [`GridConfig`].
    pub fn from_cfg(cfg: &GridConfig) -> Self {
        Self::new(cfg.get_tab_width())
    }

    /// Creates a [`CfgWidthFunction`] with a tab size.
    pub fn new(tab_size: usize) -> Self {
        Self {
            tab_width: tab_size,
        }
    }
}

impl WidthFunc for CfgWidthFunction {
    fn width(&self, text: &str) -> usize {
        string_width_tab(text, self.tab_width)
    }

    fn width_multiline(&self, text: &str) -> usize {
        string_width_multiline_tab(text, self.tab_width)
    }
}
