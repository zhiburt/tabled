//! The module contains [`Measurement`] trait and its implementations to be used in [`Height`] and [`Width`].;

use papergrid::{
    grid_projection::GridProjection,
    util::string::{count_lines, string_width_multiline_tab},
};

use crate::{
    grid::{config::GridConfig, dimension::ExactDimension},
    records::{ExactRecords, Records},
    // Height,
    Width,
};

/// A width value which can be obtained on behalf of [`Table`].
///
/// [`Table`]: crate::Table
pub trait Measurement<Attribute> {
    /// Returns a measurement value.
    fn measure<R: Records + ExactRecords>(&self, records: R, cfg: &GridConfig) -> usize;
}

impl<T> Measurement<T> for usize {
    fn measure<R>(&self, _: R, _: &GridConfig) -> usize {
        *self
    }
}

/// Max width value.
#[derive(Debug)]
pub struct Max;

impl Measurement<Width> for Max {
    fn measure<R: Records + ExactRecords>(&self, records: R, cfg: &GridConfig) -> usize {
        grid_widths(&records, cfg.get_tab_width())
            .map(|r| r.max().unwrap_or(0))
            .max()
            .unwrap_or(0)
    }
}

// impl Measurement<Height> for Max {
//     fn measure<R: Records + ExactRecords>(&self, records: R, _: &GridConfig) -> usize {
//         records_heights(&records)
//             .map(|r| r.max().unwrap_or(0))
//             .max()
//             .unwrap_or(0)
//     }
// }

/// Min width value.
#[derive(Debug)]
pub struct Min;

impl Measurement<Width> for Min {
    fn measure<R: Records + ExactRecords>(&self, records: R, cfg: &GridConfig) -> usize {
        grid_widths(&records, cfg.get_tab_width())
            .map(|r| r.min().unwrap_or(0))
            .max()
            .unwrap_or(0)
    }
}

// impl Measurement<Height> for Min {
//     fn measure<R: Records + ExactRecords>(&self, records: R, _: &GridConfig) -> usize {
//         records_heights(&records)
//             .map(|r| r.max().unwrap_or(0))
//             .min()
//             .unwrap_or(0)
//     }
// }

/// Percent from a total table width.
#[derive(Debug)]
pub struct Percent(pub usize);

impl Measurement<Width> for Percent {
    fn measure<R>(&self, records: R, cfg: &GridConfig) -> usize
    where
        R: Records,
    {
        let (_, total) = get_table_widths_with_total(records, cfg);
        (total * self.0) / 100
    }
}

// impl Measurement<Height> for Percent {
//     fn measure<R>(&self, records: R, cfg: &GridConfig) -> usize
//     where
//         R: Records + ExactRecords,
//     {
//         let (total, _) = get_table_height(records, cfg);
//         (total * self.0) / 100
//     }
// }

fn records_heights<R: Records + ExactRecords>(
    records: &R,
) -> impl Iterator<Item = impl Iterator<Item = usize> + '_> + '_ {
    (0..records.count_rows()).map(move |row| {
        (0..records.count_columns()).map(move |col| {
            let text = records.get_cell((row, col)).as_ref();
            count_lines(text)
        })
    })
}

fn grid_widths<'a, R: Records + ExactRecords>(
    records: &'a R,
    tab_width: usize,
) -> impl Iterator<Item = impl Iterator<Item = usize> + 'a> + 'a {
    let (count_rows, count_cols) = (records.count_rows(), records.count_columns());
    (0..count_rows).map(move |row| {
        (0..count_cols).map(move |col| {
            string_width_multiline_tab(records.get_cell((row, col)).as_ref(), tab_width)
        })
    })
}

fn get_table_widths_with_total<R>(records: R, cfg: &GridConfig) -> (Vec<usize>, usize)
where
    R: Records,
{
    let widths = ExactDimension::width(records, cfg);
    let total_width = get_table_total_width(&widths, cfg);
    (widths, total_width)
}

fn get_table_total_width(list: &[usize], cfg: &GridConfig) -> usize {
    let gp = GridProjection::new(cfg).count_columns(list.len());
    let margin = cfg.get_margin();

    let total = list.iter().sum::<usize>();

    total + gp.count_vertical() + margin.left.size + margin.right.size
}

fn get_table_height<R: Records + ExactRecords>(
    records: R,
    cfg: &GridConfig,
) -> (usize, Vec<usize>) {
    let gp = GridProjection::new(cfg).count_rows(records.count_rows());
    let count_horizontals = gp.count_horizontal();

    let margin = cfg.get_margin();
    let margin_size = margin.top.size + margin.bottom.size;

    let list = ExactDimension::height(records, cfg);
    let total = list.iter().sum::<usize>();

    let total = total + count_horizontals + margin_size;

    (total, list)
}
