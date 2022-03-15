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

use papergrid::{
    AlignmentHorizontal, AlignmentVertical, Entity, Grid, Indent, Settings, DEFAULT_CELL_STYLE,
    DEFAULT_INDENT_FILL_CHAR,
};

#[test]
fn render_2x2() {
    let mut grid = Grid::new(2, 2);
    grid.set_cell_borders(DEFAULT_CELL_STYLE.clone());

    grid.set(&Entity::Cell(0, 0), Settings::new().text("0-0"));
    grid.set(&Entity::Cell(0, 1), Settings::new().text("0-1"));
    grid.set(&Entity::Cell(1, 0), Settings::new().text("1-0"));
    grid.set(&Entity::Cell(1, 1), Settings::new().text("1-1"));

    let expected = concat!(
        "+---+---+\n",
        "|0-0|0-1|\n",
        "+---+---+\n",
        "|1-0|1-1|\n",
        "+---+---+\n",
    );

    assert_eq!(expected, grid.to_string());
}

#[test]
fn render_1x1() {
    let mut grid = Grid::new(1, 1);
    grid.set_cell_borders(DEFAULT_CELL_STYLE.clone());
    grid.set(&Entity::Cell(0, 0), Settings::new().text("one line"));

    let expected = concat!("+--------+\n", "|one line|\n", "+--------+\n",);

    assert_eq!(expected, grid.to_string());
}

#[test]
fn render_not_quadratic() {
    let mut grid = Grid::new(1, 2);
    grid.set_cell_borders(DEFAULT_CELL_STYLE.clone());

    grid.set(&Entity::Cell(0, 0), Settings::new().text("hello"));
    grid.set(&Entity::Cell(0, 1), Settings::new().text("world"));

    let expected = concat!("+-----+-----+\n", "|hello|world|\n", "+-----+-----+\n",);

    assert_eq!(expected, grid.to_string());
}

#[test]
fn render_empty() {
    let grid = Grid::new(0, 0);

    let expected = "";

    assert_eq!(expected, grid.to_string());
}

#[test]
fn render_multilane() {
    let mut grid = Grid::new(2, 2);
    grid.set_cell_borders(DEFAULT_CELL_STYLE.clone());

    grid.set(&Entity::Cell(0, 0), Settings::new().text("left\ncell"));
    grid.set(&Entity::Cell(0, 1), Settings::new().text("right one"));
    grid.set(
        &Entity::Cell(1, 0),
        Settings::new().text("the second column got the beginning here"),
    );
    grid.set(
        &Entity::Cell(1, 1),
        Settings::new().text("and here\nwe\nsee\na\nlong\nstring"),
    );

    let expected = concat!(
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
    );

    let g = grid.to_string();
    assert_eq!(expected, g);
}

#[test]
fn render_multilane_alignment() {
    let mut grid = Grid::new(2, 2);
    grid.set_cell_borders(DEFAULT_CELL_STYLE.clone());

    grid.set(
        &Entity::Cell(0, 0),
        Settings::new()
            .text("left\ncell")
            .alignment(AlignmentHorizontal::Center),
    );
    grid.set(&Entity::Cell(0, 1), Settings::new().text("right one"));
    grid.set(
        &Entity::Cell(1, 0),
        Settings::new().text("the second column got the beginning here"),
    );
    grid.set(
        &Entity::Cell(1, 1),
        Settings::new()
            .text("and here\nwe\nsee\na\nlong\nstring")
            .alignment(AlignmentHorizontal::Right),
    );

    let expected = concat!(
        "+----------------------------------------+---------+\n\
         |                  left                  |right one|\n\
         |                  cell                  |         |\n\
         +----------------------------------------+---------+\n\
         |the second column got the beginning here| and here|\n\
         |                                        |       we|\n\
         |                                        |      see|\n\
         |                                        |        a|\n\
         |                                        |     long|\n\
         |                                        |   string|\n\
         +----------------------------------------+---------+\n"
    );

    let g = grid.to_string();
    assert_eq!(expected, g);
}

