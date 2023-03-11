//! This module contains primitives to create a spread row.
//! Ultimately it is a cell with a span set to a number of columns on the [`Table`].
//!
//! You can use a [`Span`] to set a custom span.
//!
//! # Example
//!
//! ```
//! use tabled::{Table, settings::Panel};
//!
//! let data = [[1, 2, 3], [4, 5, 6]];
//!
//! let table = Table::new(data)
//!     .with(Panel::vertical(1, "S\np\nl\ni\nt"))
//!     .with(Panel::header("Numbers"))
//!     .to_string();
//!
//! println!("{}", table);
//!
//! assert_eq!(
//!     table,
//!     concat!(
//!         "+---+---+---+---+\n",
//!         "| Numbers       |\n",
//!         "+---+---+---+---+\n",
//!         "| 0 | S | 1 | 2 |\n",
//!         "+---+ p +---+---+\n",
//!         "| 1 | l | 2 | 3 |\n",
//!         "+---+ i +---+---+\n",
//!         "| 4 | t | 5 | 6 |\n",
//!         "+---+---+---+---+",
//!     )
//! )
//! ```
//!
//! [`Table`]: crate::Table
//! [`Span`]: crate::settings::span::Span

mod footer;
mod header;
mod horizontal_panel;
mod vertical_panel;

pub use footer::Footer;
pub use header::Header;
pub use horizontal_panel::HorizontalPanel;
pub use vertical_panel::VerticalPanel;

/// Panel allows to add a Row which has 1 continues Cell to a [`Table`].
///
/// See `examples/panel.rs`.
///
/// [`Table`]: crate::Table
#[derive(Debug)]
pub struct Panel;

impl Panel {
    /// Creates an empty vertical row at given index.
    ///
    /// ```
    /// use tabled::{settings::Panel, Table};
    ///
    /// let data = [[1, 2, 3], [4, 5, 6]];
    ///
    /// let table = Table::new(data)
    ///     .with(Panel::vertical(1, "Tabled Releases"))
    ///     .to_string();
    ///
    /// println!("{}", table);
    ///
    /// assert_eq!(
    ///     table,
    ///     concat!(
    ///         "+---+-----------------+---+---+\n",
    ///         "| 0 | Tabled Releases | 1 | 2 |\n",
    ///         "+---+                 +---+---+\n",
    ///         "| 1 |                 | 2 | 3 |\n",
    ///         "+---+                 +---+---+\n",
    ///         "| 4 |                 | 5 | 6 |\n",
    ///         "+---+-----------------+---+---+",
    ///     )
    /// )
    /// ```
    pub fn vertical<S: AsRef<str>>(column: usize, text: S) -> VerticalPanel<S> {
        VerticalPanel::new(column, text)
    }

    /// Creates an empty horizontal row at given index.
    ///
    /// ```
    /// use tabled::{Table, settings::Panel};
    ///
    /// let data = [[1, 2, 3], [4, 5, 6]];
    ///
    /// let table = Table::new(data)
    ///     .with(Panel::vertical(1, ""))
    ///     .to_string();
    ///
    /// println!("{}", table);
    ///
    /// assert_eq!(
    ///     table,
    ///     concat!(
    ///         "+---+--+---+---+\n",
    ///         "| 0 |  | 1 | 2 |\n",
    ///         "+---+  +---+---+\n",
    ///         "| 1 |  | 2 | 3 |\n",
    ///         "+---+  +---+---+\n",
    ///         "| 4 |  | 5 | 6 |\n",
    ///         "+---+--+---+---+",
    ///     )
    /// )
    /// ```
    pub fn horizontal<S: AsRef<str>>(row: usize, text: S) -> HorizontalPanel<S> {
        HorizontalPanel::new(row, text)
    }

    /// Creates an horizontal row at first row.
    pub fn header<S: AsRef<str>>(text: S) -> Header<S> {
        Header::new(text)
    }

    /// Creates an horizontal row at last row.
    pub fn footer<S: AsRef<str>>(text: S) -> Footer<S> {
        Footer::new(text)
    }
}
