use papergrid::{records::Records, width::WidthFunc, GridConfig};

use super::{get_table_widths_with_total, grid_widths};

/// A width value which can be obtained on behalf of [`Table`].
///
/// [`Table`]: crate::Table
pub trait WidthValue {
    /// Returns a width value.
    fn width<R, W>(&self, records: R, cfg: &GridConfig, ctrl: W) -> usize
    where
        W: WidthFunc,
        R: Records;
}

impl WidthValue for usize {
    fn width<R, W>(&self, _: R, _: &GridConfig, _: W) -> usize {
        *self
    }
}

/// Max width value.
#[derive(Debug)]
pub struct Max;

impl WidthValue for Max {
    fn width<R, W>(&self, records: R, _: &GridConfig, ctrl: W) -> usize
    where
        W: WidthFunc,
        R: Records,
    {
        grid_widths(&records, &ctrl)
            .map(|r| r.max().unwrap_or(0))
            .max()
            .unwrap_or(0)
    }
}

/// Min width value.
#[derive(Debug)]
pub struct Min;

impl WidthValue for Min {
    fn width<R, W>(&self, records: R, _: &GridConfig, ctrl: W) -> usize
    where
        W: WidthFunc,
        R: Records,
    {
        grid_widths(&records, &ctrl)
            .map(|r| r.min().unwrap_or(0))
            .max()
            .unwrap_or(0)
    }
}

/// Percent from a total table width.
#[derive(Debug)]
pub struct Percent(pub usize);

impl WidthValue for Percent {
    fn width<R, W>(&self, records: R, cfg: &GridConfig, _: W) -> usize
    where
        W: WidthFunc,
        R: Records,
    {
        let (_, total) = get_table_widths_with_total(&records, cfg);
        (total * self.0) / 100
    }
}
