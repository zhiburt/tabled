// Copyright (c) 2021 Maxim Zhiburt
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

use papergrid::{AlignmentHorizontal, AlignmentVertical, Borders, Entity, Grid, Indent, Settings};

mod util;

#[test]
fn render_2x2_test() {
    let grid = util::new_grid::<2, 2>();

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+\n",
            "|0-0|0-1|\n",
            "+---+---+\n",
            "|1-0|1-1|\n",
            "+---+---+\n",
        )
    );
}

#[test]
fn render_1x1_test() {
    let mut grid = util::new_grid::<1, 1>();
    grid.set(Entity::Cell(0, 0), Settings::new().text("one line"));

    assert_eq!(
        grid.to_string(),
        concat!("+--------+\n", "|one line|\n", "+--------+\n")
    );
}

#[test]
fn render_3x2_test() {
    let mut grid = util::new_grid::<3, 2>();
    grid.set(Entity::Global, Settings::new().text("asd"));

    assert_eq!(
        grid.to_string(),
        "+---+---+\n\
         |asd|asd|\n\
         +---+---+\n\
         |asd|asd|\n\
         +---+---+\n\
         |asd|asd|\n\
         +---+---+\n"
    )
}

#[test]
fn render_not_quadratic() {
    let mut grid = util::new_grid::<1, 2>();

    grid.set(Entity::Cell(0, 0), Settings::new().text("hello"));
    grid.set(Entity::Cell(0, 1), Settings::new().text("world"));

    assert_eq!(
        grid.to_string(),
        concat!("+-----+-----+\n", "|hello|world|\n", "+-----+-----+\n")
    );
}

#[test]
fn render_empty() {
    let grid = Grid::new(0, 0);
    assert_eq!("", grid.to_string());
}

#[test]
fn render_multilane() {
    let mut grid = util::new_grid::<2, 2>();

    grid.set(Entity::Cell(0, 0), Settings::new().text("left\ncell"));
    grid.set(Entity::Cell(0, 1), Settings::new().text("right one"));
    grid.set(
        Entity::Cell(1, 0),
        Settings::new().text("the second column got the beginning here"),
    );
    grid.set(
        Entity::Cell(1, 1),
        Settings::new().text("and here\nwe\nsee\na\nlong\nstring"),
    );

    assert_eq!(
        grid.to_string(),
        concat!(
            "+----------------------------------------+---------+\n",
            "|left                                    |right one|\n",
            "|cell                                    |         |\n",
            "+----------------------------------------+---------+\n",
            "|the second column got the beginning here|and here |\n",
            "|                                        |we       |\n",
            "|                                        |see      |\n",
            "|                                        |a        |\n",
            "|                                        |long     |\n",
            "|                                        |string   |\n",
            "+----------------------------------------+---------+\n"
        )
    );
}

#[test]
fn render_multilane_alignment() {
    let mut grid = util::new_grid::<2, 2>();

    grid.set(
        Entity::Cell(0, 0),
        Settings::new()
            .text("left\ncell")
            .alignment(AlignmentHorizontal::Center),
    );
    grid.set(Entity::Cell(0, 1), Settings::new().text("right one"));
    grid.set(
        Entity::Cell(1, 0),
        Settings::new().text("the second column got the beginning here"),
    );
    grid.set(
        Entity::Cell(1, 1),
        Settings::new()
            .text("and here\nwe\nsee\na\nlong\nstring")
            .alignment(AlignmentHorizontal::Right),
    );

    assert_eq!(
        grid.to_string(),
        concat!(
            "+----------------------------------------+---------+\n\
         |                  left                  |right one|\n\
         |                  cell                  |         |\n\
         +----------------------------------------+---------+\n\
         |the second column got the beginning here| and here|\n\
         |                                        | we      |\n\
         |                                        | see     |\n\
         |                                        | a       |\n\
         |                                        | long    |\n\
         |                                        | string  |\n\
         +----------------------------------------+---------+\n"
        )
    );
}

#[test]
fn render_multilane_vertical_alignment() {
    let mut grid = util::new_grid::<2, 2>();

    grid.set(
        Entity::Cell(0, 0),
        Settings::new()
            .text("left\ncell")
            .alignment(AlignmentHorizontal::Center),
    );
    grid.set(Entity::Cell(0, 1), Settings::new().text("right one"));
    grid.set(
        Entity::Cell(1, 0),
        Settings::new()
            .text("the second column got the beginning here")
            .vertical_alignment(AlignmentVertical::Center),
    );
    grid.set(
        Entity::Cell(1, 1),
        Settings::new()
            .text("and here\nwe\nsee\na\nlong\nstring")
            .alignment(AlignmentHorizontal::Right),
    );

    assert_eq!(
        grid.to_string(),
        concat!(
            "+----------------------------------------+---------+\n\
         |                  left                  |right one|\n\
         |                  cell                  |         |\n\
         +----------------------------------------+---------+\n\
         |                                        | and here|\n\
         |                                        | we      |\n\
         |the second column got the beginning here| see     |\n\
         |                                        | a       |\n\
         |                                        | long    |\n\
         |                                        | string  |\n\
         +----------------------------------------+---------+\n"
        )
    );
}

#[test]
fn render_empty_cell() {
    let mut grid = util::new_grid::<2, 2>();

    grid.set(Entity::Cell(0, 1), Settings::new().text(""));

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+\n",
            "|0-0|   |\n",
            "+---+---+\n",
            "|1-0|1-1|\n",
            "+---+---+\n",
        )
    );
}

#[test]
fn render_row_span() {
    let mut grid = util::new_grid::<2, 2>();

    grid.set(
        Entity::Cell(0, 0),
        Settings::new()
            .span(2)
            .alignment(AlignmentHorizontal::Center),
    );

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+\n",
            "|  0-0  |\n",
            "+---+---+\n",
            "|1-0|1-1|\n",
            "+---+---+\n"
        )
    );
}

#[test]
fn render_miltiline_span() {
    let mut grid = util::new_grid::<2, 2>();

    grid.set(
        Entity::Cell(0, 0),
        Settings::new()
            .text("0-0\n0-1")
            .span(2)
            .alignment(AlignmentHorizontal::Center),
    );

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+\n",
            "|  0-0  |\n",
            "|  0-1  |\n",
            "+---+---+\n",
            "|1-0|1-1|\n",
            "+---+---+\n"
        )
    );
}

