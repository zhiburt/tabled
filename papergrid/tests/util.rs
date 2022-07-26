use papergrid::{Borders, Grid, Position};

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
    vertical_intersection: Some('|'),

    intersection: Some('+'),
};

pub fn grid<const ROWS: usize, const COLS: usize>() -> Grid {
    let mut grid = Grid::new(records(ROWS, COLS), ROWS, COLS);
    grid.set_borders(DEFAULT_BORDERS);

    grid
}

#[allow(unused)]
pub fn grid_from<const ROWS: usize, const COLS: usize>(data: [[&str; COLS]; ROWS]) -> Grid {
    let records = data
        .iter()
        .map(|row| row.iter().map(|text| text.to_string()).collect())
        .collect();

    let mut grid = Grid::new(records, ROWS, COLS);
    grid.set_borders(DEFAULT_BORDERS);

    grid
}

#[allow(unused)]
pub fn grid_with_data<const ROWS: usize, const COLS: usize>(
    data: &[(Position, &'static str)],
) -> Grid {
    let mut records = records(ROWS, COLS);

    for &((row, col), text) in data {
        records[row][col] = text.to_owned();
    }

    let mut grid = Grid::new(records, ROWS, COLS);
    grid.set_borders(DEFAULT_BORDERS);

    grid
}

#[allow(unused)]
pub fn grid_const<const ROWS: usize, const COLS: usize>(text: &'static str) -> Grid {
    grid_with_data::<ROWS, COLS>(&build_array::<ROWS, COLS>(text))
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

fn build_array<const ROWS: usize, const COLS: usize>(text: &str) -> Vec<(Position, &str)> {
    let mut records = Vec::with_capacity(ROWS * COLS);
    for row in 0..ROWS {
        for col in 0..COLS {
            records.push(((row, col), text));
        }
    }

    records
}
