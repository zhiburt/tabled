use std::cmp::max;

use crate::{
    grid::GridConfig,
    records::{Cell, Records},
    Entity, Position,
};

use super::Estimate;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct HeightEstimator {
    heights: Vec<usize>,
}

impl<R> Estimate<R> for HeightEstimator
where
    R: Records,
    R::Cell: Cell,
{
    fn estimate(&mut self, records: R, cfg: &GridConfig) {
        self.heights = build_heights(cfg, &records).collect();
    }

    fn get(&self, column: usize) -> Option<usize> {
        self.heights.get(column).copied()
    }

    fn total(&self) -> usize {
        self.heights.iter().sum()
    }
}

impl From<Vec<usize>> for HeightEstimator {
    fn from(heights: Vec<usize>) -> Self {
        Self { heights }
    }
}

fn build_heights<'a, R>(cfg: &'a GridConfig, records: &'a R) -> impl Iterator<Item = usize> + 'a
where
    R: Records,
    R::Cell: Cell,
{
    let (count_rows, count_columns) = records.size();
    (0..count_rows).map(move |row| {
        (0..count_columns)
            .map(|col| cell_height(records, cfg, (row, col)))
            .max()
            .unwrap_or(0)
    })
}

fn cell_height<R>(records: &R, cfg: &GridConfig, pos: Position) -> usize
where
    R: Records,
    R::Cell: Cell,
{
    let count_lines = max(1, records.get(pos).count_lines());
    let padding = cfg.get_padding(Entity::Cell(pos.0, pos.1));
    count_lines + padding.top.size + padding.bottom.size
}
