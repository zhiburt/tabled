//! The module contains [`Measurement`] trait and its implementations to be used in [`Height`] and [`Width`].;

use crate::{
    grid::config::SpannedConfig,
    grid::dimension::SpannedGridDimension,
    grid::records::{ExactRecords, IntoRecords, PeekableRecords, Records},
    grid::util::string::{self, get_text_width},
    settings::{Height, Width},
};

// todo: Change the trait to not bind to exact Records

/// A width value which can be obtained on behalf of [`Table`].
///
/// [`Table`]: crate::Table
pub trait Measurement<Attribute> {
    /// Returns a measurement value.
    fn measure<R>(&self, records: R, cfg: &SpannedConfig) -> usize
    where
        R: Records + ExactRecords + PeekableRecords,
        <R::Iter as IntoRecords>::Cell: AsRef<str>;
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
        <R::Iter as IntoRecords>::Cell: AsRef<str>,
    {
        let total = SpannedGridDimension::width_total(records, cfg);
        (total * self.0) / 100
    }
}

impl Measurement<Height> for Percent {
    fn measure<R>(&self, records: R, cfg: &SpannedConfig) -> usize
    where
        R: Records + ExactRecords,
        <R::Iter as IntoRecords>::Cell: AsRef<str>,
    {
        let total = SpannedGridDimension::height_total(records, cfg);
        (total * self.0) / 100
    }
}

fn grid_widths<R>(records: &R) -> impl Iterator<Item = impl Iterator<Item = usize> + '_> + '_
where
    R: Records + ExactRecords + PeekableRecords,
{
    let (count_rows, count_cols) = (records.count_rows(), records.count_columns());
    (0..count_rows).map(move |row| {
        (0..count_cols).map(move |col| get_text_width(records.get_text((row, col))))
    })
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
