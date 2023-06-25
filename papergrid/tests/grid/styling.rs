#![cfg(feature = "std")]

use papergrid::config::{AlignmentHorizontal, Border, Borders, Entity, Indent, Sides};

use crate::util::grid;
use testing_table::test_table;

#[cfg(feature = "color")]
use ::{owo_colors::OwoColorize, papergrid::color::AnsiColor, std::convert::TryFrom};

test_table!(
    grid_2x2_custom_frame_test,
    grid(2, 2)
        .config(|cfg| (0..2).for_each(|r| (0..2).for_each(|c| cfg.set_border((r, c), Border::full('*', '*', '|', '|', '#', '#', '#', '#')))))
        .build(),
    "#***#***#"
    "|0-0|0-1|"
    "#***#***#"
    "|1-0|1-1|"
    "#***#***#"
);

test_table!(
    grid_2x2_custom_column_test_0,
    grid(2, 2)
        .config(|cfg| (0..2).for_each(|r| cfg.set_border((r, 1), Border::full('*', '*', '|', '|', '#', '#', '#', '#'))))
        .build(),
    "+---#***#"
    "|0-0|0-1|"
    "+---#***#"
    "|1-0|1-1|"
    "+---#***#"
);

test_table!(
    grid_2x2_custom_column_test_1,
    grid(2, 2)
        .config(|cfg| (0..2).for_each(|r| cfg.set_border((r, 0), Border::full('*', '*', '|', '|', '#', '#', '#', '#'))))
        .build(),
    "#***#---+"
    "|0-0|0-1|"
    "#***#---+"
    "|1-0|1-1|"
    "#***#---+"
);

test_table!(
    grid_2x2_custom_row_test_0,
    grid(2, 2)
        .config(|cfg| (0..2).for_each(|c| cfg.set_border((0, c), Border::full('*', '*', '|', '|', '#', '#', '#', '#'))))
        .build(),
    "#***#***#"
    "|0-0|0-1|"
    "#***#***#"
    "|1-0|1-1|"
    "+---+---+"
);

test_table!(
    grid_2x2_custom_row_test_1,
    grid(2, 2)
        .config(|cfg| (0..2).for_each(|c| cfg.set_border((1, c), Border::full('*', '*', '|', '|', '#', '#', '#', '#'))))
        .build(),
    "+---+---+"
    "|0-0|0-1|"
    "#***#***#"
    "|1-0|1-1|"
    "#***#***#"
);

test_table!(
    grid_2x2_change_cell_border_test_0,
    grid(2, 2)
        .config(|cfg| (0..2).for_each(|_| cfg.set_border((0, 1), Border::full('*', '^', '@', '#', '~', '!', '%', '&'))))
        .build(),
    "+---~***!"
    "|0-0@0-1#"
    "+---%^^^&"
    "|1-0|1-1|"
    "+---+---+"
);

test_table!(
    grid_2x2_alignment_test_0,
    grid(2, 2)
        .change_cell((0, 0), "asd    ")
        .change_cell((0, 1), "asd    ")
        .config(|cfg| {
            cfg.set_alignment_horizontal(Entity::Column(0), AlignmentHorizontal::Left);
            cfg.set_alignment_horizontal(Entity::Column(1), AlignmentHorizontal::Right);
        })
        .build(),
    "+-------+-------+"
    "|asd    |asd    |"
    "+-------+-------+"
    "|1-0    |    1-1|"
    "+-------+-------+"
);

test_table!(
    grid_2x2_alignment_test_1,
    grid(2, 2)
        .data([["asd    ", "asd    "], ["asd    ", "asd    "]])
        .config(|cfg| {
            cfg.set_alignment_horizontal(Entity::Column(0), AlignmentHorizontal::Left);
            cfg.set_alignment_horizontal(Entity::Column(1), AlignmentHorizontal::Right);
        })
        .build(),
    "+-------+-------+"
    "|asd    |asd    |"
    "+-------+-------+"
    "|asd    |asd    |"
    "+-------+-------+"
);

