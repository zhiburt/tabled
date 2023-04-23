//! This module contains a [`Shadow`] option for a [`Table`].
//!
//! # Example
//!
//! ```
//! use tabled::{Table, settings::{Shadow, Style}};
//!
//! let data = vec!["Hello", "World", "!"];
//!
//! let table = Table::new(data)
//!     .with(Style::markdown())
//!     .with(Shadow::new(1))
//!     .to_string();
//!
//! assert_eq!(
//!     table,
//!     concat!(
//!         "| &str  | \n",
//!         "|-------|▒\n",
//!         "| Hello |▒\n",
//!         "| World |▒\n",
//!         "| !     |▒\n",
//!         " ▒▒▒▒▒▒▒▒▒",
//!     )
//! );
//! ```
//!
//! [`Table`]: crate::Table

use crate::{
    grid::color::AnsiColor,
    grid::config::{ColoredConfig, Indent, Offset, Sides},
    settings::{color::Color, TableOption},
};

/// The structure represents a shadow of a table.
///
/// NOTICE: It uses [`Margin`] therefore it often can't be combined.
///
/// [`Margin`]: crate::settings::Margin
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Shadow {
    c: char,
    size: usize,
    size_offset: usize,
    direction: Sides<bool>,
    color: Option<Color>,
}

impl Shadow {
    /// A default fill character to be used.
    pub const DEFAULT_FILL: char = '▒';

    /// Construct's an [`Shadow`] object with default fill [`Shadow::DEFAULT_FILL`].
    ///
    /// It uses space(' ') as a default fill character.
    /// To set a custom character you can use [`Self::set_fill`] function.
    pub fn new(size: usize) -> Self {
        Self {
            c: Self::DEFAULT_FILL,
            size,
            size_offset: 1,
            direction: Sides::new(false, true, false, true),
            color: None,
        }
    }

    /// The function, sets a characters for the [`Shadow`] to be used.
    pub fn set_fill(mut self, c: char) -> Self {
        self.c = c;
        self
    }

    /// Set an offset value (default is '1').
    pub fn set_offset(mut self, size: usize) -> Self {
        self.size_offset = size;
        self
    }

    /// Switch shadow to top.
    pub fn set_top(mut self) -> Self {
        self.direction.top = true;
        self.direction.bottom = false;
        self
    }

    /// Switch shadow to bottom.
    pub fn set_bottom(mut self) -> Self {
        self.direction.bottom = true;
        self.direction.top = false;
        self
    }

    /// Switch shadow to left.
    pub fn set_left(mut self) -> Self {
        self.direction.left = true;
        self.direction.right = false;
        self
    }

    /// Switch shadow to right.
    pub fn set_right(mut self) -> Self {
        self.direction.right = true;
        self.direction.left = false;
        self
    }

    /// Sets a color for a shadow.
    pub fn set_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}

impl<R, D> TableOption<R, D, ColoredConfig> for Shadow {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        set_margin(cfg, self.size, self.c, &self.direction);
        set_margin_offset(cfg, self.size_offset, &self.direction);

        if let Some(color) = &self.color {
            set_margin_color(cfg, color.clone().into(), &self.direction);
        }
    }
}

fn set_margin(cfg: &mut ColoredConfig, size: usize, c: char, direction: &Sides<bool>) {
    let mut margin: Sides<Indent> = Sides::default();
    if direction.top {
        margin.top.size = size;
        margin.top.fill = c;
    }

    if direction.bottom {
        margin.bottom.size = size;
        margin.bottom.fill = c;
    }

    if direction.left {
        margin.left.size = size;
        margin.left.fill = c;
    }

    if direction.right {
        margin.right.size = size;
        margin.right.fill = c;
    }

    cfg.set_margin(margin);
}

fn set_margin_offset(cfg: &mut ColoredConfig, size: usize, direction: &Sides<bool>) {
    let mut margin = Sides::filled(Offset::Begin(0));
    if direction.right && direction.bottom {
        margin.bottom = Offset::Begin(size);
        margin.right = Offset::Begin(size);
    }

    if direction.right && direction.top {
        margin.top = Offset::Begin(size);
        margin.right = Offset::End(size);
    }

    if direction.left && direction.bottom {
        margin.bottom = Offset::End(size);
        margin.left = Offset::Begin(size);
    }

    if direction.left && direction.top {
        margin.top = Offset::End(size);
        margin.left = Offset::End(size);
    }

    cfg.set_margin_offset(margin);
}

fn set_margin_color(cfg: &mut ColoredConfig, color: AnsiColor<'static>, direction: &Sides<bool>) {
    let mut margin: Sides<Option<AnsiColor<'static>>> = Sides::default();
    if direction.right {
        margin.right = Some(color.clone());
    }

    if direction.top {
        margin.top = Some(color.clone());
    }

    if direction.left {
        margin.left = Some(color.clone());
    }

    if direction.bottom {
        margin.bottom = Some(color.clone());
    }

    cfg.set_margin_color(margin);
}
