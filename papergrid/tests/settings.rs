use papergrid::{Border, Borders, Entity, Settings};

mod util;

#[test]
fn set_global_text_2x2() {
    let mut grid = util::new_grid::<2, 2>();
    grid.set(Entity::Global, Settings::new().text("asd"));
    let str = grid.to_string();
    assert_eq!(
        str,
        "+---+---+\n\
         |asd|asd|\n\
         +---+---+\n\
         |asd|asd|\n\
         +---+---+",
    )
}

#[test]
fn global_styles_overrides_everything_2x2() {
    let mut grid = util::new_grid::<2, 2>();
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
         +-----+----------+",
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
         +-----+----------+",
    );
}

#[test]
fn remove_border_test() {
    let mut grid = util::new_grid::<2, 2>();
    grid.set_borders(Borders::default());

    grid.set(
        Entity::Cell(0, 0),
        Settings::new().border(Border {
            top: Some('x'),
            bottom: Some('o'),
            left: Some('q'),
            ..Default::default()
        }),
    );

    // assert_eq!(grid.to_string(), " xxx   \nq0-00-1\n ooo   \n 1-01-1");

    grid.remove_border(Entity::Cell(0, 0));

    assert_eq!(grid.to_string(), "0-00-1\n1-01-1");
}