test_table!(
    grid_2x2_indent_test,
    grid(2, 2)
        .config(|cfg| {
            cfg.set_padding(
                Entity::Global,
                Sides::new(
                    Indent::spaced(1),
                    Indent::spaced(1),
                    Indent::spaced(1),
                    Indent::spaced(1),
                ),
            );
            cfg.set_padding(Entity::Column(0), Sides::new(
                Indent::default(),
                Indent::default(),
                Indent::default(),
                Indent::default(),
            ));
        })
        .build(),
    "+---+-----+"
    "|0-0|     |"
    "|   | 0-1 |"
    "|   |     |"
    "+---+-----+"
    "|1-0|     |"
    "|   | 1-1 |"
    "|   |     |"
    "+---+-----+"
);

test_table!(
    grid_2x2_vertical_resize_test,
    grid(2, 2).change_cell((1, 1), "asd     ").build(),
    "+---+--------+"
    "|0-0|0-1     |"
    "+---+--------+"
    "|1-0|asd     |"
    "+---+--------+"
);

test_table!(
    grid_2x2_without_frame_test_0,
    grid(2, 2)
        .config(|cfg| {
            cfg.set_borders(Borders {
                vertical: Some(' '),
                ..Default::default()
            });
        })
        .build(),
    "0-0 0-1"
    "1-0 1-1"
);

test_table!(
    grid_2x2_without_frame_test_1,
    grid(2, 2)
        .config(|cfg| {
            cfg.set_borders(Borders {
                vertical: Some(' '),
                horizontal: Some(' '),
                intersection: Some(' '),
                ..Default::default()
            });
        })
        .build(),
    "0-0 0-1"
    "       "
    "1-0 1-1"
);

