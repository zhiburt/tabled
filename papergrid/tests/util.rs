use papergrid::{Borders, Entity, Grid, Settings};

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

pub fn new_grid<const N_ROWS: usize, const N_COLUMNS: usize>() -> Grid {
    let mut grid = Grid::new(N_ROWS, N_COLUMNS);
    for row in 0..N_ROWS {
        for column in 0..N_COLUMNS {
            let text = format!("{}-{}", row, column);
            grid.set(Entity::Cell(row, column), Settings::new().text(text));
        }
    }

    grid.set_borders(DEFAULT_BORDERS);

    grid
}
