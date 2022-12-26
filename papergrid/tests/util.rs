#![allow(dead_code, unused_macros, unused_imports)]

use std::collections::HashMap;

use papergrid::{
    height::HeightEstimator,
    records::{cell_info::CellInfo, vec_records::VecRecords, Records},
    width::{CfgWidthFunction, WidthEstimator},
    Borders, Estimate, Grid, GridConfig, Position,
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
) -> Grid<'static, &'static VecRecords<CellInfo<'static>>, WidthEstimator, HeightEstimator> {
    let cfg = Box::leak(Box::new(cfg));

    let records = data;
    let records = Box::leak(records.into_boxed_slice());
    let records = VecRecords::new(records, (rows, cols), CfgWidthFunction::from_cfg(cfg));
    let records = Box::leak(Box::new(records));

    let width: &mut WidthEstimator = Box::leak(Box::default());
    width.estimate(&*records, cfg);
    let height: &mut HeightEstimator = Box::leak(Box::default());
    height.estimate(&*records, cfg);

    Grid::new(&*records, cfg, &*width, &*height)
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
            println!("{}", $table);
            let table = $table.to_string();
            assert_eq!(table, crate::util::static_table!($($line)*));
        }
    };
}

pub(crate) use test_table;

pub const DEFAULT_BORDERS: Borders = Borders {
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
    vertical: Some('|'),

    intersection: Some('+'),
};

/// A [`Estimate`]or of a width for a [`Grid`].
///
/// [`Grid`]: crate::Grid
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct EstimationList {
    list: Vec<usize>,
}

impl From<Vec<usize>> for EstimationList {
    fn from(list: Vec<usize>) -> Self {
        Self { list }
    }
}

impl<R> Estimate<R> for EstimationList
where
    R: Records,
{
    fn estimate(&mut self, _: R, _: &GridConfig) {}

    fn get(&self, column: usize) -> Option<usize> {
        self.list.get(column).cloned()
    }

    fn total(&self) -> usize {
        self.list.iter().sum()
    }
}