test_table!(
    grid_2x2_custom_border_test,
    grid(2, 2)
        .config(|cfg| {
            cfg.set_border(
                (0, 0),
                Border {
                    bottom: Some('-'),
                    top: Some('*'),
                    left: Some('$'),
                    left_top_corner: Some(' '),
                    left_bottom_corner: Some('+'),
                    ..Default::default()
                },
            );
            cfg.set_border(
                (0, 1),
                Border::full('*', '-', '@', '%', ' ', ' ', '+', '+'),
            );
            cfg.set_border(
                (1, 0),
                Border {
                    bottom: Some('*'),
                    left: Some('#'),
                    left_top_corner: Some('+'),
                    left_bottom_corner: Some('\u{0020}'),
                    ..Default::default()
                },
            );
            cfg.set_border(
                (1, 1),
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
        })
        .build(),
    " *** *** "
    "$0-0@0-1%"
    "+---+---+"
    "#1-0^1-1!"
    "\u{0020}*** *** "
);

test_table!(
    when_border_is_not_complete_default_char_is_used_test,
    grid(2, 2)
        .config(|cfg| {
            cfg.set_borders(Borders {
                vertical: Some(' '),
                ..Default::default()
            });
            cfg.set_border(
                (1, 1),
                Border {
                    top: Some('*'),
                    ..Default::default()
                },
            );
        })
        .build(),
    "0-0 0-1"
    "    ***"
    "1-0 1-1"
);

test_table!(
    when_1_vertical_is_set_second_must_use_default_test,
    grid(2, 2)
        .config(|cfg| {
            cfg.set_borders(Borders::default());
            cfg.set_border(
                (1, 0),
                Border {
                    right: Some('*'),
                    ..Default::default()
                },
            );
        })
        .build(),
    "0-0 0-1"
    "1-0*1-1"
);

#[cfg(feature = "color")]
test_table!(
    grid_2x2_ansi_border_test,
    grid(2, 2)
        .config(|cfg| {
            (0..2).for_each(|r| (0..2).for_each(|c| {
                let top = AnsiColor::try_from(" ".green().on_red().to_string()).unwrap();
                let bottom = AnsiColor::try_from(" ".on_green().blue().to_string()).unwrap();
                let left = AnsiColor::try_from(" ".on_red().white().to_string()).unwrap();
                let right = AnsiColor::try_from(" ".on_red().green().to_string()).unwrap();
                let tl = AnsiColor::try_from(" ".magenta().to_string()).unwrap();
                let tr = AnsiColor::try_from(" ".on_blue().to_string()).unwrap();
                let bl = AnsiColor::try_from(" ".yellow().to_string()).unwrap();
                let br = AnsiColor::try_from(" ".on_yellow().to_string()).unwrap();

                cfg.set_border((r, c), Border::full('*', '#', '~', '!', '@', '$', '%', '^'));
                cfg.set_border_color((r, c), Border::full(top, bottom, left, right, tl, tr, bl, br));
            }))
        })
        .build(),
    "\u{1b}[35m@\u{1b}[39m\u{1b}[32m\u{1b}[41m***\u{1b}[39m\u{1b}[49m\u{1b}[35m@\u{1b}[39m\u{1b}[32m\u{1b}[41m***\u{1b}[39m\u{1b}[49m\u{1b}[44m$\u{1b}[49m"
    "\u{1b}[37m\u{1b}[41m~\u{1b}[39m\u{1b}[49m0-0\u{1b}[37m\u{1b}[41m~\u{1b}[39m\u{1b}[49m0-1\u{1b}[32m\u{1b}[41m!\u{1b}[39m\u{1b}[49m"
    "\u{1b}[35m@\u{1b}[39m\u{1b}[32m\u{1b}[41m***\u{1b}[39m\u{1b}[49m\u{1b}[35m@\u{1b}[39m\u{1b}[32m\u{1b}[41m***\u{1b}[39m\u{1b}[49m\u{1b}[44m$\u{1b}[49m"
    "\u{1b}[37m\u{1b}[41m~\u{1b}[39m\u{1b}[49m1-0\u{1b}[37m\u{1b}[41m~\u{1b}[39m\u{1b}[49m1-1\u{1b}[32m\u{1b}[41m!\u{1b}[39m\u{1b}[49m"
    "\u{1b}[33m%\u{1b}[39m\u{1b}[34m\u{1b}[42m###\u{1b}[39m\u{1b}[49m\u{1b}[33m%\u{1b}[39m\u{1b}[34m\u{1b}[42m###\u{1b}[39m\u{1b}[49m\u{1b}[43m^\u{1b}[49m"
);

#[cfg(feature = "color")]
test_table!(
    grid_2x2_ansi_global_set_test,
    grid(2, 2)
        .config(|cfg| {
            let color = " ".on_blue().red().bold().to_string();
            cfg.set_border_color_global(AnsiColor::try_from(color).unwrap());
        })
        .build(),
    "\u{1b}[1m\u{1b}[31m\u{1b}[44m+---+---+\u{1b}[22m\u{1b}[39m\u{1b}[49m"
    "\u{1b}[1m\u{1b}[31m\u{1b}[44m|\u{1b}[22m\u{1b}[39m\u{1b}[49m0-0\u{1b}[1m\u{1b}[31m\u{1b}[44m|\u{1b}[22m\u{1b}[39m\u{1b}[49m0-1\u{1b}[1m\u{1b}[31m\u{1b}[44m|\u{1b}[22m\u{1b}[39m\u{1b}[49m"
    "\u{1b}[1m\u{1b}[31m\u{1b}[44m+---+---+\u{1b}[22m\u{1b}[39m\u{1b}[49m"
    "\u{1b}[1m\u{1b}[31m\u{1b}[44m|\u{1b}[22m\u{1b}[39m\u{1b}[49m1-0\u{1b}[1m\u{1b}[31m\u{1b}[44m|\u{1b}[22m\u{1b}[39m\u{1b}[49m1-1\u{1b}[1m\u{1b}[31m\u{1b}[44m|\u{1b}[22m\u{1b}[39m\u{1b}[49m"
    "\u{1b}[1m\u{1b}[31m\u{1b}[44m+---+---+\u{1b}[22m\u{1b}[39m\u{1b}[49m"
);

#[cfg(feature = "color")]
#[test]
fn grid_2x2_ansi_border_none_if_string_is_not_1_char_test() {
    assert!(AnsiColor::try_from("12").is_ok());
    assert!(AnsiColor::try_from("123").is_ok());
    assert!(AnsiColor::try_from("").is_err());

    assert!(AnsiColor::try_from("1").is_ok());
    assert!(AnsiColor::try_from("1".on_red().to_string()).is_ok());
    assert!(AnsiColor::try_from("1".on_red().blue().to_string()).is_ok());
    assert!(AnsiColor::try_from("1".truecolor(0, 1, 3).on_truecolor(1, 2, 3).to_string()).is_ok());
}
