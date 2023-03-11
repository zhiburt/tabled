use crate::{
    records::{ExactRecords, Records},
    settings::{
        measurement::Measurement,
        peaker::{Peaker, PriorityNone},
        Height, TableOption,
    },
    tables::table::{ColoredConfig, TableDimension},
};

use super::util::get_table_height;

/// A modification of a table to increase the table height.
#[derive(Debug, Clone)]
pub struct TableHeightIncrease<W = usize, P = PriorityNone> {
    height: W,
    priority: P,
}

impl<W> TableHeightIncrease<W, PriorityNone> {
    /// Creates a new object.
    pub fn new(height: W) -> Self
    where
        W: Measurement<Height>,
    {
        Self {
            height,
            priority: PriorityNone::default(),
        }
    }

    /// Sets a different priority logic.
    pub fn priority<P>(self) -> TableHeightIncrease<W, P>
    where
        P: Peaker,
    {
        TableHeightIncrease {
            priority: P::create(),
            height: self.height,
        }
    }
}

impl<R, W, P> TableOption<R, TableDimension<'static>, ColoredConfig> for TableHeightIncrease<W, P>
where
    W: Measurement<Height>,
    P: Peaker + Clone,
    R: Records + ExactRecords,
    for<'a> &'a R: Records,
{
    fn change(
        &mut self,
        records: &mut R,
        cfg: &mut ColoredConfig,
        dims: &mut TableDimension<'static>,
    ) {
        if records.count_rows() == 0 || records.count_columns() == 0 {
            return;
        }

        let height = self.height.measure(&*records, cfg);
        let (total, mut heights) = get_table_height(&*records, cfg);
        if total >= height {
            return;
        }

        get_increase_list(&mut heights, height, total, self.priority.clone());

        let _ = dims.set_heights(heights);
    }
}

fn get_increase_list<P>(list: &mut [usize], total: usize, mut current: usize, mut peaker: P)
where
    P: Peaker,
{
    while current != total {
        let col = match peaker.peak(&[], list) {
            Some(col) => col,
            None => break,
        };

        list[col] += 1;
        current += 1;
    }
}
