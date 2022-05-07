use papergrid::{AlignmentHorizontal, Border, Entity, Indent, Settings, DEFAULT_CELL_STYLE};

mod util;

#[test]
fn grid_2x2_custom_frame_test() {
    let mut grid = util::new_grid::<2, 2>();
    grid.set(
        Entity::Global,
        Settings::new().border(Border::new('*', '*', '|', '|', '#', '#', '#', '#')),
    );

    assert_eq!(
        grid.to_string(),
        "#***#***#\n\
         |0-0|0-1|\n\
         #***#***#\n\
         |1-0|1-1|\n\
         #***#***#\n"
    )
}

#[test]
fn grid_2x2_custom_column_test() {
    let mut grid = util::new_grid::<2, 2>();
    grid.set(
        Entity::Column(1),
        Settings::new().border(Border::new('*', '*', '|', '|', '#', '#', '#', '#')),
    );

    assert_eq!(
        grid.to_string(),
        "+---#***#\n\
         |0-0|0-1|\n\
         +---#***#\n\
         |1-0|1-1|\n\
         +---#***#\n",
    );

    grid.set_cell_borders(DEFAULT_CELL_STYLE);
    grid.set(
        Entity::Column(0),
        Settings::new().border(Border::new('*', '*', '|', '|', '#', '#', '#', '#')),
    );

    assert_eq!(
        grid.to_string(),
        "#***#---+\n\
         |0-0|0-1|\n\
         #***#---+\n\
         |1-0|1-1|\n\
         #***#---+\n",
    )
}

#[test]
fn grid_2x2_custom_row_test() {
    let mut grid = util::new_grid::<2, 2>();

    grid.set(
        Entity::Row(0),
        Settings::new().border(Border::new('*', '*', '|', '|', '#', '#', '#', '#')),
    );

    assert_eq!(
        grid.to_string(),
        concat!(
            "#***#***#\n",
            "|0-0|0-1|\n",
            "#***#***#\n",
            "|1-0|1-1|\n",
            "+---+---+\n",
        )
    );

    grid.set_cell_borders(DEFAULT_CELL_STYLE);
    grid.set(
        Entity::Row(1),
        Settings::new().border(Border::new('*', '*', '|', '|', '#', '#', '#', '#')),
    );

    let str = grid.to_string();
    assert_eq!(
        str,
        "+---+---+\n\
         |0-0|0-1|\n\
         #***#***#\n\
         |1-0|1-1|\n\
         #***#***#\n"
    );
}

