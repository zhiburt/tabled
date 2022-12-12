//! This module contains [`Justify`] structure, used to set an exact width to each column.

use papergrid::records::{Records, RecordsMut};

use crate::{
    measurement::{Max, Measurement, Min},
    CellOption, Table, TableOption, Width,
};

/// Justify sets all columns widths to the set value.
///
/// Be aware that it doesn't consider padding.
/// So if you want to set a exact width you might need to use [`Padding`] to set it to 0.
///
/// ## Examples
///
/// ```
/// use tabled::{width::Justify, Style, Modify, object::Segment, Padding, Table};
///
/// let data = ["Hello", "World", "!"];
///
/// let table = Table::new(&data)
///     .with(Style::markdown())
///     .with(Modify::new(Segment::all()).with(Padding::zero()))
///     .with(Justify::new(3));
/// ```
///
/// [`Max`] usage to justify by a max column width.
///
/// ```
/// use tabled::{width::Justify, Style, Table};
///
/// let data = ["Hello", "World", "!"];
///
/// let table = Table::new(&data)
///     .with(Style::markdown())
///     .with(Justify::max());
/// ```
///
/// [`Padding`]: crate::Padding
#[derive(Debug)]
pub struct Justify<W> {
    width: W,
}

impl<W> Justify<W>
where
    W: Measurement<Width>,
{
    /// Creates a new [`Justify`] instance.
    ///
    /// Be aware that [`Padding`] is not considered when comparing the width.
    ///
    /// [`Padding`]: crate::Padding
    pub fn new(width: W) -> Self {
        Self { width }
    }
}

impl Justify<Max> {
    /// Creates a new Justify instance with a Max width used as a value.
    pub fn max() -> Self {
        Self { width: Max }
    }
}

impl Justify<Min> {
    /// Creates a new Justify instance with a Min width used as a value.
    pub fn min() -> Self {
        Self { width: Min }
    }
}

impl<W, R> TableOption<R> for Justify<W>
where
    W: Measurement<Width>,
    R: Records + RecordsMut<String>,
{
    fn change(&mut self, table: &mut Table<R>) {
        let width = self.width.measure(table.get_records(), table.get_config());

        let (count_rows, count_cols) = table.shape();
        for row in 0..count_rows {
            for col in 0..count_cols {
                let pos = (row, col).into();
                Width::increase(width).change_cell(table, pos);
                Width::truncate(width).change_cell(table, pos);
            }
        }

        // we can't cache the widths because it doesn't consider padding
        // table.cache_width(vec![width; table.shape().1]);
        table.destroy_width_cache();
        table.destroy_height_cache();
    }
}