#[test]
fn render_row_span_multilane() {
    let mut grid = util::new_grid::<4, 3>();

    grid.set(
        Entity::Cell(0, 0),
        Settings::new().text("first line").span(2),
    );
    grid.set(Entity::Cell(0, 2), Settings::new().text("e.g."));
    grid.set(Entity::Cell(1, 0), Settings::new().text("0"));
    grid.set(Entity::Cell(1, 1), Settings::new().text("1"));
    grid.set(Entity::Cell(1, 2), Settings::new().text("2"));
    grid.set(Entity::Cell(2, 0), Settings::new().text("0"));
    grid.set(Entity::Cell(2, 1), Settings::new().text("1"));
    grid.set(Entity::Cell(2, 2), Settings::new().text("2"));
    grid.set(
        Entity::Cell(3, 0),
        Settings::new().text("full last line").span(3),
    );

    assert_eq!(
        grid.to_string(),
        concat!(
            "+-----+----+----+\n",
            "|first line|e.g.|\n",
            "+-----+----+----+\n",
            "|0    |1   |2   |\n",
            "+-----+----+----+\n",
            "|0    |1   |2   |\n",
            "+-----+----+----+\n",
            "|full last line |\n",
            "+-----+----+----+\n",
        )
    );
}

#[test]
fn render_row_span_with_horizontal_ident() {
    let mut grid = util::new_grid::<3, 2>();
    grid.set(Entity::Cell(0, 0), Settings::new().span(2));
    grid.set(
        Entity::Cell(1, 0),
        Settings::new().padding(
            Indent::spaced(4),
            Indent::spaced(4),
            Indent::default(),
            Indent::default(),
        ),
    );

    assert_eq!(
        grid.to_string(),
        concat!(
            "+-----------+---+\n",
            "|0-0            |\n",
            "+-----------+---+\n",
            "|    1-0    |1-1|\n",
            "+-----------+---+\n",
            "|2-0        |2-1|\n",
            "+-----------+---+\n",
        )
    );
}

#[test]
fn render_row_span_3x3() {
    let mut grid = util::new_grid::<3, 3>();

    grid.set(Entity::Cell(0, 0), Settings::new().span(3));
    grid.set(Entity::Cell(1, 0), Settings::new().span(2));
    grid.set(Entity::Cell(2, 0), Settings::new().span(2));

    assert_eq!(
        grid.to_string(),
        concat!(
            "+-+-+---+\n",
            "|0-0    |\n",
            "+-+-+---+\n",
            "|1-0|1-2|\n",
            "+-+-+---+\n",
            "|2-0|2-2|\n",
            "+-+-+---+\n",
        )
    );
}

#[test]
fn render_2_colided_row_span_3x3() {
    let mut grid = util::new_grid::<3, 3>();
    grid.set(
        Entity::Cell(0, 0),
        Settings::new().text("0-0xxxxxxx").span(2),
    );
    grid.set(Entity::Cell(1, 1), Settings::new().span(2));

    assert_eq!(
        grid.to_string(),
        concat!(
            "+-----+----+---+\n",
            "|0-0xxxxxxx|0-2|\n",
            "+-----+----+---+\n",
            "|1-0  |1-1     |\n",
            "+-----+----+---+\n",
            "|2-0  |2-1 |2-2|\n",
            "+-----+----+---+\n",
        )
    );

    let mut grid = util::new_grid::<3, 3>();
    grid.set(Entity::Cell(0, 0), Settings::new().span(2));
    grid.set(
        Entity::Cell(1, 1),
        Settings::new().text("1-1xxxxxxx").span(2),
    );

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+-----+----+\n",
            "|0-0      |0-2 |\n",
            "+---+-----+----+\n",
            "|1-0|1-1xxxxxxx|\n",
            "+---+-----+----+\n",
            "|2-0|2-1  |2-2 |\n",
            "+---+-----+----+\n",
        )
    );

    let mut grid = util::new_grid::<3, 3>();
    grid.set(Entity::Cell(0, 0), Settings::new().span(2));
    grid.set(
        Entity::Cell(1, 1),
        Settings::new().text("1-1xxxxxxx").span(2),
    );
    grid.set(Entity::Cell(2, 0), Settings::new().text("2-0xxxxxxxxxxxxx"));

    assert_eq!(
        grid.to_string(),
        concat!(
            "+----------------+-----+----+\n",
            "|0-0                   |0-2 |\n",
            "+----------------+-----+----+\n",
            "|1-0             |1-1xxxxxxx|\n",
            "+----------------+-----+----+\n",
            "|2-0xxxxxxxxxxxxx|2-1  |2-2 |\n",
            "+----------------+-----+----+\n",
        )
    );

    let mut grid = util::new_grid::<3, 3>();
    grid.set(Entity::Cell(0, 0), Settings::new().span(2));
    grid.set(Entity::Cell(1, 1), Settings::new().span(2));
    grid.set(Entity::Cell(2, 1), Settings::new().text("2-1xxxxxxxxxxxxx"));

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+----------------+---+\n",
            "|0-0                 |0-2|\n",
            "+---+----------------+---+\n",
            "|1-0|1-1                 |\n",
            "+---+----------------+---+\n",
            "|2-0|2-1xxxxxxxxxxxxx|2-2|\n",
            "+---+----------------+---+\n",
        )
    );

    let mut grid = util::new_grid::<3, 3>();
    grid.set(Entity::Cell(0, 0), Settings::new().span(2));
    grid.set(Entity::Cell(0, 2), Settings::new().text("0-2xxxxxxxxxxxxx"));
    grid.set(Entity::Cell(1, 1), Settings::new().span(2));

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+----------------+\n",
            "|0-0    |0-2xxxxxxxxxxxxx|\n",
            "+---+---+----------------+\n",
            "|1-0|1-1                 |\n",
            "+---+---+----------------+\n",
            "|2-0|2-1|2-2             |\n",
            "+---+---+----------------+\n",
        )
    );
}

#[test]
fn render_spaned_column_in_first_cell_3x3() {
    let mut grid = util::new_grid::<3, 3>();

    grid.set(
        Entity::Cell(0, 0),
        Settings::new().text("0-0xxxxxxx").span(2),
    );

    assert_eq!(
        grid.to_string(),
        concat!(
            "+-----+----+---+\n",
            "|0-0xxxxxxx|0-2|\n",
            "+-----+----+---+\n",
            "|1-0  |1-1 |1-2|\n",
            "+-----+----+---+\n",
            "|2-0  |2-1 |2-2|\n",
            "+-----+----+---+\n",
        )
    );
}

