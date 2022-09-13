//! The module contains [`Measurment`] trait and its implementations to be used in [`Height`] and [`Width`].

use papergrid::{
    records::Records,
    width::{CfgWidthFunction, WidthFunc},
    GridConfig,
};

use crate::{height::get_table_total_height, width::get_table_widths_with_total, Height, Width};

/// A width value which can be obtained on behalf of [`Table`].
///
/// [`Table`]: crate::Table
pub trait Measurment<Attribute> {
    /// Returns a measurment value.
    fn measure<R>(&self, records: R, cfg: &GridConfig) -> usize
    where
        R: Records;
}

impl<T> Measurment<T> for usize {
    fn measure<R>(&self, _: R, _: &GridConfig) -> usize {
        *self
    }
}

/// Max width value.
#[derive(Debug)]
pub struct Max;

impl Measurment<Width> for Max {
    fn measure<R>(&self, records: R, cfg: &GridConfig) -> usize
    where
        R: Records,
    {
        let ctrl = CfgWidthFunction::from_cfg(cfg);
        grid_widths(&records, &ctrl)
            .map(|r| r.max().unwrap_or(0))
            .max()
            .unwrap_or(0)
    }
}

impl Measurment<Height> for Max {
    fn measure<R>(&self, records: R, _: &GridConfig) -> usize
    where
        R: Records,
    {
        records_heights(&records)
            .map(|r| r.max().unwrap_or(0))
            .max()
            .unwrap_or(0)
    }
}

/// Min width value.
#[derive(Debug)]
pub struct Min;

impl Measurment<Width> for Min {
    fn measure<R>(&self, records: R, cfg: &GridConfig) -> usize
    where
        R: Records,
    {
        let ctrl = CfgWidthFunction::from_cfg(cfg);
        grid_widths(&records, &ctrl)
            .map(|r| r.min().unwrap_or(0))
            .max()
            .unwrap_or(0)
    }
}

impl Measurment<Height> for Min {
    fn measure<R>(&self, records: R, _: &GridConfig) -> usize
    where
        R: Records,
    {
        records_heights(&records)
            .map(|r| r.max().unwrap_or(0))
            .min()
            .unwrap_or(0)
    }
}

/// Percent from a total table width.
#[derive(Debug)]
pub struct Percent(pub usize);

impl Measurment<Width> for Percent {
    fn measure<R>(&self, records: R, cfg: &GridConfig) -> usize
    where
        R: Records,
    {
        let (_, total) = get_table_widths_with_total(&records, cfg);
        (total * self.0) / 100
    }
}

impl Measurment<Height> for Percent {
    fn measure<R>(&self, records: R, cfg: &GridConfig) -> usize
    where
        R: Records,
    {
        let total = get_table_total_height(&records, cfg);
        (total * self.0) / 100
    }
}

fn records_heights<R>(records: &R) -> impl Iterator<Item = impl Iterator<Item = usize> + '_> + '_
where
    R: Records,
{
    (0..records.count_rows()).map(move |row| {
        (0..records.count_columns()).map(move |col| records.count_lines((row, col)))
    })
}

fn grid_widths<'a, R, W>(
    records: &'a R,
    ctrl: &'a W,
) -> impl Iterator<Item = impl Iterator<Item = usize> + 'a> + 'a
where
    W: WidthFunc,
    R: Records,
{
    let (count_rows, count_cols) = (records.count_rows(), records.count_columns());
    (0..count_rows)
        .map(move |row| (0..count_cols).map(move |col| records.get_width((row, col), ctrl)))
}
