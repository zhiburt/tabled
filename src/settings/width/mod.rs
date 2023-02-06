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
//! use tabled::{Width, Table};
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

use crate::{grid::dimension::ExactDimension, measurement::Measurement};

pub use self::{
    justify::Justify,
    min_width::MinWidth,
    truncate::{SuffixLimit, Truncate},
    width_list::WidthList,
    wrap::Wrap,
};

use papergrid::{dimension::Dimension, records::Records, GridConfig};

pub(crate) use wrap::wrap_text;

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
/// use tabled::{object::Segment, Width, Modify, Style, Table};
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
/// use tabled::{Width, Table};
///
/// let table = Table::new(&["Hello World!"]).with(Width::wrap(5));
/// ```
///
/// ### Total width
///
/// ```
/// use tabled::{Width, Table};
///
/// let table = Table::new(&["Hello World!"])
///     .with(Width::wrap(5))
///     .with(Width::increase(5));
/// ```
///
/// [`Padding`]: crate::Padding
/// [`Table`]: crate::Table
#[derive(Debug)]
pub struct Width;

impl Width {
    /// Returns a [`Wrap`] structure.
    pub fn wrap<W>(width: W) -> Wrap<W>
    where
        W: Measurement<Width>,
    {
        Wrap::new(width)
    }

    /// Returns a [`Truncate`] structure.
    pub fn truncate<W>(width: W) -> Truncate<'static, W>
    where
        W: Measurement<Width>,
    {
        Truncate::new(width)
    }

    /// Returns a [`MinWidth`] structure.
    pub fn increase<W>(width: W) -> MinWidth<W>
    where
        W: Measurement<Width>,
    {
        MinWidth::new(width)
    }

    /// Returns a [`Justify`] structure.
    pub fn justify<W>(width: W) -> Justify<W>
    where
        W: Measurement<Width>,
    {
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
    /// use tabled::{Table, Width};
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
    pub fn list<I>(rows: I) -> WidthList
    where
        I: IntoIterator<Item = usize>,
    {
        WidthList::new(rows.into_iter().collect())
    }
}