#[test]
fn render_row_span_with_different_length() {
    let mut grid = util::new_grid::<3, 2>();

    grid.set(
        Entity::Cell(0, 0),
        Settings::new().text("first row").span(2),
    );
    grid.set(Entity::Cell(1, 0), Settings::new().text("0"));
    grid.set(Entity::Cell(1, 1), Settings::new().text("1"));
    grid.set(
        Entity::Cell(2, 0),
        Settings::new().text("a longer second row").span(2),
    );

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---------+---------+\n",
            "|first row          |\n",
            "+---------+---------+\n",
            "|0        |1        |\n",
            "+---------+---------+\n",
            "|a longer second row|\n",
            "+---------+---------+\n",
        )
    );
}

#[test]
fn render_row_span_with_odd_length() {
    let mut grid = util::new_grid::<2, 2>();

    grid.set(Entity::Cell(0, 0), Settings::new().text("3   ").span(2));
    grid.set(Entity::Cell(1, 0), Settings::new().text("2"));
    grid.set(Entity::Cell(1, 1), Settings::new().text("4"));

    assert_eq!(
        grid.to_string(),
        concat!("+--+-+\n", "|3   |\n", "+--+-+\n", "|2 |4|\n", "+--+-+\n")
    );
}

#[test]
fn render_only_row_spaned() {
    let mut grid = util::new_grid::<3, 2>();

    grid.set(Entity::Cell(0, 0), Settings::new().span(2));
    grid.set(Entity::Cell(1, 0), Settings::new().span(2));
    grid.set(Entity::Cell(2, 0), Settings::new().span(2));

    assert_eq!(
        grid.to_string(),
        "+-+-+\n\
         |0-0|\n\
         +-+-+\n\
         |1-0|\n\
         +-+-+\n\
         |2-0|\n\
         +-+-+\n"
    );
}

#[test]
fn grid_2x2_span_test() {
    let mut grid = util::new_grid::<2, 2>();
    grid.set(Entity::Global, Settings::new().text("asd"));
    grid.set(Entity::Cell(0, 0), Settings::new().text("123").span(2));

    assert_eq!(
        grid.to_string(),
        "+---+---+\n\
         |123    |\n\
         +---+---+\n\
         |asd|asd|\n\
         +---+---+\n"
    )
}

#[test]
fn grid_2x2_span_2_test() {
    let mut grid = util::new_grid::<2, 2>();
    grid.set(Entity::Global, Settings::new().text("asd"));
    grid.set(Entity::Cell(0, 0), Settings::new().text("1234").span(2));
    grid.set(Entity::Cell(1, 0), Settings::new().text("asdw").span(2));

    assert_eq!(
        grid.to_string(),
        "+--+-+\n\
         |1234|\n\
         +--+-+\n\
         |asdw|\n\
         +--+-+\n"
    );

    grid.set(Entity::Cell(0, 0), Settings::new().text("1"));
    grid.set(Entity::Cell(1, 0), Settings::new().text("a"));

    let str = grid.to_string();
    assert_eq!(
        str,
        "+++\n\
         |1|\n\
         +++\n\
         |a|\n\
         +++\n"
    );
}

#[test]
fn render_row_span_with_no_split_style() {
    let mut grid = util::new_grid::<2, 2>();
    grid.set_borders(Borders::default());

    grid.set(
        Entity::Cell(0, 0),
        Settings::new()
            .span(2)
            .alignment(AlignmentHorizontal::Center),
    );
    grid.set(Entity::Cell(0, 1), Settings::new().text(""));

    assert_eq!(grid.to_string(), concat!(" 0-0  \n", "1-01-1\n"));
}

#[test]
#[ignore = "This is a pretty complex logic which is not clear if is worth to support"]
fn render_zero_span_of_first_cell() {
    let mut grid = util::new_grid::<2, 2>();

    grid.set(Entity::Cell(0, 0), Settings::new().span(0));

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+\n",
            "|0-1    |\n",
            "+---+---+\n",
            "|1-0|1-1|\n",
            "+---+---+\n",
        )
    );

    grid.set(Entity::Cell(1, 0), Settings::new().span(0));

    assert_eq!(
        grid.to_string(),
        concat!("+-+-+\n", "|0-1|\n", "+-+-+\n", "|1-1|\n", "+-+-+\n")
    );
}

#[test]
fn render_zero_span_between_cells() {
    let mut grid = util::new_grid::<2, 3>();

    grid.set(Entity::Cell(0, 1), Settings::new().span(0));

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+---+\n",
            "|0-0    |0-2|\n",
            "+---+---+---+\n",
            "|1-0|1-1|1-2|\n",
            "+---+---+---+\n",
        )
    );

    grid.set(Entity::Cell(1, 1), Settings::new().span(0));

    assert_eq!(
        grid.to_string(),
        concat!(
            "+-+-+---+\n",
            "|0-0|0-2|\n",
            "+-+-+---+\n",
            "|1-0|1-2|\n",
            "+-+-+---+\n",
        )
    );
}

#[test]
fn render_zero_span_at_the_end() {
    let mut grid = util::new_grid::<2, 3>();

    grid.set(Entity::Cell(0, 1), Settings::new().span(0));
    grid.set(Entity::Cell(0, 2), Settings::new().span(0));

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+---+\n",
            "|0-0        |\n",
            "+---+---+---+\n",
            "|1-0|1-1|1-2|\n",
            "+---+---+---+\n",
        )
    );

    grid.set(Entity::Cell(1, 1), Settings::new().span(0));
    grid.set(Entity::Cell(1, 2), Settings::new().span(0));

    assert_eq!(
        grid.to_string(),
        concat!("+-+++\n", "|0-0|\n", "+-+++\n", "|1-0|\n", "+-+++\n")
    );
}

#[test]
fn render_zero_span_grid() {
    let mut grid = util::new_grid::<2, 2>();

    grid.set(Entity::Cell(0, 0), Settings::new().span(0));
    grid.set(Entity::Cell(0, 1), Settings::new().span(0));
    grid.set(Entity::Cell(1, 0), Settings::new().span(0));
    grid.set(Entity::Cell(1, 1), Settings::new().span(0));

    // todo: determine if it's correct behaviour?
    assert_eq!("+-+-+\n|0-0|\n+-+-+\n|1-0|\n+-+-+\n", grid.to_string());
}

#[test]
#[ignore = "I am not sure what is the right behaiviour here"]
fn hieroglyph_handling() {
    let mut grid = util::new_grid::<1, 2>();
    grid.set(Entity::Cell(0, 0), Settings::new().text("哈哈"));
    grid.set(Entity::Cell(0, 1), Settings::new().text("哈"));

    assert_eq!(
        grid.to_string(),
        "+----+--+\n\
         |哈哈  |哈 |\n\
         +----+--+\n"
    )
}