#[test]
fn render_multilane_vertical_alignment() {
    let mut grid = Grid::new(2, 2);
    grid.set_cell_borders(DEFAULT_CELL_STYLE.clone());

    grid.set(
        &Entity::Cell(0, 0),
        Settings::new()
            .text("left\ncell")
            .alignment(AlignmentHorizontal::Center),
    );
    grid.set(&Entity::Cell(0, 1), Settings::new().text("right one"));
    grid.set(
        &Entity::Cell(1, 0),
        Settings::new()
            .text("the second column got the beginning here")
            .vertical_alignment(AlignmentVertical::Center),
    );
    grid.set(
        &Entity::Cell(1, 1),
        Settings::new()
            .text("and here\nwe\nsee\na\nlong\nstring")
            .alignment(AlignmentHorizontal::Right),
    );

    let expected = concat!(
        "+----------------------------------------+---------+\n\
         |                  left                  |right one|\n\
         |                  cell                  |         |\n\
         +----------------------------------------+---------+\n\
         |                                        | and here|\n\
         |                                        |       we|\n\
         |the second column got the beginning here|      see|\n\
         |                                        |        a|\n\
         |                                        |     long|\n\
         |                                        |   string|\n\
         +----------------------------------------+---------+\n"
    );

    let g = grid.to_string();
    assert_eq!(expected, g);
}

#[test]
fn render_empty_cell() {
    let mut grid = Grid::new(2, 2);
    grid.set_cell_borders(DEFAULT_CELL_STYLE.clone());

    grid.set(&Entity::Cell(0, 0), Settings::new().text("0-0"));
    grid.set(&Entity::Cell(0, 1), Settings::new().text(""));
    grid.set(&Entity::Cell(1, 0), Settings::new().text("1-0"));
    grid.set(&Entity::Cell(1, 1), Settings::new().text("1-1"));

    let expected = concat!(
        "+---+---+\n",
        "|0-0|   |\n",
        "+---+---+\n",
        "|1-0|1-1|\n",
        "+---+---+\n",
    );

    assert_eq!(expected, grid.to_string());
}

#[test]
fn render_row_span() {
    let mut grid = Grid::new(2, 2);
    grid.set_cell_borders(DEFAULT_CELL_STYLE.clone());

    grid.set(
        &Entity::Cell(0, 0),
        Settings::new()
            .text("0-0")
            .span(2)
            .alignment(AlignmentHorizontal::Center),
    );
    grid.set(&Entity::Cell(0, 1), Settings::new().text(""));
    grid.set(&Entity::Cell(1, 0), Settings::new().text("1-0"));
    grid.set(&Entity::Cell(1, 1), Settings::new().text("1-1"));

    let expected = concat!(
        "+---+---+\n",
        "|  0-0  |\n",
        "+---+---+\n",
        "|1-0|1-1|\n",
        "+---+---+\n"
    );

    assert_eq!(expected, grid.to_string());
}

#[test]
fn render_miltiline_span() {
    let mut grid = Grid::new(2, 2);
    grid.set_cell_borders(DEFAULT_CELL_STYLE.clone());

    grid.set(
        &Entity::Cell(0, 0),
        Settings::new()
            .text("0-0\n0-1")
            .span(2)
            .alignment(AlignmentHorizontal::Center),
    );
    grid.set(&Entity::Cell(0, 1), Settings::new().text(""));
    grid.set(&Entity::Cell(1, 0), Settings::new().text("1-0"));
    grid.set(&Entity::Cell(1, 1), Settings::new().text("1-1"));

    let expected = concat!(
        "+---+---+\n",
        "|  0-0  |\n",
        "|  0-1  |\n",
        "+---+---+\n",
        "|1-0|1-1|\n",
        "+---+---+\n"
    );

    assert_eq!(expected, grid.to_string());
}

