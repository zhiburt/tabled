use papergrid::{AlignmentHorizontal, Border, Borders, Entity, Indent, Settings};

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
         #***#***#",
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
         +---#***#",
    );

    let mut grid = util::new_grid::<2, 2>();
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
         #***#---+",
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
            "+---+---+",
        )
    );

    let mut grid = util::new_grid::<2, 2>();
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
         #***#***#",
    );
}

#[test]
fn grid_2x2_change_cell_border_test() {
    let mut grid = util::new_grid::<2, 2>();

    grid.set(
        Entity::Cell(0, 1),
        Settings::new().border(Border::new('*', '^', '@', '#', '~', '!', '%', '&')),
    );

    assert_eq!(
        grid.to_string(),
        "+---~***!\n\
         |0-0@0-1#\n\
         +---%^^^&\n\
         |1-0|1-1|\n\
         +---+---+",
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
         +-------+-------+",
    );

    grid.set(Entity::Global, Settings::new().text("asd    "));

    assert_eq!(
        grid.to_string(),
        "+-------+-------+\n\
         |asd    |asd    |\n\
         +-------+-------+\n\
         |asd    |asd    |\n\
         +-------+-------+",
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
         +---+-----+",
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
         +---+--------+",
    )
}

#[test]
fn grid_2x2_without_frame_test() {
    let mut grid = util::new_grid::<2, 2>();
    grid.set_borders(Borders {
        vertical_intersection: Some(' '),
        ..Default::default()
    });

    assert_eq!(
        grid.to_string(),
        "0-0 0-1\n\
         1-0 1-1",
    );

    grid.set_borders(Borders {
        vertical_intersection: Some(' '),
        horizontal: Some(' '),
        intersection: Some(' '),
        ..Default::default()
    });

    assert_eq!(
        grid.to_string(),
        concat!("0-0 0-1\n", "       \n", "1-0 1-1"),
    );
}

#[test]
fn grid_2x2_custom_border_test() {
    let mut grid = util::new_grid::<2, 2>();

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
        Settings::new().border(Border::new('*', '-', '@', '%', ' ', ' ', '+', '+')),
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
         \u{0020}*** *** ",
    )
}

#[cfg(feature = "color")]
#[test]
fn grid_2x2_ansi_border_test() {
    use owo_colors::OwoColorize;
    use papergrid::{Border, Symbol};

    let mut grid = util::new_grid::<2, 2>();

    let top = Symbol::ansi("*".green().on_red().to_string()).unwrap();
    let bottom = Symbol::ansi("#".on_green().blue().to_string()).unwrap();
    let left = Symbol::ansi("~".on_red().white().to_string()).unwrap();
    let right = Symbol::ansi("!".on_red().green().to_string()).unwrap();
    let top_left = Symbol::ansi("@".magenta().to_string()).unwrap();
    let top_right = Symbol::ansi("$".on_blue().to_string()).unwrap();
    let bottom_left = Symbol::ansi("%".yellow().to_string()).unwrap();
    let bottom_right = Symbol::ansi("^".on_yellow().to_string()).unwrap();

    grid.set_colored_border(
        Entity::Global,
        Border::new(
            top,
            bottom,
            left,
            right,
            top_left,
            top_right,
            bottom_left,
            bottom_right,
        ),
    );

    assert_eq!(
        grid.to_string(),
        concat!(
            "\u{1b}[35m@\u{1b}[39m\u{1b}[32m\u{1b}[41m***\u{1b}[39m\u{1b}[49m\u{1b}[35m@\u{1b}[39m\u{1b}[32m\u{1b}[41m***\u{1b}[39m\u{1b}[49m\u{1b}[44m$\u{1b}[49m\n",
            "\u{1b}[37m\u{1b}[41m~\u{1b}[39m\u{1b}[49m0-0\u{1b}[37m\u{1b}[41m~\u{1b}[39m\u{1b}[49m0-1\u{1b}[32m\u{1b}[41m!\u{1b}[39m\u{1b}[49m\n",
            "\u{1b}[35m@\u{1b}[39m\u{1b}[32m\u{1b}[41m***\u{1b}[39m\u{1b}[49m\u{1b}[35m@\u{1b}[39m\u{1b}[32m\u{1b}[41m***\u{1b}[39m\u{1b}[49m\u{1b}[44m$\u{1b}[49m\n",
            "\u{1b}[37m\u{1b}[41m~\u{1b}[39m\u{1b}[49m1-0\u{1b}[37m\u{1b}[41m~\u{1b}[39m\u{1b}[49m1-1\u{1b}[32m\u{1b}[41m!\u{1b}[39m\u{1b}[49m\n",
            "\u{1b}[33m%\u{1b}[39m\u{1b}[34m\u{1b}[42m###\u{1b}[39m\u{1b}[49m\u{1b}[33m%\u{1b}[39m\u{1b}[34m\u{1b}[42m###\u{1b}[39m\u{1b}[49m\u{1b}[43m^\u{1b}[49m",
        )
    )
}