#[test]
fn hieroglyph_multiline_handling() {
    let mut grid = util::new_grid::<1, 2>();
    grid.set(Entity::Cell(0, 0), Settings::new().text("哈哈"));
    grid.set(Entity::Cell(0, 1), Settings::new().text("哈\n哈"));

    println!("{grid}");

    assert_eq!(
        grid.to_string(),
        "+----+--+\n\
         |哈哈|哈|\n\
         |    |哈|\n\
         +----+--+\n"
    )
}

#[test]
fn hieroglyph_handling_2() {
    let mut grid = util::new_grid::<2, 1>();
    grid.set(
        Entity::Cell(0, 0),
        Settings::new().text("জী._ডি._ব্লক_সল্টলেক_দূর্গা_পুজো_২০১৮.jpg"),
    );
    grid.set(Entity::Cell(1, 0), Settings::new().text("Hello"));

    println!("{grid}");

    assert_eq!(
        grid.to_string(),
        concat!(
            "+------------------------------------+\n",
            "|জী._ডি._ব্লক_সল্টলেক_দূর্গা_পুজো_২০১৮.jpg|\n",
            "+------------------------------------+\n",
            "|Hello                               |\n",
            "+------------------------------------+\n",
        )
    )
}

#[test]
fn render_col_span_v_alignment_center() {
    let mut grid = util::new_grid::<2, 2>();

    grid.set(
        Entity::Cell(0, 0),
        Settings::new()
            .span_vertical(2)
            .vertical_alignment(AlignmentVertical::Center),
    );

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+\n",
            "|   |0-1|\n",
            "+0-0+---+\n",
            "|   |1-1|\n",
            "+---+---+\n",
        )
    );
}

#[test]
fn render_col_span_v_alignment_top() {
    let mut grid = util::new_grid::<2, 2>();

    grid.set(
        Entity::Cell(0, 0),
        Settings::new()
            .span_vertical(2)
            .vertical_alignment(AlignmentVertical::Top),
    );

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+\n",
            "|0-0|0-1|\n",
            "+   +---+\n",
            "|   |1-1|\n",
            "+---+---+\n",
        )
    );
}

#[test]
fn render_col_span_v_alignment_bottom() {
    let mut grid = util::new_grid::<2, 2>();

    grid.set(
        Entity::Cell(0, 0),
        Settings::new()
            .span_vertical(2)
            .vertical_alignment(AlignmentVertical::Bottom),
    );

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+\n",
            "|   |0-1|\n",
            "+   +---+\n",
            "|0-0|1-1|\n",
            "+---+---+\n",
        )
    );
}

#[test]
fn render_col_span_miltiline() {
    let mut grid = util::new_grid::<2, 2>();

    grid.set(
        Entity::Cell(0, 0),
        Settings::new()
            .text("0-0\n0-1xxx")
            .span_vertical(2)
            .alignment(AlignmentHorizontal::Center),
    );

    assert_eq!(
        grid.to_string(),
        concat!(
            "+------+---+\n",
            "|0-0   |0-1|\n",
            "+0-1xxx+---+\n",
            "|      |1-1|\n",
            "+------+---+\n",
        )
    );
}

#[test]
fn render_col_span_multilane() {
    let mut grid = util::new_grid::<4, 3>();

    grid.set(
        Entity::Cell(0, 0),
        Settings::new().text("first line").span_vertical(2),
    );
    grid.set(Entity::Cell(0, 2), Settings::new().text("e.g."));
    grid.set(Entity::Cell(1, 0), Settings::new().text("0"));
    grid.set(Entity::Cell(1, 1), Settings::new().text("1"));
    grid.set(Entity::Cell(1, 2), Settings::new().text("2"));
    grid.set(Entity::Cell(2, 0), Settings::new().text("0"));
    grid.set(Entity::Cell(2, 1), Settings::new().text("1"));
    grid.set(Entity::Cell(2, 2), Settings::new().text("2"));
    grid.set(
        Entity::Cell(0, 2),
        Settings::new().text("full last line").span_vertical(4),
    );

    assert_eq!(
        grid.to_string(),
        concat!(
            "+----------+---+--------------+\n",
            "|first line|0-1|full last line|\n",
            "+          +---+              +\n",
            "|          |1  |              |\n",
            "+----------+---+              +\n",
            "|0         |1  |              |\n",
            "+----------+---+              +\n",
            "|3-0       |3-1|              |\n",
            "+----------+---+--------------+\n",
        )
    );
}

#[test]
fn render_col_span_with_horizontal_ident_on_spanned_cell() {
    let mut grid = util::new_grid::<3, 2>();
    grid.set(Entity::Cell(0, 0), Settings::new().span_vertical(2));
    grid.set(
        Entity::Cell(1, 0),
        Settings::new().padding(
            Indent::spaced(4),
            Indent::spaced(4),
            Indent::default(),
            Indent::default(),
        ),
    );

    println!("{}", grid);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+\n",
            "|0-0|0-1|\n",
            "+   +---+\n",
            "|   |1-1|\n",
            "+---+---+\n",
            "|2-0|2-1|\n",
            "+---+---+\n",
        )
    );
}

#[test]
fn render_col_span_with_horizontal_ident() {
    let mut grid = util::new_grid::<3, 2>();
    grid.set(Entity::Cell(0, 0), Settings::new().span_vertical(2));
    grid.set(
        Entity::Cell(0, 0),
        Settings::new().padding(
            Indent::spaced(4),
            Indent::spaced(4),
            Indent::default(),
            Indent::default(),
        ),
    );

    println!("{}", grid);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+-----------+---+\n",
            "|    0-0    |0-1|\n",
            "+           +---+\n",
            "|           |1-1|\n",
            "+-----------+---+\n",
            "|2-0        |2-1|\n",
            "+-----------+---+\n",
        )
    );
}

#[test]
fn render_col_span_with_vertical_ident() {
    let mut grid = util::new_grid::<3, 2>();
    grid.set(Entity::Cell(0, 0), Settings::new().span_vertical(2));
    grid.set(
        Entity::Cell(0, 0),
        Settings::new().padding(
            Indent::default(),
            Indent::default(),
            Indent::spaced(4),
            Indent::spaced(4),
        ),
    );

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+\n",
            "|   |0-1|\n",
            "|   |   |\n",
            "|   |   |\n",
            "|   |   |\n",
            "+0-0+---+\n",
            "|   |1-1|\n",
            "|   |   |\n",
            "|   |   |\n",
            "|   |   |\n",
            "+---+---+\n",
            "|2-0|2-1|\n",
            "+---+---+\n",
        )
    );
}