#[test]
fn render_row_span_multilane() {
    let mut grid = Grid::new(4, 3);
    grid.set_cell_borders(DEFAULT_CELL_STYLE.clone());

    grid.set(
        &Entity::Cell(0, 0),
        Settings::new().text("first line").span(2),
    );
    grid.set(&Entity::Cell(0, 2), Settings::new().text("e.g."));
    grid.set(&Entity::Cell(1, 0), Settings::new().text("0"));
    grid.set(&Entity::Cell(1, 1), Settings::new().text("1"));
    grid.set(&Entity::Cell(1, 2), Settings::new().text("2"));
    grid.set(&Entity::Cell(2, 0), Settings::new().text("0"));
    grid.set(&Entity::Cell(2, 1), Settings::new().text("1"));
    grid.set(&Entity::Cell(2, 2), Settings::new().text("2"));
    grid.set(
        &Entity::Cell(3, 0),
        Settings::new().text("full last line").span(3),
    );

    let expected = concat!(
        "+-----+----+----+\n",
        "|first line|e.g.|\n",
        "+-----+----+----+\n",
        "|0    |1   |2   |\n",
        "+-----+----+----+\n",
        "|0    |1   |2   |\n",
        "+-----+----+----+\n",
        "|full last line |\n",
        "+-----+----+----+\n",
    );

    println!("{}", grid);

    assert_eq!(expected, grid.to_string());
}

#[test]
fn render_row_span_with_horizontal_ident() {
    let mut grid = Grid::new(3, 2);
    grid.set_cell_borders(DEFAULT_CELL_STYLE.clone());

    grid.set(&Entity::Cell(0, 0), Settings::new().text("0-0").span(2));
    grid.set(
        &Entity::Cell(1, 0),
        Settings::new().text("1-0").padding(
            Indent::new(4, DEFAULT_INDENT_FILL_CHAR),
            Indent::new(4, DEFAULT_INDENT_FILL_CHAR),
            Indent::default(),
            Indent::default(),
        ),
    );
    grid.set(&Entity::Cell(1, 1), Settings::new().text("1-1"));
    grid.set(&Entity::Cell(2, 0), Settings::new().text("2-0"));
    grid.set(&Entity::Cell(2, 1), Settings::new().text("2-1"));

    let grid = grid.to_string();

    let expected = concat!(
        "+-----------+---+\n",
        "|0-0            |\n",
        "+-----------+---+\n",
        "|    1-0    |1-1|\n",
        "+-----------+---+\n",
        "|2-0        |2-1|\n",
        "+-----------+---+\n",
    );

    println!("{}", grid);

    assert_eq!(grid, expected);
}

#[test]
fn render_row_span_3x3_with_horizontal_ident() {
    let mut grid = Grid::new(3, 3);
    grid.set_cell_borders(DEFAULT_CELL_STYLE.clone());

    grid.set(&Entity::Cell(0, 0), Settings::new().text("0-0").span(3));
    grid.set(&Entity::Cell(1, 0), Settings::new().text("1-0").span(2));
    grid.set(&Entity::Cell(2, 0), Settings::new().text("2-0").span(2));
    grid.set(&Entity::Cell(1, 2), Settings::new().text("1-1"));
    grid.set(&Entity::Cell(2, 2), Settings::new().text("2-1"));

    let grid = grid.to_string();

    let expected = concat!(
        "+-+-+---+\n",
        "|0-0    |\n",
        "+-+-+---+\n",
        "|1-0|1-1|\n",
        "+-+-+---+\n",
        "|2-0|2-1|\n",
        "+-+-+---+\n",
    );

    println!("{}", grid);

    assert_eq!(grid, expected);
}

