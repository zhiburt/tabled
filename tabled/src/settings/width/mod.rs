//! This module contains object which can be used to limit a cell to a given width:
//!
//! - [`Truncate`] cuts a cell content to limit width.
//! - [`Wrap`] split the content via new lines in order to fit max width.
//! - [`Justify`] sets columns width to the same value.
//!
//! To set a a table width, a combination of [`Width::truncate`] or [`Width::wrap`] and [`Width::increase`] can be used.
//!
//! ## Example
//!
//! ```
//! use tabled::{Table, settings::Width};
//!
//! let table = Table::new(&["Hello World!"])
//!     .with(Width::wrap(7))
//!     .with(Width::increase(7))
//!     .to_string();
//!
//! assert_eq!(
//!     table,
//!     concat!(
//!         "+-----+\n",
//!         "| &st |\n",
//!         "| r   |\n",
//!         "+-----+\n",
//!         "| Hel |\n",
//!         "| lo  |\n",
//!         "| Wor |\n",
//!         "| ld! |\n",
//!         "+-----+",
//!     )
//! );
//! ```

mod justify;
mod min_width;
mod truncate;
mod util;
mod width_list;
mod wrap;

use crate::settings::measurement::Measurement;

pub use self::{
    justify::Justify,
    min_width::MinWidth,
    truncate::{SuffixLimit, Truncate},
    width_list::WidthList,
    wrap::Wrap,
};

/// Width allows you to set a min and max width of an object on a [`Table`]
/// using different strategies.
///
/// It also allows you to set a min and max width for a whole table.
///
/// You can apply a min and max strategy at the same time with the same value,
/// the value will be a total table width.
///
/// It is an abstract factory.
///
/// Beware that borders are not removed when you set a size value to very small.
/// For example if you set size to 0 the table still be rendered but with all content removed.
///
/// Also be aware that it doesn't changes [`Padding`] settings nor it considers them.
///
/// The function is color aware if a `color` feature is on.
///
/// ## Examples
///
/// ### Cell change
///
/// ```
/// use tabled::{Table, settings::{object::Segment, Width, Style, Modify}};
///
/// let data = ["Hello", "World", "!"];
///
/// let table = Table::new(&data)
///     .with(Style::markdown())
///     .with(Modify::new(Segment::all()).with(Width::truncate(3).suffix("...")));
/// ```
///
/// ### Table change
///
/// ```
/// use tabled::{Table, settings::Width};
///
/// let table = Table::new(&["Hello World!"]).with(Width::wrap(5));
/// ```
///
/// ### Total width
///
/// ```
/// use tabled::{Table, settings::Width};
///
/// let table = Table::new(&["Hello World!"])
///     .with(Width::wrap(5))
///     .with(Width::increase(5));
/// ```
///
/// [`Padding`]: crate::settings::Padding
/// [`Table`]: crate::Table
#[derive(Debug)]
pub struct Width;

impl Width {
    /// Returns a [`Wrap`] structure.
    pub fn wrap<W: Measurement<Width>>(width: W) -> Wrap<W> {
        Wrap::new(width)
    }

    /// Returns a [`Truncate`] structure.
    pub fn truncate<W: Measurement<Width>>(width: W) -> Truncate<'static, W> {
        Truncate::new(width)
    }

    /// Returns a [`MinWidth`] structure.
    pub fn increase<W: Measurement<Width>>(width: W) -> MinWidth<W> {
        MinWidth::new(width)
    }

    /// Returns a [`Justify`] structure.
    pub fn justify<W: Measurement<Width>>(width: W) -> Justify<W> {
        Justify::new(width)
    }

    /// Create [`WidthList`] to set a table width to a constant list of column widths.
    ///
    /// Notice if you provide a list with `.len()` smaller than `Table::count_columns` then it will have no affect.
    ///
    /// Also notice that you must provide values bigger than or equal to a real content width, otherwise it may panic.
    ///
    /// # Example
    ///
    /// ```
    /// use tabled::{Table, settings::Width};
    ///
    /// let data = vec![
    ///     ("Some\ndata", "here", "and here"),
    ///     ("Some\ndata on a next", "line", "right here"),
    /// ];
    ///
    /// let table = Table::new(data)
    ///     .with(Width::list([20, 10, 12]))
    ///     .to_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     "+--------------------+----------+------------+\n\
    ///      | &str               | &str     | &str       |\n\
    ///      +--------------------+----------+------------+\n\
    ///      | Some               | here     | and here   |\n\
    ///      | data               |          |            |\n\
    ///      +--------------------+----------+------------+\n\
    ///      | Some               | line     | right here |\n\
    ///      | data on a next     |          |            |\n\
    ///      +--------------------+----------+------------+"
    /// )
    /// ```
    pub fn list<I: IntoIterator<Item = usize>>(rows: I) -> WidthList {
        WidthList::new(rows.into_iter().collect())
    }
}
