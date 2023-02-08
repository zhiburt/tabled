//! This module contains a [`Shadow`] option for a [`Table`].
//!
//! # Example
//!
//! ```
//! use tabled::{Style, TableIteratorExt, shadow::Shadow};
//!
//! let data = vec!["Hello", "World", "!"];
//!
//! let table = data.table()
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
    grid::config::{Offset, Sides},
    settings::{color::Color, TableOption},
};

/// The structure represents a shadow of a table.
///
/// NOTICE: It uses [`Margin`] therefore it often can't be combined.
///
/// [`Margin`]: crate::Margin
#[derive(Debug, Clone)]
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
    pub fn set_fill(&mut self, c: char) -> &mut Self {
        self.c = c;
        self
    }

    /// Set an offset value (default is '1').
    pub fn set_offset(&mut self, size: usize) -> &mut Self {
        self.size_offset = size;
        self
    }

    /// Switch shadow to top.
    pub fn set_top(&mut self) -> &mut Self {
        self.direction.top = true;
        self.direction.bottom = false;
        self
    }

    /// Switch shadow to bottom.
    pub fn set_bottom(&mut self) -> &mut Self {
        self.direction.bottom = true;
        self.direction.top = false;
        self
    }

    /// Switch shadow to left.
    pub fn set_left(&mut self) -> &mut Self {
        self.direction.left = true;
        self.direction.right = false;
        self
    }

    /// Switch shadow to right.
    pub fn set_right(&mut self) -> &mut Self {
        self.direction.right = true;
        self.direction.left = false;
        self
    }

    /// Sets a color for a shadow.
    pub fn set_color(&mut self, color: Color) -> &mut Self {
        self.color = Some(color);
        self
    }
}

impl<R, D> TableOption<R, D> for Shadow {
    fn change(&mut self, records: &mut R, cfg: &mut papergrid::GridConfig, dimension: &mut D) {
        let mut margin = *cfg.get_margin();

        if self.direction.top {
            margin.top.size = self.size;
            margin.top.fill = self.c;
        }

        if self.direction.bottom {
            margin.bottom.size = self.size;
            margin.bottom.fill = self.c;
        }

        if self.direction.left {
            margin.left.size = self.size;
            margin.left.fill = self.c;
        }

        if self.direction.right {
            margin.right.size = self.size;
            margin.right.fill = self.c;
        }

        let mut offset = Sides::new(
            Offset::Begin(0),
            Offset::Begin(0),
            Offset::Begin(0),
            Offset::Begin(0),
        );

        if self.direction.right && self.direction.bottom {
            offset.bottom = Offset::Begin(self.size_offset);
            offset.right = Offset::Begin(self.size_offset);
        }

        if self.direction.right && self.direction.top {
            offset.top = Offset::Begin(self.size_offset);
            offset.right = Offset::End(self.size_offset);
        }

        if self.direction.left && self.direction.bottom {
            offset.bottom = Offset::End(self.size_offset);
            offset.left = Offset::Begin(self.size_offset);
        }

        if self.direction.left && self.direction.top {
            offset.top = Offset::End(self.size_offset);
            offset.left = Offset::End(self.size_offset);
        }

        cfg.set_margin(margin);
        cfg.set_margin_offset(offset);

        if let Some(color) = self.color.as_ref() {
            let mut colors = Sides::default();
            if self.direction.top {
                colors.top = color.clone().into();
            }
            if self.direction.bottom {
                colors.bottom = color.clone().into();
            }
            if self.direction.left {
                colors.left = color.clone().into();
            }
            if self.direction.right {
                colors.right = color.clone().into();
            }

            cfg.set_margin_color(colors);
        }
    }
}
