#![allow(dead_code, unused_macros, unused_imports)]

use std::collections::HashMap;

use papergrid::{
    height::HeightEstimator, records::records_info::RecordsInfo, width::WidthEstimator, Borders,
    Estimate, Grid, GridConfig, Position,
};

pub fn grid(rows: usize, cols: usize) -> GridBuilder {
    GridBuilder::new(rows, cols)
}

#[derive(Debug, Default, Clone)]
pub struct GridBuilder {
    size: (usize, usize),
    cfg: GridConfig,
    data: HashMap<Position, String>,
}

impl GridBuilder {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut cfg = GridConfig::default();
        cfg.set_borders(DEFAULT_BORDERS);

        Self {
            size: (rows, cols),
            cfg,
            ..Default::default()
        }
    }

    pub fn config(mut self, mut f: impl FnMut(&mut GridConfig)) -> Self {
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

        let grid = build_grid(self.size.0, self.size.1, self.cfg, data);

        grid.to_string()
    }
}

fn build_grid(
    rows: usize,
    cols: usize,
    cfg: GridConfig,
    data: Vec<Vec<String>>,
) -> Grid<'static, &'static RecordsInfo<'static>, WidthEstimator, HeightEstimator> {
    let cfg = Box::leak(Box::new(cfg));

    let records = data;
    let records = Box::leak(records.into_boxed_slice());
    let records = RecordsInfo::new(records, (rows, cols), cfg);
    let records = Box::leak(Box::new(records));

    let mut width = WidthEstimator::default();
    width.estimate(&*records, cfg);
    let mut height = HeightEstimator::default();
    height.estimate(&*records, cfg);

    let grid = Grid::new(&*records, cfg, width, height);

    grid.to_string();

    grid
}

fn records(rows: usize, cols: usize) -> Vec<Vec<String>> {
    let mut records = vec![vec![String::new(); cols]; rows];
    (0..rows).for_each(|row| {
        (0..cols).for_each(|col| {
            let text = format!("{}-{}", row, col);
            records[row][col] = text;
        });
    });

    records
}

macro_rules! static_table {
    ($($line:expr)*) => {
        concat!(
            $($line, "\n",)*
        )
        .trim_end_matches('\n')
    };
}

pub(crate) use static_table;

macro_rules! test_table {
    ($test:ident, $table:expr, $($line:expr)*) => {
        #[test]
        fn $test() {
            let table = $table.to_string();
            assert_eq!(table, crate::util::static_table!($($line)*));
        }
    };
}

pub(crate) use test_table;

const DEFAULT_BORDERS: Borders = Borders {
    top: Some('-'),
    top_left: Some('+'),
    top_right: Some('+'),
    top_intersection: Some('+'),

    bottom: Some('-'),
    bottom_left: Some('+'),
    bottom_right: Some('+'),
    bottom_intersection: Some('+'),

    horizontal: Some('-'),
    horizontal_left: Some('+'),
    horizontal_right: Some('+'),

    vertical_left: Some('|'),
    vertical_right: Some('|'),
    vertical_intersection: Some('|'),

    intersection: Some('+'),
};
