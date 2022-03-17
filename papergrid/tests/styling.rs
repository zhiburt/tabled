use papergrid::{AlignmentHorizontal, Border, Entity, Indent, Settings, DEFAULT_CELL_STYLE};

mod util;

#[test]
fn grid_2x2_custom_frame_test() {
    let mut grid = util::new_grid::<2, 2>();
    grid.set(
        &Entity::Global,
        Settings::new().border(Border::full('*', '*', '|', '|', '#', '#', '#', '#')),
    );

    assert_eq!(
        grid.to_string(),
        "#*******#\n\
         |0-0|0-1|\n\
         |---+---|\n\
         |1-0|1-1|\n\
         #*******#\n"
    )
}

#[test]
fn grid_2x2_custom_column_test() {
    let mut grid = util::new_grid::<2, 2>();
    grid.set(
        &Entity::Column(1),
        Settings::new().border(Border::full('*', '*', '|', '|', '#', '#', '#', '#')),
    );

    assert_eq!(
        grid.to_string(),
        "+---#***#\n\
         |0-0|0-1|\n\
         +---|---|\n\
         |1-0|1-1|\n\
         +---#***#\n"
    );

    grid.set_cell_borders(DEFAULT_CELL_STYLE.clone());
    grid.set(
        &Entity::Column(0),
        Settings::new().border(Border::full('*', '*', '|', '|', '#', '#', '#', '#')),
    );

    assert_eq!(
        grid.to_string(),
        "#***#---+\n\
         |0-0|0-1|\n\
         |---|---+\n\
         |1-0|1-1|\n\
         #***#---+\n"
    )
}

#[test]
fn grid_2x2_custom_row_test() {
    let mut grid = util::new_grid::<2, 2>();

    grid.set(
        &Entity::Row(0),
        Settings::new().border(Border::full('*', '*', '|', '|', '#', '#', '#', '#')),
    );

    assert_eq!(
        grid.to_string(),
        "#*******#\n\
         |0-0|0-1|\n\
         #*******#\n\
         |1-0|1-1|\n\
         +---+---+\n"
    );

    grid.set_cell_borders(DEFAULT_CELL_STYLE.clone());
    grid.set(
        &Entity::Row(1),
        Settings::new().border(Border::full('*', '*', '|', '|', '#', '#', '#', '#')),
    );

    let str = grid.to_string();
    assert_eq!(
        str,
        "+---+---+\n\
         |0-0|0-1|\n\
         #*******#\n\
         |1-0|1-1|\n\
         #*******#\n"
    );
}

#[test]
fn grid_2x2_change_cell_border_test() {
    let mut grid = util::new_grid::<2, 2>();

    grid.set(
        &Entity::Cell(0, 1),
        Settings::new().border(Border::full('*', '^', '@', '#', '~', '!', '%', '&')),
    );
    let str = grid.to_string();
    assert_eq!(
        str,
        "+---~***!\n\
         |0-0@0-1#\n\
         +---%^^^&\n\
         |1-0|1-1|\n\
         +---+---+\n"
    )
}

#[test]
fn grid_2x2_alignment_test() {
    let mut grid = util::new_grid::<2, 2>();
    grid.set(&Entity::Global, Settings::new().text("asd    "));
    grid.set(
        &Entity::Column(0),
        Settings::new().alignment(AlignmentHorizontal::Left),
    );
    grid.set(
        &Entity::Column(1),
        Settings::new().alignment(AlignmentHorizontal::Right),
    );
    let str = grid.to_string();

    assert_eq!(
        str,
        "+-------+-------+\n\
         |asd    |    asd|\n\
         +-------+-------+\n\
         |asd    |    asd|\n\
         +-------+-------+\n"
    )
}

#[test]
fn grid_2x2_indent_test() {
    let mut grid = util::new_grid::<2, 2>();
    grid.set(
        &Entity::Global,
        Settings::new().padding(
            Indent::spaced(1),
            Indent::spaced(1),
            Indent::spaced(1),
            Indent::spaced(1),
        ),
    );
    grid.set(
        &Entity::Column(0),
        Settings::new().padding(
            Indent::default(),
            Indent::default(),
            Indent::default(),
            Indent::default(),
        ),
    );

    let str = grid.to_string();

    assert_eq!(
        str,
        "+---+-----+\n\
         |0-0|     |\n\
         |   | 0-1 |\n\
         |   |     |\n\
         +---+-----+\n\
         |1-0|     |\n\
         |   | 1-1 |\n\
         |   |     |\n\
         +---+-----+\n"
    )
}

#[test]
fn grid_2x2_vertical_resize_test() {
    let mut grid = util::new_grid::<2, 2>();

    grid.set(&Entity::Cell(1, 1), Settings::new().text("asd     "));

    let str = grid.to_string();
    assert_eq!(
        str,
        "+---+--------+\n\
         |0-0|0-1     |\n\
         +---+--------+\n\
         |1-0|asd     |\n\
         +---+--------+\n"
    )
}

#[test]
fn grid_2x2_without_frame_test() {
    let mut grid = util::new_grid::<2, 2>();
    grid.set_cell_borders(Border::default());
    grid.clear_split_grid();

    grid.add_vertical_split(1);

    assert_eq!(
        grid.to_string(),
        "0-0 0-1\n\
         1-0 1-1\n"
    );

    grid.add_horizontal_split(1);

    assert_eq!(
        grid.to_string(),
        concat!("0-0 0-1\n", "       \n", "1-0 1-1\n",),
    );
}

#[test]
fn grid_2x2_custom_border_test() {
    let mut grid = util::new_grid::<2, 2>();

    grid.add_grid_split();
    grid.set(
        &Entity::Cell(0, 0),
        Settings::new().border(
            Border::default()
                .top('*')
                .bottom('-')
                .left('$')
                .top_left_corner(' ')
                .bottom_left_corner('+'),
        ),
    );
    grid.set(
        &Entity::Cell(0, 1),
        Settings::new().border(
            Border::default()
                .top('*')
                .bottom('-')
                .left('@')
                .top_left_corner(' ')
                .bottom_left_corner('+')
                .right('%')
                .top_right_corner(' ')
                .bottom_right_corner('+'),
        ),
    );
    grid.set(
        &Entity::Cell(1, 0),
        Settings::new().border(
            Border::default()
                .bottom('*')
                .left('#')
                .top_left_corner('+')
                .bottom_left_corner('\u{0020}'),
        ),
    );
    grid.set(
        &Entity::Cell(1, 1),
        Settings::new().border(
            Border::default()
                .bottom('*')
                .left('^')
                .top_left_corner('+')
                .bottom_left_corner(' ')
                .right('!')
                .top_right_corner('+')
                .bottom_right_corner(' '),
        ),
    );

    let str = grid.to_string();
    assert_eq!(
        str,
        " *** *** \n\
         $0-0@0-1%\n\
         +---+---+\n\
         #1-0^1-1!\n\
         \u{0020}*** *** \n"
    )
}
