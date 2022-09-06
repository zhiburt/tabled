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

mod column_peaker;
mod justify;
mod min_width;
mod truncate;
mod width_value;
mod wrap;

use crate::Table;

pub use self::{
    column_peaker::{ColumnPeaker, PriorityMax, PriorityMin, PriorityNone},
    justify::Justify,
    min_width::MinWidth,
    truncate::{SuffixLimit, Truncate},
    width_value::{Max, Min, Percent, WidthValue},
    wrap::Wrap,
};

use papergrid::{
    records::Records,
    width::{CfgWidthFunction, WidthEstimator, WidthFunc},
    Estimate, GridConfig,
};

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
        W: WidthValue,
    {
        Wrap::new(width)
    }

    /// Returns a [`Truncate`] structure.
    pub fn truncate<W>(width: W) -> Truncate<'static, W>
    where
        W: WidthValue,
    {
        Truncate::new(width)
    }

    /// Returns a [`MinWidth`] structure.
    pub fn increase<W>(width: W) -> MinWidth<W>
    where
        W: WidthValue,
    {
        MinWidth::new(width)
    }

    /// Returns a [`Justify`] structure.
    pub fn justify<W>(width: W) -> Justify<W>
    where
        W: WidthValue,
    {
        Justify::new(width)
    }
}

pub(crate) fn get_table_widths<R>(records: R, cfg: &GridConfig) -> Vec<usize>
where
    R: Records,
{
    let mut evaluator = WidthEstimator::default();
    evaluator.estimate(records, cfg);
    evaluator.into()
}

pub(crate) fn get_table_widths_with_total<R>(records: R, cfg: &GridConfig) -> (Vec<usize>, usize)
where
    R: Records,
{
    let mut evaluator = WidthEstimator::default();
    evaluator.estimate(&records, cfg);
    let total_width = get_total_width(&records, cfg, &evaluator);
    let widths = evaluator.into();
    (widths, total_width)
}

pub(crate) fn get_width_value<R, W>(value: &W, table: &Table<R>) -> usize
where
    W: WidthValue,
    R: Records,
{
    let ctrl = CfgWidthFunction::from_cfg(table.get_config());
    value.width(table.get_records(), table.get_config(), ctrl)
}

pub(crate) fn get_total_width<W, R>(records: R, cfg: &GridConfig, ctrl: &W) -> usize
where
    W: Estimate<R>,
    R: Records,
{
    ctrl.total()
        + cfg.count_vertical(records.count_columns())
        + cfg.get_margin().left.size
        + cfg.get_margin().right.size
}

pub(crate) fn count_borders(
    cfg: &GridConfig,
    start: usize,
    end: usize,
    count_columns: usize,
) -> usize {
    (start..end)
        .skip(1)
        .filter(|&i| cfg.has_vertical(i, count_columns))
        .count()
}

fn grid_widths<'a, R, W>(
    records: &'a R,
    width_ctrl: &'a W,
) -> impl Iterator<Item = impl Iterator<Item = usize> + 'a> + 'a
where
    W: WidthFunc,
    R: Records,
{
    let (count_rows, count_cols) = (records.count_rows(), records.count_columns());
    (0..count_rows).map(move |row| {
        (0..count_cols).map(move |col| width_ctrl.width_multiline(records.get_text((row, col))))
    })
}
