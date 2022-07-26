use papergrid::{AlignmentHorizontal, Border, Borders, Entity, Indent, Padding};

mod util;

#[test]
fn grid_2x2_custom_frame_test() {
    let mut grid = util::grid::<2, 2>();
    grid.set_border(
        Entity::Global,
        Border::new('*', '*', '|', '|', '#', '#', '#', '#'),
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
    let mut grid = util::grid::<2, 2>();
    grid.set_border(
        Entity::Column(1),
        Border::new('*', '*', '|', '|', '#', '#', '#', '#'),
    );

    assert_eq!(
        grid.to_string(),
        "+---#***#\n\
         |0-0|0-1|\n\
         +---#***#\n\
         |1-0|1-1|\n\
         +---#***#",
    );

    let mut grid = util::grid::<2, 2>();
    grid.set_border(
        Entity::Column(0),
        Border::new('*', '*', '|', '|', '#', '#', '#', '#'),
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
    let mut grid = util::grid::<2, 2>();
    grid.set_border(
        Entity::Row(0),
        Border::new('*', '*', '|', '|', '#', '#', '#', '#'),
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

    let mut grid = util::grid::<2, 2>();
    grid.set_border(
        Entity::Row(1),
        Border::new('*', '*', '|', '|', '#', '#', '#', '#'),
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
    let mut grid = util::grid::<2, 2>();

    grid.set_border(
        Entity::Cell(0, 1),
        Border::new('*', '^', '@', '#', '~', '!', '%', '&'),
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
    let mut grid = util::grid_with_data::<2, 2>(&[((0, 0), "asd    "), ((0, 1), "asd    ")]);
    grid.set_alignment_horizontal(Entity::Column(0), AlignmentHorizontal::Left);
    grid.set_alignment_horizontal(Entity::Column(1), AlignmentHorizontal::Right);

    assert_eq!(
        grid.to_string(),
        "+-------+-------+\n\
         |asd    |asd    |\n\
         +-------+-------+\n\
         |1-0    |    1-1|\n\
         +-------+-------+",
    );

    let mut grid = util::grid_const::<2, 2>("asd    ");
    grid.set_alignment_horizontal(Entity::Column(0), AlignmentHorizontal::Left);
    grid.set_alignment_horizontal(Entity::Column(1), AlignmentHorizontal::Right);

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
    let mut grid = util::grid::<2, 2>();
    grid.set_padding(
        Entity::Global,
        Padding::new(
            Indent::spaced(1),
            Indent::spaced(1),
            Indent::spaced(1),
            Indent::spaced(1),
        ),
    );

    grid.set_padding(Entity::Column(0), Padding::default());

    assert_eq!(
        grid.to_string(),
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
    let grid = util::grid_with_data::<2, 2>(&[((1, 1), "asd     ")]);

    assert_eq!(
        grid.to_string(),
        "+---+--------+\n\
         |0-0|0-1     |\n\
         +---+--------+\n\
         |1-0|asd     |\n\
         +---+--------+",
    )
}

#[test]
fn grid_2x2_without_frame_test() {
    let mut grid = util::grid::<2, 2>();
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
    let mut grid = util::grid::<2, 2>();

    grid.set_border(
        Entity::Cell(0, 0),
        Border {
            bottom: Some('-'),
            top: Some('*'),
            left: Some('$'),
            left_top_corner: Some(' '),
            left_bottom_corner: Some('+'),
            ..Default::default()
        },
    );
    grid.set_border(
        Entity::Cell(0, 1),
        Border::new('*', '-', '@', '%', ' ', ' ', '+', '+'),
    );
    grid.set_border(
        Entity::Cell(1, 0),
        Border {
            bottom: Some('*'),
            left: Some('#'),
            left_top_corner: Some('+'),
            left_bottom_corner: Some('\u{0020}'),
            ..Default::default()
        },
    );
    grid.set_border(
        Entity::Cell(1, 1),
        Border {
            bottom: Some('*'),
            left: Some('^'),
            left_top_corner: Some('+'),
            right_top_corner: Some('+'),
            right: Some('!'),
            left_bottom_corner: Some(' '),
            right_bottom_corner: Some(' '),
            ..Default::default()
        },
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

#[test]
fn when_border_is_not_complet_default_char_is_used_test() {
    let mut grid = util::grid::<2, 2>();
    grid.set_borders(Borders {
        vertical_intersection: Some(' '),
        ..Default::default()
    });
    grid.set_border(
        Entity::Cell(1, 1),
        Border {
            top: Some('*'),
            ..Default::default()
        },
    );

    assert_eq!(
        grid.to_string(),
        concat!("0-0 0-1\n", "    ***\n", "1-0 1-1"),
    );
}

#[test]
fn when_1_vertical_is_set_second_must_use_default_test() {
    let mut grid = util::grid::<2, 2>();
    grid.set_borders(Borders::default());
    grid.set_border(
        Entity::Cell(1, 0),
        Border {
            right: Some('*'),
            ..Default::default()
        },
    );

    assert_eq!(
        grid.to_string(),
        "0-0 0-1\n\
         1-0*1-1",
    );
}

#[cfg(feature = "color")]
#[test]
fn grid_2x2_ansi_border_test() {
    use owo_colors::OwoColorize;
    use papergrid::{Border, Symbol};

    let mut grid = util::grid::<2, 2>();

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
    use papergrid::Color;

    let color = " ".on_blue().red().bold().to_string();

    let mut grid = util::grid::<2, 2>();

    grid.set_border_color(Color::try_from(color).unwrap());

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
