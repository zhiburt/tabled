use papergrid::{Entity, Grid, Settings, DEFAULT_CELL_STYLE};

pub fn new_grid<const N_ROWS: usize, const N_COLUMNS: usize>() -> Grid {
    let mut grid = Grid::new(N_ROWS, N_COLUMNS);
    grid.set_cell_borders(DEFAULT_CELL_STYLE);

    for row in 0..N_ROWS {
        for column in 0..N_COLUMNS {
            let text = format!("{}-{}", row, column);
            grid.set(Entity::Cell(row, column), Settings::new().text(text));
        }
    }

    grid
}