#[test]
fn render_col_span_3x3() {
    let mut grid = util::new_grid::<3, 3>();

    grid.set(Entity::Cell(0, 0), Settings::new().span_vertical(3));
    grid.set(Entity::Cell(0, 1), Settings::new().span_vertical(2));
    grid.set(Entity::Cell(0, 2), Settings::new().span_vertical(2));

    println!("{}", grid);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+---+\n",
            "|0-0|0-1|0-2|\n",
            "+   +---+---+\n",
            "|   |2-1|2-2|\n",
            "+---+---+---+\n",
        )
    );
}

#[test]
fn render_2_colided_col_span_3x3() {
    let mut grid = util::new_grid::<3, 3>();
    grid.set(
        Entity::Cell(0, 0),
        Settings::new().text("0-0xxxxxxx").span_vertical(2),
    );
    grid.set(Entity::Cell(1, 1), Settings::new().span_vertical(2));

    assert_eq!(
        grid.to_string(),
        concat!(
            "+----------+---+---+\n",
            "|0-0xxxxxxx|0-1|0-2|\n",
            "+          +---+---+\n",
            "|          |1-1|1-2|\n",
            "+----------+   +---+\n",
            "|2-0       |   |2-2|\n",
            "+----------+---+---+\n",
        )
    );

    let mut grid = util::new_grid::<3, 3>();
    grid.set(Entity::Cell(0, 0), Settings::new().span_vertical(2));
    grid.set(
        Entity::Cell(1, 1),
        Settings::new().text("1-1xxxxxxx").span_vertical(2),
    );

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+----------+---+\n",
            "|0-0|0-1       |0-2|\n",
            "+   +----------+---+\n",
            "|   |1-1xxxxxxx|1-2|\n",
            "+---+          +---+\n",
            "|2-0|          |2-2|\n",
            "+---+----------+---+\n",
        )
    );

    let mut grid = util::new_grid::<3, 3>();
    grid.set(Entity::Cell(0, 0), Settings::new().span_vertical(2));
    grid.set(
        Entity::Cell(1, 1),
        Settings::new().text("1-1\nx\nx\nxxxxx").span_vertical(2),
    );
    grid.set(
        Entity::Cell(0, 2),
        Settings::new().text("2-0x\nxxx\nxx\nxxxxxxx"),
    );

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+-----+-------+\n",
            "|0-0|0-1  |2-0x   |\n",
            "|   |     |xxx    |\n",
            "|   |     |xx     |\n",
            "|   |     |xxxxxxx|\n",
            "+   +-----+-------+\n",
            "|   |1-1  |1-2    |\n",
            "|   |x    |       |\n",
            "+---+x    +-------+\n",
            "|2-0|xxxxx|2-2    |\n",
            "+---+-----+-------+\n",
        )
    );

    let mut grid = util::new_grid::<3, 3>();
    grid.set(Entity::Cell(0, 0), Settings::new().span_vertical(2));
    grid.set(Entity::Cell(1, 1), Settings::new().span_vertical(2));
    grid.set(
        Entity::Cell(1, 2),
        Settings::new().text("2-1\nxx\nxx\nxx\nxxxxxx\nx"),
    );

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+------+\n",
            "|0-0|0-1|0-2   |\n",
            "+   +---+------+\n",
            "|   |1-1|2-1   |\n",
            "|   |   |xx    |\n",
            "|   |   |xx    |\n",
            "|   |   |xx    |\n",
            "|   |   |xxxxxx|\n",
            "|   |   |x     |\n",
            "+---+   +------+\n",
            "|2-0|   |2-2   |\n",
            "+---+---+------+\n",
        )
    );

    let mut grid = util::new_grid::<3, 3>();
    grid.set(Entity::Cell(0, 0), Settings::new().span_vertical(2));
    grid.set(Entity::Cell(1, 2), Settings::new().span_vertical(2));
    grid.set(
        Entity::Cell(2, 1),
        Settings::new().text("0-2\nx\nx\nx\nx\nxxxxxxx\nx\nx"),
    );

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+-------+---+\n",
            "|0-0|0-1    |0-2|\n",
            "+   +-------+---+\n",
            "|   |1-1    |1-2|\n",
            "+---+-------+   +\n",
            "|2-0|0-2    |   |\n",
            "|   |x      |   |\n",
            "|   |x      |   |\n",
            "|   |x      |   |\n",
            "|   |x      |   |\n",
            "|   |xxxxxxx|   |\n",
            "|   |x      |   |\n",
            "|   |x      |   |\n",
            "+---+-------+---+\n",
        )
    );
}

#[test]
fn render_spaned_row_in_first_cell_3x3() {
    let mut grid = util::new_grid::<3, 3>();
    grid.set(
        Entity::Cell(0, 0),
        Settings::new()
            .text("0-0\nxx\nx\nx\nx\nx\nx")
            .span_vertical(2),
    );

    println!("{}", grid);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+---+\n",
            "|0-0|0-1|0-2|\n",
            "|xx |   |   |\n",
            "|x  |   |   |\n",
            "+x  +---+---+\n",
            "|x  |1-1|1-2|\n",
            "|x  |   |   |\n",
            "|x  |   |   |\n",
            "+---+---+---+\n",
            "|2-0|2-1|2-2|\n",
            "+---+---+---+\n",
        )
    );
}

#[test]
fn render_col_span_with_different_length() {
    let mut grid = util::new_grid::<2, 3>();

    grid.set(
        Entity::Cell(0, 0),
        Settings::new().text("f\nir\nst\n ro\nw").span_vertical(2),
    );
    grid.set(Entity::Cell(1, 0), Settings::new().text("0"));
    grid.set(Entity::Cell(1, 1), Settings::new().text("1"));
    grid.set(
        Entity::Cell(0, 2),
        Settings::new()
            .text("a\n \nlonger\n \nsecond\n \nrow")
            .span_vertical(2),
    );

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+------+\n",
            "|f  |0-1|a     |\n",
            "|ir |   |      |\n",
            "|st |   |longer|\n",
            "+ ro+---+      +\n",
            "|w  |1  |second|\n",
            "|   |   |      |\n",
            "|   |   |row   |\n",
            "+---+---+------+\n",
        )
    );
}

#[test]
fn render_col_span_with_odd_length() {
    let mut grid = util::new_grid::<2, 2>();

    grid.set(
        Entity::Cell(0, 0),
        Settings::new().text("3\n \n \n ").span_vertical(2),
    );
    grid.set(Entity::Cell(0, 1), Settings::new().text("2"));
    grid.set(Entity::Cell(1, 1), Settings::new().text("4"));

    println!("{}", grid);

    assert_eq!(
        grid.to_string(),
        concat!("+-+-+\n", "|3|2|\n", "| | |\n", "+ +-+\n", "| |4|\n", "+-+-+\n",)
    );
}

