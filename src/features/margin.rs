//! This module contains a Margin settings of a [`Table`].
//!
//! # Example
//!
//! ```
//! use tabled::{Margin, Style, TableIteratorExt};
//!
//! let data = vec!["Hello", "World", "!"];
//!
//! let mut table = data.table();
//! table.with(Style::markdown()).with(Margin::new(3, 3, 1, 0));
//!
//! assert_eq!(
//!     table.to_string(),
//!     concat!(
//!         "               \n",
//!         "   | &str  |   \n",
//!         "   |-------|   \n",
//!         "   | Hello |   \n",
//!         "   | World |   \n",
//!         "   | !     |   ",
//!     )
//! );
//! ```
//!
//! [`Table`]: crate::Table

use papergrid::Indent;

use crate::{Table, TableOption};

/// Margin is responsible for a left/right/top/bottom outer indent of a grid.
///
/// ```rust,no_run
/// # use tabled::{Margin, Table};
/// # let data: Vec<&'static str> = Vec::new();
/// let table = Table::new(&data).with(Margin::new(1, 1, 1, 1).set_fill('>', '<', 'V', '^'));
/// ```
#[derive(Debug, Clone)]
pub struct Margin(papergrid::Margin);

impl Margin {
    /// Construct's an Margin object.
    ///
    /// It uses space(' ') as a default fill character.
    /// To set a custom character you can use [`Self::set_fill`] function.
    pub fn new(left: usize, right: usize, top: usize, bottom: usize) -> Self {
        Self(papergrid::Margin {
            top: Indent::spaced(top),
            bottom: Indent::spaced(bottom),
            left: Indent::spaced(left),
            right: Indent::spaced(right),
        })
    }

    /// The function, sets a characters for the margin on an each side.
    pub fn set_fill(mut self, left: char, right: char, top: char, bottom: char) -> Self {
        self.0.left.fill = left;
        self.0.right.fill = right;
        self.0.top.fill = top;
        self.0.bottom.fill = bottom;
        self
    }
}

impl<R> TableOption<R> for Margin {
    fn change(&mut self, table: &mut Table<R>) {
        table.get_config_mut().set_margin(self.0);
    }
}