#[test]
fn grid_2x2_change_cell_border_test() {
    let mut grid = util::new_grid::<2, 2>();

    grid.set(
        Entity::Cell(0, 1),
        Settings::new().border(Border::new('*', '^', '@', '#', '~', '!', '%', '&')),
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
    grid.set(
        Entity::Column(0),
        Settings::new().alignment(AlignmentHorizontal::Left),
    );
    grid.set(
        Entity::Column(1),
        Settings::new().alignment(AlignmentHorizontal::Right),
    );

    grid.set(Entity::Cell(0, 0), Settings::new().text("asd    "));
    grid.set(Entity::Cell(0, 1), Settings::new().text("asd    "));

    assert_eq!(
        grid.to_string(),
        "+-------+-------+\n\
         |asd    |asd    |\n\
         +-------+-------+\n\
         |1-0    |    1-1|\n\
         +-------+-------+\n"
    );

    grid.set(Entity::Global, Settings::new().text("asd    "));

    assert_eq!(
        grid.to_string(),
        "+-------+-------+\n\
         |asd    |asd    |\n\
         +-------+-------+\n\
         |asd    |asd    |\n\
         +-------+-------+\n"
    );
}

#[test]
fn grid_2x2_indent_test() {
    let mut grid = util::new_grid::<2, 2>();
    grid.set(
        Entity::Global,
        Settings::new().padding(
            Indent::spaced(1),
            Indent::spaced(1),
            Indent::spaced(1),
            Indent::spaced(1),
        ),
    );
    grid.set(
        Entity::Column(0),
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

    grid.set(Entity::Cell(1, 1), Settings::new().text("asd     "));

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
        Entity::Cell(0, 0),
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
        Entity::Cell(0, 1),
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
        Entity::Cell(1, 0),
        Settings::new().border(
            Border::default()
                .bottom('*')
                .left('#')
                .top_left_corner('+')
                .bottom_left_corner('\u{0020}'),
        ),
    );
    grid.set(
        Entity::Cell(1, 1),
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

#[cfg(feature = "color")]
#[test]
fn grid_2x2_ansi_border_test() {
    use colored::Colorize;
    use papergrid::Symbol;

    let mut grid = util::new_grid::<2, 2>();

    let top = Symbol::ansi("*".green().on_red().to_string()).unwrap();
    let bottom = Symbol::ansi("#".on_green().blue().to_string()).unwrap();
    let left = Symbol::ansi("~".on_red().white().to_string()).unwrap();
    let right = Symbol::ansi("!".on_red().green().to_string()).unwrap();
    let top_left = Symbol::ansi("@".magenta().to_string()).unwrap();
    let top_right = Symbol::ansi("$".on_blue().to_string()).unwrap();
    let bottom_left = Symbol::ansi("%".yellow().to_string()).unwrap();
    let bottom_right = Symbol::ansi("^".on_yellow().to_string()).unwrap();

    eprintln!("{:?}", "*".green().on_red().to_string());
    eprintln!("{:?}", "*".on_red().to_string());
    eprintln!("{:?}", "*".green().to_string());

    grid.set(
        Entity::Global,
        Settings::new().border(Border::new(
            top,
            bottom,
            left,
            right,
            top_left,
            top_right,
            bottom_left,
            bottom_right,
        )),
    );

    assert_eq!(
            grid.to_string(),
            concat!(
                "\u{1b}[35m@\u{1b}[0m\u{1b}[41;32m*\u{1b}[0m\u{1b}[41;32m*\u{1b}[0m\u{1b}[41;32m*\u{1b}[0m\u{1b}[35m@\u{1b}[0m\u{1b}[41;32m*\u{1b}[0m\u{1b}[41;32m*\u{1b}[0m\u{1b}[41;32m*\u{1b}[0m\u{1b}[44m$\u{1b}[0m\n",
                "\u{1b}[41;37m~\u{1b}[0m0-0\u{1b}[41;37m~\u{1b}[0m0-1\u{1b}[41;32m!\u{1b}[0m\n",
                "\u{1b}[35m@\u{1b}[0m\u{1b}[41;32m*\u{1b}[0m\u{1b}[41;32m*\u{1b}[0m\u{1b}[41;32m*\u{1b}[0m\u{1b}[35m@\u{1b}[0m\u{1b}[41;32m*\u{1b}[0m\u{1b}[41;32m*\u{1b}[0m\u{1b}[41;32m*\u{1b}[0m\u{1b}[44m$\u{1b}[0m\n",
                "\u{1b}[41;37m~\u{1b}[0m1-0\u{1b}[41;37m~\u{1b}[0m1-1\u{1b}[41;32m!\u{1b}[0m\n",
                "\u{1b}[33m%\u{1b}[0m\u{1b}[42;34m#\u{1b}[0m\u{1b}[42;34m#\u{1b}[0m\u{1b}[42;34m#\u{1b}[0m\u{1b}[33m%\u{1b}[0m\u{1b}[42;34m#\u{1b}[0m\u{1b}[42;34m#\u{1b}[0m\u{1b}[42;34m#\u{1b}[0m\u{1b}[43m^\u{1b}[0m\n",
            )
        )
}

#[cfg(feature = "color")]
#[test]
fn grid_2x2_ansi_border_none_if_string_is_not_1_char_test() {
    use colored::Colorize;
    use papergrid::Symbol;

    assert!(Symbol::ansi("12".to_string()).is_none());
    assert!(Symbol::ansi("123".to_string()).is_none());
    assert!(Symbol::ansi("".to_string()).is_none());

    assert!(Symbol::ansi("1".to_string()).is_some());
    assert!(Symbol::ansi("1".on_red().to_string()).is_some());
    assert!(Symbol::ansi("1".on_red().blue().to_string()).is_some());
    assert!(Symbol::ansi("1".truecolor(0, 1, 3).on_truecolor(1, 2, 3).to_string()).is_some());
}