#[test]
fn render_only_col_spaned() {
    let mut grid = util::new_grid::<2, 3>();

    grid.set(Entity::Cell(0, 0), Settings::new().span_vertical(2));
    grid.set(Entity::Cell(0, 1), Settings::new().span_vertical(2));
    grid.set(Entity::Cell(0, 2), Settings::new().span_vertical(2));

    println!("{}", grid);

    assert_eq!(
        grid.to_string(),
        concat!("+---+---+---+\n", "|0-0|0-1|0-2|\n", "+---+---+---+\n",),
    );
}

#[test]
fn grid_2x2_col_span_test() {
    let mut grid = util::new_grid::<2, 2>();
    grid.set(Entity::Global, Settings::new().text("asd"));
    grid.set(
        Entity::Cell(0, 0),
        Settings::new().text("1\n\n\n\n\n\n\n23").span_vertical(2),
    );

    println!("{}", grid);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+--+---+\n",
            "|1 |asd|\n",
            "|  |   |\n",
            "|  |   |\n",
            "|  |   |\n",
            "+  +---+\n",
            "|  |asd|\n",
            "|  |   |\n",
            "|23|   |\n",
            "+--+---+\n",
        )
    )
}

#[test]
fn grid_2x2_col_span_2_test() {
    let mut grid = util::new_grid::<2, 2>();
    grid.set(Entity::Global, Settings::new().text("asd"));
    grid.set(
        Entity::Cell(0, 0),
        Settings::new().text("12\n3\n4").span_vertical(2),
    );
    grid.set(
        Entity::Cell(0, 1),
        Settings::new().text("a\ns\ndw").span_vertical(2),
    );

    assert_eq!(
        grid.to_string(),
        "+--+--+\n\
         |12|a |\n\
         |3 |s |\n\
         +--+--+\n"
    );

    grid.set(Entity::Cell(0, 0), Settings::new().text("1"));
    grid.set(Entity::Cell(0, 1), Settings::new().text("a"));

    let str = grid.to_string();
    assert_eq!(
        str,
        "+-+-+\n\
         |1|a|\n\
         +-+-+\n"
    );
}

#[test]
fn render_col_span_with_no_split_style() {
    let mut grid = util::new_grid::<2, 2>();
    grid.set_borders(Borders::default());

    grid.set(
        Entity::Cell(0, 0),
        Settings::new()
            .span_vertical(2)
            .vertical_alignment(AlignmentVertical::Center),
    );
    grid.set(Entity::Cell(0, 0), Settings::new().text("1\n2\n3\n"));

    println!("{}", grid);

    assert_eq!(grid.to_string(), concat!("10-1\n", "2   \n", "31-1\n",));
}

#[test]
fn render_zero_col_span_between_cells() {
    let mut grid = util::new_grid::<3, 2>();

    grid.set(Entity::Cell(1, 0), Settings::new().span_vertical(0));

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+\n",
            "|0-0|0-1|\n",
            "+   +---+\n",
            "|   |1-1|\n",
            "+---+---+\n",
            "|2-0|2-1|\n",
            "+---+---+\n",
        )
    );

    grid.set(Entity::Cell(1, 1), Settings::new().span_vertical(0));

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+\n",
            "|0-0|0-1|\n",
            "+---+---+\n",
            "|2-0|2-1|\n",
            "+---+---+\n",
        )
    );
}

#[test]
fn render_zero_col_span_at_the_end() {
    let mut grid = util::new_grid::<3, 2>();

    grid.set(Entity::Cell(1, 1), Settings::new().span_vertical(0));
    grid.set(Entity::Cell(2, 1), Settings::new().span_vertical(0));

    println!("{}", grid);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+\n",
            "|0-0|0-1|\n",
            "+---+   +\n",
            "|1-0|   |\n",
            "+---+   +\n",
            "|2-0|   |\n",
            "+---+---+\n",
        )
    );

    grid.set(Entity::Cell(1, 0), Settings::new().span_vertical(0));
    grid.set(Entity::Cell(2, 0), Settings::new().span_vertical(0));

    assert_eq!(
        grid.to_string(),
        concat!("+---+---+\n", "|0-0|0-1|\n", "+---+---+\n",)
    );
}

#[test]
fn render_zero_col_span_grid() {
    let mut grid = util::new_grid::<2, 2>();

    grid.set(Entity::Cell(0, 0), Settings::new().span_vertical(0));
    grid.set(Entity::Cell(0, 1), Settings::new().span_vertical(0));
    grid.set(Entity::Cell(1, 0), Settings::new().span_vertical(0));
    grid.set(Entity::Cell(1, 1), Settings::new().span_vertical(0));

    // todo: determine if it's correct behaviour?
    assert_eq!(grid.to_string(), "+---+---+\n|0-0|0-1|\n+---+---+\n");
}

