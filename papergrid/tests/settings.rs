use papergrid::{Entity, Grid, Settings, DEFAULT_CELL_STYLE};

#[test]
fn set_global_text_2x2() {
    let mut grid = Grid::new(2, 2);
    grid.set(Entity::Global, Settings::new().text("asd"));
    grid.set_cell_borders(&DEFAULT_CELL_STYLE);
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

#[test]
fn global_styles_overrides_everything_2x2() {
    let mut grid = Grid::new(2, 2);
    grid.set_cell_borders(&DEFAULT_CELL_STYLE);
    grid.set(Entity::Cell(0, 0), Settings::new().text("xxxxx"));
    grid.set(Entity::Cell(0, 1), Settings::new().text("xx"));
    grid.set(Entity::Cell(1, 0), Settings::new().text("y"));
    grid.set(Entity::Cell(1, 1), Settings::new().text("yyyyyyyyyy"));

    assert_eq!(
        grid.to_string(),
        "+-----+----------+\n\
         |xxxxx|xx        |\n\
         +-----+----------+\n\
         |y    |yyyyyyyyyy|\n\
         +-----+----------+\n"
    );

    grid.set(
        Entity::Global,
        Settings::new().alignment(papergrid::AlignmentHorizontal::Center),
    );

    assert_eq!(
        grid.to_string(),
        "+-----+----------+\n\
         |xxxxx|    xx    |\n\
         +-----+----------+\n\
         |  y  |yyyyyyyyyy|\n\
         +-----+----------+\n"
    );
}
