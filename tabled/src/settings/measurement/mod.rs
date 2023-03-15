//! The module contains [`Measurement`] trait and its implementations to be used in [`Height`] and [`Width`].;

use crate::{
    grid::config::SpannedConfig,
    grid::dimension::SpannedGridDimension,
    grid::records::{ExactRecords, PeekableRecords, Records},
    grid::util::string::{self, string_width_multiline},
    settings::{Height, Width},
};

/// A width value which can be obtained on behalf of [`Table`].
///
/// [`Table`]: crate::Table
pub trait Measurement<Attribute> {
    /// Returns a measurement value.
    fn measure<R: Records + ExactRecords + PeekableRecords>(
        &self,
        records: R,
        cfg: &SpannedConfig,
    ) -> usize;
}

impl<T> Measurement<T> for usize {
    fn measure<R>(&self, _: R, _: &SpannedConfig) -> usize {
        *self
    }
}

/// Max width value.
#[derive(Debug)]
pub struct Max;

impl Measurement<Width> for Max {
    fn measure<R: Records + ExactRecords + PeekableRecords>(
        &self,
        records: R,
        _: &SpannedConfig,
    ) -> usize {
        grid_widths(&records)
            .map(|r| r.max().unwrap_or(0))
            .max()
            .unwrap_or(0)
    }
}

impl Measurement<Height> for Max {
    fn measure<R: Records + ExactRecords + PeekableRecords>(
        &self,
        records: R,
        _: &SpannedConfig,
    ) -> usize {
        records_heights(&records)
            .map(|r| r.max().unwrap_or(0))
            .max()
            .unwrap_or(0)
    }
}

/// Min width value.
#[derive(Debug)]
pub struct Min;

impl Measurement<Width> for Min {
    fn measure<R: Records + ExactRecords + PeekableRecords>(
        &self,
        records: R,
        _: &SpannedConfig,
    ) -> usize {
        grid_widths(&records)
            .map(|r| r.min().unwrap_or(0))
            .max()
            .unwrap_or(0)
    }
}

impl Measurement<Height> for Min {
    fn measure<R: Records + ExactRecords + PeekableRecords>(
        &self,
        records: R,
        _: &SpannedConfig,
    ) -> usize {
        records_heights(&records)
            .map(|r| r.max().unwrap_or(0))
            .min()
            .unwrap_or(0)
    }
}

/// Percent from a total table width.
#[derive(Debug)]
pub struct Percent(pub usize);

impl Measurement<Width> for Percent {
    fn measure<R>(&self, records: R, cfg: &SpannedConfig) -> usize
    where
        R: Records,
    {
        let (_, total) = get_table_widths_with_total(records, cfg);
        (total * self.0) / 100
    }
}

impl Measurement<Height> for Percent {
    fn measure<R>(&self, records: R, cfg: &SpannedConfig) -> usize
    where
        R: Records + ExactRecords,
    {
        let (_, total) = get_table_heights_width_total(records, cfg);
        (total * self.0) / 100
    }
}

fn grid_widths<R: Records + ExactRecords + PeekableRecords>(
    records: &R,
) -> impl Iterator<Item = impl Iterator<Item = usize> + '_> + '_ {
    let (count_rows, count_cols) = (records.count_rows(), records.count_columns());
    (0..count_rows).map(move |row| {
        (0..count_cols).map(move |col| string_width_multiline(records.get_text((row, col))))
    })
}

fn get_table_widths_with_total<R>(records: R, cfg: &SpannedConfig) -> (Vec<usize>, usize)
where
    R: Records,
{
    let widths = SpannedGridDimension::width(records, cfg);
    let total_width = get_table_total_width(&widths, cfg);
    (widths, total_width)
}

fn get_table_total_width(list: &[usize], cfg: &SpannedConfig) -> usize {
    let total = list.iter().sum::<usize>();

    total + cfg.count_vertical(list.len())
}

fn records_heights<R>(records: &R) -> impl Iterator<Item = impl Iterator<Item = usize> + '_> + '_
where
    R: Records + ExactRecords + PeekableRecords,
{
    (0..records.count_rows()).map(move |row| {
        (0..records.count_columns())
            .map(move |col| string::count_lines(records.get_text((row, col))))
    })
}

fn get_table_heights_width_total<R>(records: R, cfg: &SpannedConfig) -> (Vec<usize>, usize)
where
    R: Records,
{
    let list = SpannedGridDimension::height(records, cfg);
    let total = get_table_total_height(&list, cfg);
    (list, total)
}

fn get_table_total_height(list: &[usize], cfg: &SpannedConfig) -> usize {
    let total = list.iter().sum::<usize>();
    let counth = cfg.count_horizontal(list.len());

    total + counth
}