#[test]
fn render_cell_with_row_span_and_col_span() {
    let mut grid = util::new_grid::<4, 4>();
    grid.set(
        Entity::Cell(1, 1),
        Settings::new()
            .span_vertical(2)
            .span(2)
            .vertical_alignment(AlignmentVertical::Center)
            .alignment(AlignmentHorizontal::Center)
            .text("123\n345\n555\n333\n"),
    );

    println!("{}", grid);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+---+---+\n",
            "|0-0|0-1|0-2|0-3|\n",
            "+---+---+---+---+\n",
            "|1-0|  123  |1-3|\n",
            "|   |  345  |   |\n",
            "+---+  555  +---+\n",
            "|2-0|  333  |2-3|\n",
            "+---+---+---+---+\n",
            "|3-0|3-1|3-2|3-3|\n",
            "+---+---+---+---+\n",
        )
    );

    let mut grid = util::new_grid::<4, 4>();
    grid.set(
        Entity::Cell(0, 0),
        Settings::new()
            .span_vertical(2)
            .span(2)
            .vertical_alignment(AlignmentVertical::Center)
            .alignment(AlignmentHorizontal::Center)
            .text("123\n345\n555\n333\n"),
    );

    println!("{}", grid);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+---+---+\n",
            "|  123  |0-2|0-3|\n",
            "|  345  |   |   |\n",
            "+  555  +---+---+\n",
            "|  333  |1-2|1-3|\n",
            "+---+---+---+---+\n",
            "|2-0|2-1|2-2|2-3|\n",
            "+---+---+---+---+\n",
            "|3-0|3-1|3-2|3-3|\n",
            "+---+---+---+---+\n",
        )
    );

    let mut grid = util::new_grid::<4, 4>();
    grid.set(
        Entity::Cell(2, 0),
        Settings::new()
            .span_vertical(2)
            .span(2)
            .vertical_alignment(AlignmentVertical::Center)
            .alignment(AlignmentHorizontal::Center)
            .text("123\n345\n555\n333\n"),
    );

    println!("{}", grid);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+---+---+\n",
            "|0-0|0-1|0-2|0-3|\n",
            "+---+---+---+---+\n",
            "|1-0|1-1|1-2|1-3|\n",
            "+---+---+---+---+\n",
            "|  123  |2-2|2-3|\n",
            "|  345  |   |   |\n",
            "+  555  +---+---+\n",
            "|  333  |3-2|3-3|\n",
            "+---+---+---+---+\n",
        )
    );

    let mut grid = util::new_grid::<4, 4>();
    grid.set(
        Entity::Cell(2, 2),
        Settings::new()
            .span_vertical(2)
            .span(2)
            .vertical_alignment(AlignmentVertical::Center)
            .alignment(AlignmentHorizontal::Center)
            .text("123\n345\n555\n333\n"),
    );

    println!("{}", grid);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+---+---+\n",
            "|0-0|0-1|0-2|0-3|\n",
            "+---+---+---+---+\n",
            "|1-0|1-1|1-2|1-3|\n",
            "+---+---+---+---+\n",
            "|2-0|2-1|  123  |\n",
            "|   |   |  345  |\n",
            "+---+---+  555  +\n",
            "|3-0|3-1|  333  |\n",
            "+---+---+---+---+\n",
        )
    );

    let mut grid = util::new_grid::<4, 4>();
    grid.set(
        Entity::Cell(0, 2),
        Settings::new()
            .span_vertical(2)
            .span(2)
            .vertical_alignment(AlignmentVertical::Center)
            .alignment(AlignmentHorizontal::Center)
            .text("123\n345\n555\n333\n"),
    );

    println!("{}", grid);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+---+---+\n",
            "|0-0|0-1|  123  |\n",
            "|   |   |  345  |\n",
            "+---+---+  555  +\n",
            "|1-0|1-1|  333  |\n",
            "+---+---+---+---+\n",
            "|2-0|2-1|2-2|2-3|\n",
            "+---+---+---+---+\n",
            "|3-0|3-1|3-2|3-3|\n",
            "+---+---+---+---+\n",
        )
    );

    let mut grid = util::new_grid::<4, 4>();
    grid.set(
        Entity::Cell(0, 1),
        Settings::new()
            .span_vertical(2)
            .span(2)
            .vertical_alignment(AlignmentVertical::Center)
            .alignment(AlignmentHorizontal::Center)
            .text("123\n345\n555\n333\n"),
    );

    println!("{}", grid);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+---+---+\n",
            "|0-0|  123  |0-3|\n",
            "|   |  345  |   |\n",
            "+---+  555  +---+\n",
            "|1-0|  333  |1-3|\n",
            "+---+---+---+---+\n",
            "|2-0|2-1|2-2|2-3|\n",
            "+---+---+---+---+\n",
            "|3-0|3-1|3-2|3-3|\n",
            "+---+---+---+---+\n",
        )
    );
}

#[test]
fn render_cell_with_row_span_and_col_span_2() {
    let mut grid = util::new_grid::<4, 4>();
    grid.set(
        Entity::Cell(1, 1),
        Settings::new()
            .span_vertical(3)
            .span(3)
            .vertical_alignment(AlignmentVertical::Center)
            .alignment(AlignmentHorizontal::Center)
            .text("123\n345\n555\n333\n"),
    );

    println!("{}", grid);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+---+---+\n",
            "|0-0|0-1|0-2|0-3|\n",
            "+---+---+---+---+\n",
            "|1-0|    123    |\n",
            "+---+    345    +\n",
            "|2-0|    555    |\n",
            "+---+    333    +\n",
            "|3-0|           |\n",
            "+---+---+---+---+\n",
        )
    );

    let mut grid = util::new_grid::<4, 4>();
    grid.set(
        Entity::Cell(0, 0),
        Settings::new()
            .span_vertical(3)
            .span(3)
            .vertical_alignment(AlignmentVertical::Center)
            .alignment(AlignmentHorizontal::Center)
            .text("123\n345\n555\n333\n"),
    );

    println!("{}", grid);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+---+---+\n",
            "|    123    |0-3|\n",
            "+    345    +---+\n",
            "|    555    |1-3|\n",
            "+    333    +---+\n",
            "|           |2-3|\n",
            "+---+---+---+---+\n",
            "|3-0|3-1|3-2|3-3|\n",
            "+---+---+---+---+\n",
        )
    );

    let mut grid = util::new_grid::<4, 4>();
    grid.set(
        Entity::Cell(0, 1),
        Settings::new()
            .span_vertical(3)
            .span(3)
            .vertical_alignment(AlignmentVertical::Center)
            .alignment(AlignmentHorizontal::Center)
            .text("123\n345\n555\n333\n"),
    );

    println!("{}", grid);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+---+---+\n",
            "|0-0|    123    |\n",
            "+---+    345    +\n",
            "|1-0|    555    |\n",
            "+---+    333    +\n",
            "|2-0|           |\n",
            "+---+---+---+---+\n",
            "|3-0|3-1|3-2|3-3|\n",
            "+---+---+---+---+\n",
        )
    );

    let mut grid = util::new_grid::<4, 4>();
    grid.set(
        Entity::Cell(1, 0),
        Settings::new()
            .span_vertical(3)
            .span(3)
            .vertical_alignment(AlignmentVertical::Center)
            .alignment(AlignmentHorizontal::Center)
            .text("123\n345\n555\n333\n"),
    );

    println!("{}", grid);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+---+---+\n",
            "|0-0|0-1|0-2|0-3|\n",
            "+---+---+---+---+\n",
            "|    123    |1-3|\n",
            "+    345    +---+\n",
            "|    555    |2-3|\n",
            "+    333    +---+\n",
            "|           |3-3|\n",
            "+---+---+---+---+\n",
        )
    );
}

#[test]
fn render_grid_with_row_span_and_col_span() {
    let mut grid = util::new_grid::<4, 4>();

    grid.set(Entity::Cell(1, 1), Settings::new().span(2));
    grid.set(Entity::Cell(3, 0), Settings::new().span(3));
    grid.set(Entity::Cell(0, 0), Settings::new().span_vertical(2));
    grid.set(Entity::Cell(0, 3), Settings::new().span_vertical(3));

    grid.set(
        Entity::Cell(0, 0),
        Settings::new().text("hello\nworld\n!\n!\n!\n!\n"),
    );

    println!("{}", grid);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+-----+---+---+---+\n",
            "|hello|0-1|0-2|0-3|\n",
            "|world|   |   |   |\n",
            "|!    |   |   |   |\n",
            "+!    +---+---+   +\n",
            "|!    |1-1    |   |\n",
            "|!    |       |   |\n",
            "+-----+---+---+   +\n",
            "|2-0  |2-1|2-2|   |\n",
            "+-----+---+---+---+\n",
            "|3-0          |3-3|\n",
            "+-----+---+---+---+\n",
        )
    );
}

