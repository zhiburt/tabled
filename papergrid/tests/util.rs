use papergrid::{Entity, Grid, Settings};

pub fn new_grid<const N_ROWS: usize, const N_COLUMNS: usize>() -> Grid {
    let mut grid = Grid::new(N_ROWS, N_COLUMNS);
    for row in 0..N_ROWS {
        for column in 0..N_COLUMNS {
            let text = format!("{}-{}", row, column);
            grid.set(Entity::Cell(row, column), Settings::new().text(text));
        }
    }

    grid
}
