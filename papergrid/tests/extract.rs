use papergrid::{Entity, Grid, Settings, DEFAULT_CELL_STYLE};

#[test]
fn extract_inside_test() {
    let grid = new_grid::<3, 3>();

    let grid = grid.extract(1..2, 1..2);

    assert_eq!(
        grid.to_string(),
        "+---+\n\
         |1-1|\n\
         +---+\n"
    )
}

#[test]
fn extract_left_test() {
    let grid = new_grid::<3, 3>();

    let grid = grid.extract(.., ..1);

    assert_eq!(
        grid.to_string(),
        "+---+\n\
         |0-0|\n\
         +---+\n\
         |1-0|\n\
         +---+\n\
         |2-0|\n\
         +---+\n"
    )
}

#[test]
fn extract_right_test() {
    let grid = new_grid::<3, 3>();

    let grid = grid.extract(.., 2..);

    assert_eq!(
        grid.to_string(),
        "+---+\n\
         |0-2|\n\
         +---+\n\
         |1-2|\n\
         +---+\n\
         |2-2|\n\
         +---+\n"
    )
}

#[test]
fn extract_top_test() {
    let grid = new_grid::<3, 3>();

    let grid = grid.extract(..1, ..);

    assert_eq!(
        grid.to_string(),
        "+---+---+---+\n\
         |0-0|0-1|0-2|\n\
         +---+---+---+\n"
    )
}

#[test]
fn extract_bottom_test() {
    let grid = new_grid::<3, 3>();

    let grid = grid.extract(2.., ..);

    assert_eq!(
        grid.to_string(),
        "+---+---+---+\n\
         |2-0|2-1|2-2|\n\
         +---+---+---+\n"
    )
}

#[test]
fn extract_all_test() {
    let grid = new_grid::<3, 3>();
    let grid = grid.extract(3.., 3..);
    assert_eq!(grid.to_string(), "");
}

#[test]
fn extract_empty_test() {
    let grid = new_grid::<0, 0>();
    let grid = grid.extract(.., ..);
    assert_eq!(grid.to_string(), "");
}

fn new_grid<const N_ROWS: usize, const N_COLUMNS: usize>() -> Grid {
    let mut grid = Grid::new(N_ROWS, N_COLUMNS);

    for row in 0 .. N_ROWS {
        for column in 0 .. N_COLUMNS {
            let text = format!("{}-{}", row, column);
            grid.set(&Entity::Cell(row, column), Settings::new().text(text));
        }
    }

    grid.set_cell_borders(DEFAULT_CELL_STYLE.clone());

    grid
}