#[test]
fn check_correct_print_of_col_span() {
    let mut grid = util::new_grid::<5, 2>();

    grid.set(
        Entity::Cell(1, 1),
        Settings::new().text("1\n2\n3\n4").span_vertical(4),
    );

    println!("{}", grid);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+\n",
            "|0-0|0-1|\n",
            "+---+---+\n",
            "|1-0|1  |\n",
            "+---+2  +\n",
            "|2-0|3  |\n",
            "+---+4  +\n",
            "|3-0|   |\n",
            "+---+   +\n",
            "|4-0|   |\n",
            "+---+---+\n",
        )
    );
}

#[test]
fn render_grid_with_row_span_and_col_span_2() {
    let mut grid = util::new_grid::<4, 4>();

    grid.set(Entity::Cell(0, 0), Settings::new().span(2));
    grid.set(
        Entity::Cell(0, 2),
        Settings::new().span_vertical(2).text("q\nw\ne\nr\nt\n"),
    );
    grid.set(
        Entity::Cell(0, 3),
        Settings::new()
            .span_vertical(3)
            .text("q1\nw1\ne1\nr1\nt1\n"),
    );

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+---+---+\n",
            "|0-0    |q  |q1 |\n",
            "|       |w  |w1 |\n",
            "+---+---+e  +e1 +\n",
            "|1-0|1-1|r  |r1 |\n",
            "|   |   |t  |t1 |\n",
            "+---+---+---+   +\n",
            "|2-0|2-1|2-2|   |\n",
            "+---+---+---+---+\n",
            "|3-0|3-1|3-2|3-3|\n",
            "+---+---+---+---+\n",
        )
    );
}

#[test]
fn render_grid_with_row_span_and_col_span_3() {
    let mut grid = util::new_grid::<3, 5>();

    grid.set(Entity::Cell(0, 0), Settings::new().span(2));
    grid.set(
        Entity::Cell(0, 3),
        Settings::new().span_vertical(2).text("q\nw\ne\nr\nt\n"),
    );
    grid.set(
        Entity::Cell(0, 4),
        Settings::new()
            .span_vertical(3)
            .text("q1\nw1\ne1\nr1\nt1\n"),
    );

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+---+---+--+\n",
            "|0-0    |0-2|q  |q1|\n",
            "|       |   |w  |w1|\n",
            "+---+---+---+e  +e1+\n",
            "|1-0|1-1|1-2|r  |r1|\n",
            "|   |   |   |t  |t1|\n",
            "+---+---+---+---+  +\n",
            "|2-0|2-1|2-2|2-3|  |\n",
            "+---+---+---+---+--+\n",
        )
    );
}

#[test]
#[ignore = "todo; create some logic of combining spans? or somehow resolving to not get the following"]
fn render_grid_with_row_3() {
    let mut grid = util::new_grid::<3, 5>();

    grid.set(Entity::Cell(0, 0), Settings::new().span(2));
    grid.set(Entity::Cell(0, 1), Settings::new().span(2));
    grid.set(Entity::Cell(0, 2), Settings::new().span(2));

    println!("{}", grid);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+---+---+--+\n",
            "|0-0    |0-2|q  |q1|\n",
            "|       |   |w  |w1|\n",
            "+---+---+---+e  +e1+\n",
            "|1-0|1-1|1-2|r  |r1|\n",
            "|   |   |   |t  |t1|\n",
            "+---+---+---+---+  +\n",
            "|2-0|2-1|2-2|2-3|  |\n",
            "+---+---+---+---+--+\n",
        )
    );
}

#[test]
fn render_grid_with_row_4() {
    let mut grid = util::new_grid::<3, 4>();

    grid.set(Entity::Cell(0, 0), Settings::new().span(2));
    grid.set(Entity::Cell(1, 0), Settings::new().span(2));
    grid.set(Entity::Cell(2, 0), Settings::new().span(2));

    println!("{}", grid);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+-+-+---+---+\n",
            "|0-0|0-2|0-3|\n",
            "+-+-+---+---+\n",
            "|1-0|1-2|1-3|\n",
            "+-+-+---+---+\n",
            "|2-0|2-2|2-3|\n",
            "+-+-+---+---+\n",
        )
    );

    let mut grid = util::new_grid::<3, 4>();

    grid.set(Entity::Cell(0, 0), Settings::new().span(3));
    grid.set(Entity::Cell(1, 0), Settings::new().span(3));
    grid.set(Entity::Cell(2, 0), Settings::new().span(3));

    println!("{}", grid);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+-+++---+\n",
            "|0-0|0-3|\n",
            "+-+++---+\n",
            "|1-0|1-3|\n",
            "+-+++---+\n",
            "|2-0|2-3|\n",
            "+-+++---+\n",
        )
    );

    let mut grid = util::new_grid::<3, 4>();

    grid.set(Entity::Cell(0, 0), Settings::new().span(3).text(""));
    grid.set(Entity::Cell(1, 0), Settings::new().span(3).text(""));
    grid.set(Entity::Cell(2, 0), Settings::new().span(3).text(""));

    println!("{}", grid);

    assert_eq!(
        grid.to_string(),
        concat!(
            "++++---+\n",
            "|  |0-3|\n",
            "++++---+\n",
            "|  |1-3|\n",
            "++++---+\n",
            "|  |2-3|\n",
            "++++---+\n",
        )
    );
}

#[test]
fn render_grid_with_row_and_col_4() {
    let mut grid = util::new_grid::<3, 4>();

    grid.set(Entity::Cell(0, 0), Settings::new().span(2));
    grid.set(
        Entity::Cell(1, 0),
        Settings::new().span(2).span_vertical(2),
    );

    println!("{}", grid);

    // is this correct?

    assert_eq!(
        grid.to_string(),
        concat!(
            "+-+-+---+---+\n",
            "|0-0|0-2|0-3|\n",
            "+-+-+---+---+\n",
            "|1-0|1-2|1-3|\n",
            "+   +---+---+\n",
            "|   |2-2|2-3|\n",
            "+-+-+---+---+\n",
        )
    );
}
