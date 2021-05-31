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

use papergrid::{Alignment, Entity, Grid, Settings};

#[test]
fn render() {
    let mut grid = Grid::new(2, 2);
    grid.set(Entity::Cell(0, 0), Settings::new().text("0-0"));
    grid.set(Entity::Cell(0, 1), Settings::new().text("0-1"));
    grid.set(Entity::Cell(1, 0), Settings::new().text("1-0"));
    grid.set(Entity::Cell(1, 1), Settings::new().text("1-1"));

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
fn render_multilane() {
    let mut grid = Grid::new(2, 2);
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
    grid.set(
        Entity::Cell(0, 0),
        Settings::new()
            .text("left\ncell")
            .alignment(Alignment::Center),
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
            .alignment(Alignment::Right),
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
fn render_one_line() {
    let mut grid = Grid::new(1, 1);
    grid.set(Entity::Cell(0, 0), Settings::new().text("one line"));

    let expected = concat!("+--------+\n", "|one line|\n", "+--------+\n",);

    assert_eq!(expected, grid.to_string());
}

#[test]
fn render_not_quadratic() {
    let mut grid = Grid::new(1, 2);
    grid.set(Entity::Cell(0, 0), Settings::new().text("hello"));
    grid.set(Entity::Cell(0, 1), Settings::new().text("world"));

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
fn render_empty_cell() {
    let mut grid = Grid::new(2, 2);
    grid.set(Entity::Cell(0, 0), Settings::new().text("0-0"));
    grid.set(Entity::Cell(0, 1), Settings::new().text(""));
    grid.set(Entity::Cell(1, 0), Settings::new().text("1-0"));
    grid.set(Entity::Cell(1, 1), Settings::new().text("1-1"));

    let expected = concat!(
        "+---+---+\n",
        "|0-0|   |\n",
        "+---+---+\n",
        "|1-0|1-1|\n",
        "+---+---+\n",
    );

    assert_eq!(expected, grid.to_string());
}

// #[test]
// fn render_row_span() {
//     let mut grid = Grid::new(2, 2);
//     grid.cell(0, 0).set_content("0-0").set_row_span(1);
//     grid.cell(1, 0).set_content("1-0");
//     grid.cell(1, 1).set_content("1-1");

//     let expected = concat!(
//         "+-------+\n",
//         "|  0-0  |\n",
//         "+-------+\n",
//         "|1-0|1-1|\n",
//         "+---+---+\n"
//     );

//     assert_eq!(expected, grid.to_string());
// }

// #[test]
// fn render_row_span_multilane() {
//     let mut grid = Grid::new(4, 3);
//     grid.cell(0, 0).set_content("first line").set_row_span(1);
//     grid.cell(0, 2).set_content("e.g.");
//     grid.cell(1, 0).set_content("0");
//     grid.cell(1, 1).set_content("1");
//     grid.cell(1, 2).set_content("2");
//     grid.cell(2, 0).set_content("0");
//     grid.cell(2, 1).set_content("1");
//     grid.cell(2, 2).set_content("2");
//     grid.cell(3, 0)
//         .set_content("full last line")
//         .set_row_span(2);

//     let expected = concat!(
//         "+----------+----+\n",
//         "|first line|e.g.|\n",
//         "+----------+----+\n",
//         "|  0  | 1  | 2  |\n",
//         "+-----+----+----+\n",
//         "|  0  | 1  | 2  |\n",
//         "+-----+----+----+\n",
//         "|full last line |\n",
//         "+---------------+\n",
//     );

//     assert_eq!(expected, grid.to_string());
// }

// #[test]
// fn render_row_span_with_horizontal_ident() {
//     let mut grid = Grid::new(3, 2);
//     grid.cell(0, 0).set_content("0-0").set_row_span(1);
//     grid.cell(1, 0).set_content("1-0").set_horizontal_ident(4);
//     grid.cell(1, 1).set_content("1-1");
//     grid.cell(2, 0).set_content("2-0");
//     grid.cell(2, 1).set_content("2-1");

//     let expected = concat!(
//         "+---------------+\n",
//         "|      0-0      |\n",
//         "+---------------+\n",
//         "|    1-0    |1-1|\n",
//         "+-----------+---+\n",
//         "|    2-0    |2-1|\n",
//         "+-----------+---+\n",
//     );

//     assert_eq!(expected, grid.to_string());
// }

// #[test]
// fn render_row_span_with_odd_length() {
//     let mut grid = Grid::new(2, 2);
//     grid.cell(0, 0).set_content("3   ").set_row_span(1);
//     grid.cell(1, 0).set_content("2");
//     grid.cell(1, 1).set_content("3");

//     let expected = concat!("+----+\n", "|3   |\n", "+----+\n", "|2 |3|\n", "+--+-+\n",);

//     assert_eq!(expected, grid.to_string());
// }

// #[test]
// fn render_only_row_spaned() {
//     let mut grid = Grid::new(3, 2);
//     grid.cell(0, 0).set_content("0-0").set_row_span(1);
//     grid.cell(1, 0).set_content("1-0").set_row_span(1);
//     grid.cell(2, 0).set_content("2-0").set_row_span(1);

//     let expected =
//         concat!("+---+\n", "|0-0|\n", "+---+\n", "|1-0|\n", "+---+\n", "|2-0|\n", "+---+\n",);

//     assert_eq!(expected, grid.to_string());
// }

// #[test]
// fn render_column_span_left() {
//     let mut grid = Grid::new(2, 2);
//     grid.cell(0, 0).set_content("1").set_column_span(1);
//     grid.cell(0, 1).set_content("2");
//     grid.cell(1, 1).set_content("3");

//     let expected = concat!("+-+-+\n", "|1|2|\n", "| |-+\n", "| |3|\n", "+-+-+\n",);

//     assert_eq!(expected, grid.to_string());
// }

// #[test]
// fn render_column_span_right() {
//     let mut grid = Grid::new(2, 2);
//     grid.cell(0, 1).set_content("2").set_column_span(1);
//     grid.cell(0, 0).set_content("1");
//     grid.cell(1, 0).set_content("3");

//     let expected = concat!("+-+-+\n", "|1|2|\n", "+-+ |\n", "|3| |\n", "+-+-+\n",);

//     assert_eq!(expected, grid.to_string());
// }

// #[test]
// fn render_column_span_center() {
//     let mut grid = Grid::new(2, 3);
//     grid.cell(0, 0).set_content("0");
//     grid.cell(0, 1).set_content("1").set_column_span(1);
//     grid.cell(0, 2).set_content("2");
//     grid.cell(1, 0).set_content("3");
//     grid.cell(1, 2).set_content("3");

//     let expected = concat!(
//         "+-+-+-+\n",
//         "|0|1|2|\n",
//         "+-+ |-+\n",
//         "|3| |3|\n",
//         "+-+-+-+\n",
//     );

//     assert_eq!(expected, grid.to_string());
// }

// #[test]
// fn render_column_span_center_under_row() {
//     let mut grid = Grid::new(3, 3);
//     grid.cell(0, 0).set_content("0");
//     grid.cell(0, 1).set_content("1");
//     grid.cell(0, 2).set_content("2");
//     grid.cell(1, 0).set_content("3");
//     grid.cell(1, 1).set_content("4").set_column_span(1);
//     grid.cell(1, 2).set_content("5");
//     grid.cell(2, 0).set_content("6");
//     grid.cell(2, 2).set_content("8");

//     let expected = concat!(
//         "+-+-+-+\n",
//         "|0|1|2|\n",
//         "+-+-+-+\n",
//         "|3|4|5|\n",
//         "+-+ |-+\n",
//         "|6| |8|\n",
//         "+-+-+-+\n",
//     );

//     assert_eq!(expected, grid.to_string());
// }

// #[test]
// fn render_column_span_center_under_row_combined_with_span_row() {
//     let mut grid = Grid::new(3, 3);
//     grid.cell(0, 0).set_content("0").set_row_span(1);
//     grid.cell(0, 2).set_content("2");
//     grid.cell(1, 0).set_content("3");
//     grid.cell(1, 1).set_content("4").set_column_span(1);
//     grid.cell(1, 2).set_content("5");
//     grid.cell(2, 0).set_content("6");
//     grid.cell(2, 2).set_content("8");

//     let expected = concat!(
//         "+---+-+\n",
//         "| 0 |2|\n",
//         "+---+-+\n",
//         "|3|4|5|\n",
//         "+-+ |-+\n",
//         "|6| |8|\n",
//         "+-+-+-+\n",
//     );

//     assert_eq!(expected, grid.to_string());
// }

// #[test]
// fn render_column_span_with_3_columns() {
//     let mut grid = Grid::new(3, 2);
//     grid.cell(0, 0).set_content("0");
//     grid.cell(0, 1).set_content("1").set_column_span(1);
//     grid.cell(1, 0).set_content("2");
//     grid.cell(1, 1).set_content("3");
//     grid.cell(2, 0).set_content("4");
//     grid.cell(2, 1).set_content("5");

//     let expected =
//         concat!("+-+-+\n", "|0|1|\n", "+-+ |\n", "|2| |\n", "+-+-+\n", "|4|5|\n", "+-+-+\n",);

//     assert_eq!(expected, grid.to_string());
// }

// #[test]
// fn render_column_span_filled_column() {
//     let mut grid = Grid::new(2, 2);
//     grid.cell(0, 0).set_content("0");
//     grid.cell(0, 1).set_content("1\n1\n1").set_column_span(1);
//     grid.cell(1, 0).set_content("2");
//     grid.cell(1, 1).set_content("3");

//     let expected = concat!("+-+-+\n", "|0|1|\n", "+-+1|\n", "|2|1|\n", "+-+-+\n",);

//     assert_eq!(expected, grid.to_string());
// }

// #[test]
// fn render_column_span_resize() {
//     let mut grid = Grid::new(2, 2);
//     grid.cell(0, 0).set_content("0");
//     grid.cell(0, 1).set_content("1\n1\n1\n1").set_column_span(1);
//     grid.cell(1, 0).set_content("2");
//     grid.cell(1, 1).set_content("3");

//     let expected = concat!("+-+-+\n", "|0|1|\n", "| |1|\n", "+-+1|\n", "|2|1|\n", "+-+-+\n",);

//     assert_eq!(expected, grid.to_string());
// }

// #[test]
// fn render_column_and_row_span_on_line() {
//     let mut grid = Grid::new(3, 3);
//     grid.cell(0, 0).set_content("0").set_column_span(2);
//     grid.cell(0, 1).set_content("1").set_row_span(1);
//     grid.cell(1, 1).set_content("2");
//     grid.cell(1, 2).set_content("3");
//     grid.cell(2, 1).set_content("4");
//     grid.cell(2, 2).set_content("5");

//     let expected = concat!(
//         "+-+---+\n",
//         "|0| 1 |\n",
//         "| |---+\n",
//         "| |2|3|\n",
//         "| |-+-+\n",
//         "| |4|5|\n",
//         "+-+-+-+\n",
//     );

//     assert_eq!(expected, grid.to_string());
// }

// #[test]
// #[ignore = "
//             This seems to be an issue.

//             It relates to `parent` logic in weight and height
//             calculations. We cannot find anything to the cell with index (1, 1).
//             And as a result it panic.

//             I am not sure how to squeeze this acting in current logic.
//             Which shows that the current architecture should be reframed.
//         "]
// fn render_one_cell_with_column_and_row_span() {
//     let mut grid = Grid::new(2, 2);
//     grid.cell(0, 0)
//         .set_content("field")
//         .set_row_span(1)
//         .set_column_span(1);

//     let expected = concat!("+-----+\n", "|field|\n", "+-----+\n",);

//     assert_eq!(expected, grid.to_string());
// }

// #[test]
// #[ignore = "The behavior should be determined"]
// fn render_empty_line_biside_column_span() {
//     let mut grid = Grid::new(3, 3);
//     grid.cell(0, 0).set_content("0");
//     grid.cell(0, 1).set_content("1");
//     grid.cell(0, 2).set_content("2");
//     grid.cell(1, 0).set_content("1").set_column_span(1);
//     grid.cell(2, 1).set_content("4");
//     grid.cell(2, 2).set_content("5");

//     let expected = concat!(
//         "+-+-+-+\n",
//         "|0|1|2|\n",
//         "+-+-+-+\n",
//         "|1|-+-+\n",
//         "| |4|5|\n",
//         "+-+-+-+\n",
//     );

//     assert_eq!(expected, grid.to_string());
// }

// #[test]
// #[ignore = "
//             This issue is related to horizontal indent when we have
//             a cell which will be widen by largest row.

//             Indeed now all indents which is does not make difference in weight ignores.
//             It means that at the represented example there's no any indents.
//             There's only space which is increased by bigger row.

//             The similar issue must be with vertical_indent and column_span too.

//             And what's the right result?
//         "]
// fn render_row_span_with_indent_when_there_is_bigger_row() {
//     let mut grid = Grid::new(2, 2);
//     grid.cell(0, 0)
//         .set_content("0")
//         .set_row_span(2)
//         .set_horizontal_ident(1);
//     grid.cell(1, 1).set_content("bigger_field");

//     let expected = concat!(
//         "+-------------+\n",
//         "|      0      |\n",
//         "+-------------+\n",
//         "||bigger_field|\n",
//         "++------------+\n",
//     );

//     assert_eq!(expected, grid.to_string());
// }
