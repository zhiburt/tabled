//! This module contains a style configuration just like [`RawStyle`] but with [`Color`] by using [`Symbol`].
//!
//! [`Color`]: crate::color::Color
//! [`Symbol`]: crate::style::Symbol
//! [`RawStyle`]: crate::style::RawStyle

use papergrid::{records::Records, AnsiColor, Borders};

use crate::{
    style::{RawStyle, Symbol},
    Table, TableOption,
};

/// A colored [`RawStyle`] versions.
#[cfg_attr(docsrs, doc(cfg(feature = "color")))]
#[derive(Debug, Clone)]
pub struct RawStyleColored {
    style: RawStyle,
    colors: Borders<AnsiColor<'static>>,
}

impl RawStyleColored {
    /// Set a top border character.
    pub fn set_top(&mut self, s: Option<Symbol>) -> &mut Self {
        let c = s.as_ref().map(|s| s.c());
        let color = s.and_then(|s| s.color().cloned().map(|c| c.into()));

        self.style.set_top(c);
        self.colors.top = color;

        self
    }

    /// Set a bottom border character.
    pub fn set_bottom(&mut self, s: Option<Symbol>) -> &mut Self {
        let c = s.as_ref().map(|s| s.c());
        let color = s.and_then(|s| s.color().cloned().map(|c| c.into()));

        self.style.set_bottom(c);
        self.colors.bottom = color;

        self
    }

    /// Set a left border character.
    pub fn set_left(&mut self, s: Option<Symbol>) -> &mut Self {
        let c = s.as_ref().map(|s| s.c());
        let color = s.and_then(|s| s.color().cloned().map(|c| c.into()));

        self.style.set_left(c);
        self.colors.vertical_left = color;

        self
    }

    /// Set a right border character.
    pub fn set_right(&mut self, s: Option<Symbol>) -> &mut Self {
        let c = s.as_ref().map(|s| s.c());
        let color = s.and_then(|s| s.color().cloned().map(|c| c.into()));

        self.style.set_right(c);
        self.colors.vertical_right = color;

        self
    }

    /// Set a top split border character.
    pub fn set_top_split(&mut self, s: Option<Symbol>) -> &mut Self {
        let c = s.as_ref().map(|s| s.c());
        let color = s.and_then(|s| s.color().cloned().map(|c| c.into()));

        self.style.set_top_split(c);
        self.colors.top_intersection = color;

        self
    }

    /// Set a bottom split character.
    pub fn set_bottom_split(&mut self, s: Option<Symbol>) -> &mut Self {
        let c = s.as_ref().map(|s| s.c());
        let color = s.and_then(|s| s.color().cloned().map(|c| c.into()));

        self.style.set_bottom_split(c);
        self.colors.bottom_intersection = color;

        self
    }

    /// Set a left split character.
    pub fn set_left_split(&mut self, s: Option<Symbol>) -> &mut Self {
        let c = s.as_ref().map(|s| s.c());
        let color = s.and_then(|s| s.color().cloned().map(|c| c.into()));

        self.style.set_left_split(c);
        self.colors.horizontal_left = color;

        self
    }

    /// Set a right split character.
    pub fn set_right_split(&mut self, s: Option<Symbol>) -> &mut Self {
        let c = s.as_ref().map(|s| s.c());
        let color = s.and_then(|s| s.color().cloned().map(|c| c.into()));

        self.style.set_right_split(c);
        self.colors.horizontal_right = color;

        self
    }

    /// Set an internal character.
    pub fn set_internal(&mut self, s: Option<Symbol>) -> &mut Self {
        let c = s.as_ref().map(|s| s.c());
        let color = s.and_then(|s| s.color().cloned().map(|c| c.into()));

        self.style.set_internal_split(c);
        self.colors.intersection = color;

        self
    }

    /// Set a vertical character.
    pub fn set_vertical(&mut self, s: Option<Symbol>) -> &mut Self {
        let c = s.as_ref().map(|s| s.c());
        let color = s.and_then(|s| s.color().cloned().map(|c| c.into()));

        self.style.set_vertical(c);
        self.colors.vertical = color;

        self
    }

    /// Set a horizontal character.
    pub fn set_horizontal(&mut self, s: Option<Symbol>) -> &mut Self {
        let c = s.as_ref().map(|s| s.c());
        let color = s.and_then(|s| s.color().cloned().map(|c| c.into()));

        self.style.set_horizontal(c);
        self.colors.horizontal = color;

        self
    }

    /// Set a character for a top left corner.
    pub fn set_top_left(&mut self, s: Option<Symbol>) -> &mut Self {
        let c = s.as_ref().map(|s| s.c());
        let color = s.and_then(|s| s.color().cloned().map(|c| c.into()));

        self.style.set_top_left(c);
        self.colors.top_left = color;

        self
    }

    /// Set a character for a top right corner.
    pub fn set_top_right(&mut self, s: Option<Symbol>) -> &mut Self {
        let c = s.as_ref().map(|s| s.c());
        let color = s.and_then(|s| s.color().cloned().map(|c| c.into()));

        self.style.set_top_right(c);
        self.colors.top_right = color;

        self
    }

    /// Set a character for a bottom left corner.
    pub fn set_bottom_left(&mut self, s: Option<Symbol>) -> &mut Self {
        let c = s.as_ref().map(|s| s.c());
        let color = s.and_then(|s| s.color().cloned().map(|c| c.into()));

        self.style.set_bottom_left(c);
        self.colors.bottom_left = color;

        self
    }

    /// Set a character for a bottom right corner.
    pub fn set_bottom_right(&mut self, s: Option<Symbol>) -> &mut Self {
        let c = s.as_ref().map(|s| s.c());
        let color = s.and_then(|s| s.color().cloned().map(|c| c.into()));

        self.style.set_bottom_right(c);
        self.colors.bottom_right = color;

        self
    }
}

impl<R> TableOption<R> for RawStyleColored
where
    R: Records,
{
    fn change(&mut self, table: &mut Table<R>) {
        self.style.change(table);
        table
            .get_config_mut()
            .set_borders_color(self.colors.clone());
    }
}

impl From<RawStyle> for RawStyleColored {
    fn from(style: RawStyle) -> Self {
        Self {
            style,
            colors: Borders::default(),
        }
    }
}