#[test]
fn render_2_colided_row_span_3x3() {
    let mut grid = Grid::new(3, 3);
    grid.set_cell_borders(DEFAULT_CELL_STYLE.clone());

    grid.set(
        &Entity::Cell(0, 0),
        Settings::new().text("0-0xxxxxxx").span(2),
    );
    grid.set(&Entity::Cell(0, 2), Settings::new().text("0-2"));
    grid.set(&Entity::Cell(1, 0), Settings::new().text("1-0"));
    grid.set(&Entity::Cell(1, 1), Settings::new().text("1-1").span(2));
    grid.set(&Entity::Cell(2, 0), Settings::new().text("2-0"));
    grid.set(&Entity::Cell(2, 1), Settings::new().text("2-1"));
    grid.set(&Entity::Cell(2, 2), Settings::new().text("2-2"));

    let grid = grid.to_string();

    let expected = concat!(
        "+-----+----+---+\n",
        "|0-0xxxxxxx|0-2|\n",
        "+-----+----+---+\n",
        "|1-0  |1-1     |\n",
        "+-----+----+---+\n",
        "|2-0  |2-1 |2-2|\n",
        "+-----+----+---+\n",
    );

    assert_eq!(grid, expected);

    let mut grid = Grid::new(3, 3);
    grid.set_cell_borders(DEFAULT_CELL_STYLE.clone());

    grid.set(&Entity::Cell(0, 0), Settings::new().text("0-0").span(2));
    grid.set(&Entity::Cell(0, 2), Settings::new().text("0-2"));
    grid.set(&Entity::Cell(1, 0), Settings::new().text("1-0"));
    grid.set(
        &Entity::Cell(1, 1),
        Settings::new().text("1-1xxxxxxx").span(2),
    );
    grid.set(&Entity::Cell(2, 0), Settings::new().text("2-0"));
    grid.set(&Entity::Cell(2, 1), Settings::new().text("2-1"));
    grid.set(&Entity::Cell(2, 2), Settings::new().text("2-2"));

    let grid = grid.to_string();

    let expected = concat!(
        "+---+-----+----+\n",
        "|0-0      |0-2 |\n",
        "+---+-----+----+\n",
        "|1-0|1-1xxxxxxx|\n",
        "+---+-----+----+\n",
        "|2-0|2-1  |2-2 |\n",
        "+---+-----+----+\n",
    );

    assert_eq!(grid, expected);

    let mut grid = Grid::new(3, 3);
    grid.set_cell_borders(DEFAULT_CELL_STYLE.clone());

    grid.set(&Entity::Cell(0, 0), Settings::new().text("0-0").span(2));
    grid.set(&Entity::Cell(0, 2), Settings::new().text("0-2"));
    grid.set(&Entity::Cell(1, 0), Settings::new().text("1-0"));
    grid.set(
        &Entity::Cell(1, 1),
        Settings::new().text("1-1xxxxxxx").span(2),
    );
    grid.set(
        &Entity::Cell(2, 0),
        Settings::new().text("2-0xxxxxxxxxxxxx"),
    );
    grid.set(&Entity::Cell(2, 1), Settings::new().text("2-1"));
    grid.set(&Entity::Cell(2, 2), Settings::new().text("2-2"));

    let grid = grid.to_string();

    let expected = concat!(
        "+----------------+-----+----+\n",
        "|0-0                   |0-2 |\n",
        "+----------------+-----+----+\n",
        "|1-0             |1-1xxxxxxx|\n",
        "+----------------+-----+----+\n",
        "|2-0xxxxxxxxxxxxx|2-1  |2-2 |\n",
        "+----------------+-----+----+\n",
    );

    assert_eq!(grid, expected);

    let mut grid = Grid::new(3, 3);
    grid.set_cell_borders(DEFAULT_CELL_STYLE.clone());

    grid.set(&Entity::Cell(0, 0), Settings::new().text("0-0").span(2));
    grid.set(&Entity::Cell(0, 2), Settings::new().text("0-2"));
    grid.set(&Entity::Cell(1, 0), Settings::new().text("1-0"));
    grid.set(&Entity::Cell(1, 1), Settings::new().text("1-1").span(2));
    grid.set(&Entity::Cell(2, 0), Settings::new().text("2-0"));
    grid.set(
        &Entity::Cell(2, 1),
        Settings::new().text("2-1xxxxxxxxxxxxx"),
    );
    grid.set(&Entity::Cell(2, 2), Settings::new().text("2-2"));

    let grid = grid.to_string();

    let expected = concat!(
        "+---+----------------+---+\n",
        "|0-0                 |0-2|\n",
        "+---+----------------+---+\n",
        "|1-0|1-1                 |\n",
        "+---+----------------+---+\n",
        "|2-0|2-1xxxxxxxxxxxxx|2-2|\n",
        "+---+----------------+---+\n",
    );

    assert_eq!(grid, expected);

    let mut grid = Grid::new(3, 3);
    grid.set_cell_borders(DEFAULT_CELL_STYLE.clone());

    grid.set(&Entity::Cell(0, 0), Settings::new().text("0-0").span(2));
    grid.set(
        &Entity::Cell(0, 2),
        Settings::new().text("0-2xxxxxxxxxxxxx"),
    );
    grid.set(&Entity::Cell(1, 0), Settings::new().text("1-0"));
    grid.set(&Entity::Cell(1, 1), Settings::new().text("1-1").span(2));
    grid.set(&Entity::Cell(2, 0), Settings::new().text("2-0"));
    grid.set(&Entity::Cell(2, 1), Settings::new().text("2-1"));
    grid.set(&Entity::Cell(2, 2), Settings::new().text("2-2"));

    let grid = grid.to_string();

    let expected = concat!(
        "+---+---+----------------+\n",
        "|0-0    |0-2xxxxxxxxxxxxx|\n",
        "+---+---+----------------+\n",
        "|1-0|1-1                 |\n",
        "+---+---+----------------+\n",
        "|2-0|2-1|2-2             |\n",
        "+---+---+----------------+\n",
    );

    assert_eq!(grid, expected);
}

