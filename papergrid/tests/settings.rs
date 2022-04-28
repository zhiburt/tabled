use papergrid::{Entity, Grid, Settings, DEFAULT_CELL_STYLE};

#[test]
fn set_global_text_2x2() {
    let mut grid = Grid::new(2, 2);
    grid.set(Entity::Global, Settings::new().text("asd"));
    grid.set_cell_borders(DEFAULT_CELL_STYLE.clone());
    let str = grid.to_string();
    assert_eq!(
        str,
        "+---+---+\n\
         |asd|asd|\n\
         +---+---+\n\
         |asd|asd|\n\
         +---+---+\n"
    )
}
