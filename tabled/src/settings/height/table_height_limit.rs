use crate::{
    grid::{
        config::{ColoredConfig, Entity},
        dimension::CompleteDimension,
        records::{ExactRecords, IntoRecords, PeekableRecords, Records, RecordsMut},
        util::string::{count_lines, get_lines},
    },
    settings::{
        measurement::Measurement,
        peaker::{Peaker, PriorityNone},
        Height, TableOption,
    },
};

use super::util::get_table_height;

/// A modification of a table to decrease the table height.
#[derive(Debug)]
pub struct TableHeightLimit<W = usize, P = PriorityNone> {
    height: W,
    priority: P,
}

impl<W> TableHeightLimit<W, PriorityNone> {
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
    pub fn priority<P>(self, priority: P) -> TableHeightLimit<W, P>
    where
        P: Peaker,
    {
        TableHeightLimit {
            priority,
            height: self.height,
        }
    }
}

impl<R, W, P> TableOption<R, ColoredConfig, CompleteDimension> for TableHeightLimit<W, P>
where
    W: Measurement<Height>,
    P: Peaker + Clone,
    R: Records + ExactRecords + PeekableRecords + RecordsMut<String>,
    for<'a> &'a R: Records,
    for<'a> <<&'a R as Records>::Iter as IntoRecords>::Cell: AsRef<str>,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, dims: &mut CompleteDimension) {
        let count_rows = records.count_rows();
        let count_cols = records.count_columns();

        if count_rows == 0 || count_cols == 0 {
            return;
        }

        let height = self.height.measure(&*records, cfg);
        let (total, mut heights) = get_table_height(&*records, cfg);
        if total <= height {
            return;
        }

        decrease_list(&mut heights, total, height, self.priority);

        for (row, &height) in heights.iter().enumerate() {
            for col in 0..count_cols {
                let text = records.get_text((row, col).into());
                let count_lines = count_lines(text);

                if count_lines <= height {
                    continue;
                }

                let text = limit_lines(text, height);

                records.set((row, col).into(), text);
            }
        }

        dims.set_heights(heights);
    }

    fn hint_change(&self) -> Option<Entity> {
        // NOTE: we set correct heights but we don't change widths
        //       so it must be normal to not have recalculations
        None
    }
}

fn decrease_list<P>(list: &mut [usize], total: usize, mut value: usize, mut peaker: P)
where
    P: Peaker,
{
    while value != total {
        let p = peaker.peak(&[], list);
        let row = match p {
            Some(row) => row,
            None => break,
        };

        list[row] -= 1;
        value += 1;
    }
}

fn limit_lines(s: &str, n: usize) -> String {
    let mut text = String::new();
    for (i, line) in get_lines(s).take(n).enumerate() {
        if i > 0 {
            text.push('\n');
        }

        text.push_str(&line);
    }

    text
}