#[test]
fn render_spaned_column_in_first_cell_3x3() {
    let mut grid = Grid::new(3, 3);
    grid.set_cell_borders(DEFAULT_CELL_STYLE.clone());

    grid.set(
        &Entity::Cell(0, 0),
        Settings::new().text("0-0xxxxxxx").span(2),
    );
    grid.set(&Entity::Cell(0, 1), Settings::new().text("0-1"));
    grid.set(&Entity::Cell(0, 2), Settings::new().text("0-2"));
    grid.set(&Entity::Cell(1, 0), Settings::new().text("1-0"));
    grid.set(&Entity::Cell(1, 1), Settings::new().text("1-1"));
    grid.set(&Entity::Cell(1, 2), Settings::new().text("1-2"));
    grid.set(&Entity::Cell(2, 0), Settings::new().text("2-0"));
    grid.set(&Entity::Cell(2, 1), Settings::new().text("2-1"));
    grid.set(&Entity::Cell(2, 2), Settings::new().text("2-2"));

    let grid = grid.to_string();

    let expected = concat!(
        "+-----+----+---+\n",
        "|0-0xxxxxxx|0-2|\n",
        "+-----+----+---+\n",
        "|1-0  |1-1 |1-2|\n",
        "+-----+----+---+\n",
        "|2-0  |2-1 |2-2|\n",
        "+-----+----+---+\n",
    );

    assert_eq!(grid, expected);
}

#[test]
fn render_row_span_with_different_length() {
    let mut grid = Grid::new(3, 2);
    grid.set_cell_borders(DEFAULT_CELL_STYLE.clone());

    grid.set(
        &Entity::Cell(0, 0),
        Settings::new().text("first row").span(2),
    );
    grid.set(&Entity::Cell(1, 0), Settings::new().text("0"));
    grid.set(&Entity::Cell(1, 1), Settings::new().text("1"));
    grid.set(
        &Entity::Cell(2, 0),
        Settings::new().text("a longer second row").span(2),
    );

    let expected = concat!(
        "+---------+---------+\n",
        "|first row          |\n",
        "+---------+---------+\n",
        "|0        |1        |\n",
        "+---------+---------+\n",
        "|a longer second row|\n",
        "+---------+---------+\n",
    );

    println!("{}", grid);

    assert_eq!(expected, grid.to_string());
}

#[test]
fn render_row_span_with_odd_length() {
    let mut grid = Grid::new(2, 2);
    grid.set_cell_borders(DEFAULT_CELL_STYLE.clone());

    grid.set(&Entity::Cell(0, 0), Settings::new().text("3   ").span(2));
    grid.set(&Entity::Cell(1, 0), Settings::new().text("2"));
    grid.set(&Entity::Cell(1, 1), Settings::new().text("4"));

    let expected = concat!("+--+-+\n", "|3   |\n", "+--+-+\n", "|2 |4|\n", "+--+-+\n",);

    assert_eq!(expected, grid.to_string());
}

#[test]
fn render_only_row_spaned() {
    let mut grid = Grid::new(3, 2);
    grid.set_cell_borders(DEFAULT_CELL_STYLE.clone());

    grid.set(&Entity::Cell(0, 0), Settings::new().text("0-0").span(2));
    grid.set(&Entity::Cell(1, 0), Settings::new().text("1-0").span(2));
    grid.set(&Entity::Cell(2, 0), Settings::new().text("2-0").span(2));

    let expected = "+-+-+\n\
                         |0-0|\n\
                         +-+-+\n\
                         |1-0|\n\
                         +-+-+\n\
                         |2-0|\n\
                         +-+-+\n";

    assert_eq!(grid.to_string(), expected);
}

