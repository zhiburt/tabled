#![cfg(feature = "std")]
#![allow(dead_code, unused_macros, unused_imports)]

use std::collections::HashMap;

use papergrid::{
    colors::NoColors,
    config::spanned::SpannedConfig,
    config::{Borders, Position},
    dimension::spanned::SpannedGridDimension,
    dimension::{Dimension, Estimate},
    grid::iterable::Grid,
    records::{IterRecords, Records},
};

pub fn grid(rows: usize, cols: usize) -> GridBuilder {
    GridBuilder::new(rows, cols)
}

#[derive(Debug, Default, Clone)]
pub struct GridBuilder {
    size: (usize, usize),
    cfg: SpannedConfig,
    data: HashMap<Position, String>,
}

impl GridBuilder {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut cfg = SpannedConfig::default();
        cfg.set_borders(DEFAULT_BORDERS);

        Self {
            size: (rows, cols),
            cfg,
            ..Default::default()
        }
    }

    pub fn config(mut self, mut f: impl FnMut(&mut SpannedConfig)) -> Self {
        f(&mut self.cfg);
        self
    }

    pub fn data(
        mut self,
        data: impl IntoIterator<Item = impl IntoIterator<Item = impl Into<String>>>,
    ) -> Self {
        for (i, rows) in data.into_iter().enumerate() {
            for (j, text) in rows.into_iter().enumerate() {
                let text = text.into();
                self.data.insert((i, j), text);
            }
        }

        self
    }

    pub fn change_cell(mut self, pos: Position, text: impl Into<String>) -> Self {
        self.data.insert(pos, text.into());
        self
    }

    pub fn build(self) -> String {
        let mut data = records(self.size.0, self.size.1);
        for ((row, col), text) in self.data {
            data[row][col] = text;
        }

        let grid = build_grid(data, self.cfg, self.size);
        grid.to_string()
    }
}

fn build_grid(
    data: Vec<Vec<String>>,
    cfg: SpannedConfig,
    shape: (usize, usize),
) -> Grid<IterRecords<Vec<Vec<String>>>, SpannedGridDimension, SpannedConfig, NoColors> {
    let records = IterRecords::new(data, shape.1, Some(shape.0));

    let mut dims = SpannedGridDimension::default();
    dims.estimate(&records, &cfg);

    Grid::new(records, dims, cfg, NoColors)
}

fn records(rows: usize, cols: usize) -> Vec<Vec<String>> {
    let mut records = vec![vec![String::new(); cols]; rows];
    (0..rows).for_each(|row| {
        (0..cols).for_each(|col| {
            let text = format!("{row}-{col}");
            records[row][col] = text;
        });
    });

    records
}

pub const DEFAULT_BORDERS: Borders<char> = Borders {
    top: Some('-'),
    top_left: Some('+'),
    top_right: Some('+'),
    top_intersection: Some('+'),

    bottom: Some('-'),
    bottom_left: Some('+'),
    bottom_right: Some('+'),
    bottom_intersection: Some('+'),

    horizontal: Some('-'),
    left_intersection: Some('+'),
    right_intersection: Some('+'),

    left: Some('|'),
    right: Some('|'),
    vertical: Some('|'),

    intersection: Some('+'),
};

/// A [`Estimate`]or of a width for a [`Grid`].
///
/// [`Grid`]: crate::grid::iterable::Grid
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ConstantDimension(pub Vec<usize>, pub Vec<usize>);

impl Dimension for ConstantDimension {
    fn get_width(&self, column: usize) -> usize {
        self.0[column]
    }

    fn get_height(&self, row: usize) -> usize {
        self.1[row]
    }
}

impl<R> Estimate<R, SpannedConfig> for ConstantDimension {
    fn estimate(&mut self, _: R, _: &SpannedConfig) {}
}