#[cfg(feature = "color")]
#[test]
fn grid_2x2_ansi_global_set_test() {
    use std::convert::TryFrom;

    use owo_colors::OwoColorize;
    use papergrid::BorderColor;

    let color = " ".on_blue().red().bold().to_string();

    let mut grid = util::new_grid::<2, 2>();

    grid.set_border_color(BorderColor::try_from(color).unwrap());

    assert_eq!(
        grid.to_string(),
        concat!(
            "\u{1b}[1m\u{1b}[31m\u{1b}[44m+---+---+\u{1b}[22m\u{1b}[39m\u{1b}[49m\n",
            "\u{1b}[1m\u{1b}[31m\u{1b}[44m|\u{1b}[22m\u{1b}[39m\u{1b}[49m0-0\u{1b}[1m\u{1b}[31m\u{1b}[44m|\u{1b}[22m\u{1b}[39m\u{1b}[49m0-1\u{1b}[1m\u{1b}[31m\u{1b}[44m|\u{1b}[22m\u{1b}[39m\u{1b}[49m\n",
            "\u{1b}[1m\u{1b}[31m\u{1b}[44m+---+---+\u{1b}[22m\u{1b}[39m\u{1b}[49m\n",
            "\u{1b}[1m\u{1b}[31m\u{1b}[44m|\u{1b}[22m\u{1b}[39m\u{1b}[49m1-0\u{1b}[1m\u{1b}[31m\u{1b}[44m|\u{1b}[22m\u{1b}[39m\u{1b}[49m1-1\u{1b}[1m\u{1b}[31m\u{1b}[44m|\u{1b}[22m\u{1b}[39m\u{1b}[49m\n",
            "\u{1b}[1m\u{1b}[31m\u{1b}[44m+---+---+\u{1b}[22m\u{1b}[39m\u{1b}[49m",
        )
    )
}

#[cfg(feature = "color")]
#[test]
fn grid_2x2_ansi_border_none_if_string_is_not_1_char_test() {
    use owo_colors::OwoColorize;
    use papergrid::Symbol;

    assert!(Symbol::ansi("12").is_none());
    assert!(Symbol::ansi("123").is_none());
    assert!(Symbol::ansi("").is_none());

    assert!(Symbol::ansi("1").is_some());
    assert!(Symbol::ansi("1".on_red().to_string()).is_some());
    assert!(Symbol::ansi("1".on_red().blue().to_string()).is_some());
    assert!(Symbol::ansi("1".truecolor(0, 1, 3).on_truecolor(1, 2, 3).to_string()).is_some());
}

#[test]
fn when_border_is_not_complet_default_char_is_used_test() {
    let mut grid = util::new_grid::<2, 2>();
    grid.set_borders(Borders {
        vertical_intersection: Some(' '),
        ..Default::default()
    });
    grid.set(
        Entity::Cell(1, 1),
        Settings::default().border(Border::default().top('*')),
    );

    assert_eq!(
        grid.to_string(),
        concat!("0-0 0-1\n", "    ***\n", "1-0 1-1"),
    );
}

#[test]
fn when_1_vertical_is_set_second_must_use_default_test() {
    let mut grid = util::new_grid::<2, 2>();
    grid.set_borders(Borders::default());
    grid.set(
        Entity::Cell(1, 0),
        Settings::default().border(Border::default().right('*')),
    );

    assert_eq!(
        grid.to_string(),
        "0-0 0-1\n\
         1-0*1-1",
    );
}