#[test]
fn grid_3x2_test() {
    let mut grid = Grid::new(3, 2);
    grid.set(&Entity::Global, Settings::new().text("asd"));
    grid.set_cell_borders(DEFAULT_CELL_STYLE.clone());

    let str = grid.to_string();
    assert_eq!(
        str,
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
fn grid_2x2_span_test() {
    let mut grid = Grid::new(2, 2);
    grid.set(&Entity::Global, Settings::new().text("asd"));
    grid.set_cell_borders(DEFAULT_CELL_STYLE.clone());

    grid.set(&Entity::Cell(0, 0), Settings::new().text("123").span(2));

    let str = grid.to_string();
    assert_eq!(
        str,
        "+---+---+\n\
         |123    |\n\
         +---+---+\n\
         |asd|asd|\n\
         +---+---+\n"
    )
}

#[test]
fn grid_2x2_2_span_test() {
    let mut grid = Grid::new(2, 2);
    grid.set(&Entity::Global, Settings::new().text("asd"));
    grid.set_cell_borders(DEFAULT_CELL_STYLE.clone());

    // grid.set(&Entity::Cell(0, 0), Settings::new().text("123").span(2));
    // grid.set(&Entity::Cell(1, 0), Settings::new().text("asd").span(2));

    // let str = grid.to_string();
    // assert_eq!(
    //     str,
    //     "+-+-+\n\
    //      |123|\n\
    //      +-+-+\n\
    //      |asd|\n\
    //      +-+-+\n"
    // );

    grid.set(&Entity::Cell(0, 0), Settings::new().text("1234").span(2));
    grid.set(&Entity::Cell(1, 0), Settings::new().text("asdw").span(2));

    let str = grid.to_string();
    assert_eq!(
        str,
        "+--+-+\n\
         |1234|\n\
         +--+-+\n\
         |asdw|\n\
         +--+-+\n"
    );

    grid.set(&Entity::Cell(0, 0), Settings::new().text("1").span(2));
    grid.set(&Entity::Cell(1, 0), Settings::new().text("a").span(2));

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
    let mut grid = Grid::new(2, 2);
    grid.set_cell_borders(DEFAULT_CELL_STYLE.clone());

    grid.set(
        &Entity::Cell(0, 0),
        Settings::new()
            .text("0-0")
            .span(2)
            .alignment(AlignmentHorizontal::Center),
    );
    grid.set(&Entity::Cell(0, 1), Settings::new().text(""));
    grid.set(&Entity::Cell(1, 0), Settings::new().text("1-0"));
    grid.set(&Entity::Cell(1, 1), Settings::new().text("1-1"));

    grid.clear_split_grid();

    let expected = concat!(" 0-0  \n", "1-01-1\n");

    assert_eq!(expected, grid.to_string());
}

#[test]
#[ignore = "I am not sure what is the right behaiviour here"]
fn hieroglyph_handling() {
    let mut grid = Grid::new(1, 2);
    grid.set_cell_borders(DEFAULT_CELL_STYLE.clone());

    grid.set(&Entity::Cell(0, 0), Settings::new().text("哈哈"));
    grid.set(&Entity::Cell(0, 1), Settings::new().text("哈"));
    let s = grid.to_string();
    assert_eq!(
        s,
        "+----+--+\n\
         |哈哈  |哈 |\n\
         +----+--+\n"
    )
}

#[test]
#[ignore = "I am not sure what is the right behaiviour here"]
fn hieroglyph_multiline_handling() {
    let mut grid = Grid::new(1, 2);
    grid.set_cell_borders(DEFAULT_CELL_STYLE.clone());

    grid.set(&Entity::Cell(0, 0), Settings::new().text("哈哈"));
    grid.set(&Entity::Cell(0, 1), Settings::new().text("哈\n哈"));
    let s = grid.to_string();
    assert_eq!(
        s,
        "+----+--+\n\
         |哈哈  |哈 |\n\
         |    |哈 |\n\
         +----+--+\n"
    )
}
