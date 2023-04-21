//! This module contains [`Justify`] structure, used to set an exact width to each column.

use crate::{
    grid::config::ColoredConfig,
    grid::records::{ExactRecords, PeekableRecords, Records, RecordsMut},
    settings::{
        measurement::{Max, Measurement, Min},
        CellOption, TableOption, Width,
    },
};

/// Justify sets all columns widths to the set value.
///
/// Be aware that it doesn't consider padding.
/// So if you want to set a exact width you might need to use [`Padding`] to set it to 0.
///
/// ## Examples
///
/// ```
/// use tabled::{Table, settings::{Width, Style, object::Segment, Padding, Modify}};
///
/// let data = ["Hello", "World", "!"];
///
/// let table = Table::new(&data)
///     .with(Style::markdown())
///     .with(Modify::new(Segment::all()).with(Padding::zero()))
///     .with(Width::justify(3));
/// ```
///
/// [`Max`] usage to justify by a max column width.
///
/// ```
/// use tabled::{Table, settings::{width::Justify, Style}};
///
/// let data = ["Hello", "World", "!"];
///
/// let table = Table::new(&data)
///     .with(Style::markdown())
///     .with(Justify::max());
/// ```
///
/// [`Padding`]: crate::settings::Padding
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
    /// [`Padding`]: crate::settings::Padding
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

impl<R, D, W> TableOption<R, D, ColoredConfig> for Justify<W>
where
    W: Measurement<Width>,
    R: Records + ExactRecords + PeekableRecords + RecordsMut<String>,
    for<'a> &'a R: Records,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        let width = self.width.measure(&*records, cfg);

        let count_rows = records.count_rows();
        let count_columns = records.count_columns();

        for row in 0..count_rows {
            for col in 0..count_columns {
                let pos = (row, col).into();
                CellOption::change(Width::increase(width), records, cfg, pos);
                CellOption::change(Width::truncate(width), records, cfg, pos);
            }
        }
    }
}
