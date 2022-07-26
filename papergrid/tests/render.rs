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

use papergrid::{AlignmentHorizontal, AlignmentVertical, Borders, Entity, Grid, Indent, Padding};

mod util;

#[test]
fn render_2x2_test() {
    let grid = util::grid::<2, 2>();

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+\n",
            "|0-0|0-1|\n",
            "+---+---+\n",
            "|1-0|1-1|\n",
            "+---+---+",
        )
    );
}

#[test]
fn render_1x1_test() {
    let grid = util::grid_with_data::<1, 1>(&[((0, 0), "one line")]);

    assert_eq!(
        grid.to_string(),
        concat!("+--------+\n", "|one line|\n", "+--------+")
    );
}

#[test]
fn render_3x2_test() {
    let grid = util::grid_const::<3, 2>("asd");

    assert_eq!(
        grid.to_string(),
        "+---+---+\n\
         |asd|asd|\n\
         +---+---+\n\
         |asd|asd|\n\
         +---+---+\n\
         |asd|asd|\n\
         +---+---+",
    )
}

#[test]
fn render_not_quadratic() {
    let grid = util::grid_with_data::<1, 2>(&[((0, 0), "hello"), ((0, 1), "world")]);

    assert_eq!(
        grid.to_string(),
        concat!("+-----+-----+\n", "|hello|world|\n", "+-----+-----+")
    );
}

#[test]
fn render_empty() {
    let grid = Grid::new(vec![], 0, 0);
    assert_eq!("", grid.to_string());
}

#[test]
fn render_multilane() {
    let grid = util::grid_from([
        ["left\ncell", "right one"],
        [
            "the second column got the beginning here",
            "and here\nwe\nsee\na\nlong\nstring",
        ],
    ]);

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
            "+----------------------------------------+---------+",
        )
    );
}

#[test]
fn render_multilane_alignment() {
    let mut grid = util::grid_from([
        ["left\ncell", "right one"],
        [
            "the second column got the beginning here",
            "and here\nwe\nsee\na\nlong\nstring",
        ],
    ]);

    grid.set_alignment_horizontal(Entity::Cell(0, 0), AlignmentHorizontal::Center);
    grid.set_alignment_horizontal(Entity::Cell(1, 1), AlignmentHorizontal::Right);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+----------------------------------------+---------+\n",
            "|                  left                  |right one|\n",
            "|                  cell                  |         |\n",
            "+----------------------------------------+---------+\n",
            "|the second column got the beginning here| and here|\n",
            "|                                        | we      |\n",
            "|                                        | see     |\n",
            "|                                        | a       |\n",
            "|                                        | long    |\n",
            "|                                        | string  |\n",
            "+----------------------------------------+---------+",
        )
    );
}

#[test]
fn render_multilane_vertical_alignment() {
    let mut grid = util::grid_from([
        ["left\ncell", "right one"],
        [
            "the second column got the beginning here",
            "and here\nwe\nsee\na\nlong\nstring",
        ],
    ]);

    grid.set_alignment_horizontal(Entity::Cell(0, 0), AlignmentHorizontal::Center);
    grid.set_alignment_vertical(Entity::Cell(1, 0), AlignmentVertical::Center);
    grid.set_alignment_horizontal(Entity::Cell(1, 1), AlignmentHorizontal::Right);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+----------------------------------------+---------+\n",
            "|                  left                  |right one|\n",
            "|                  cell                  |         |\n",
            "+----------------------------------------+---------+\n",
            "|                                        | and here|\n",
            "|                                        | we      |\n",
            "|the second column got the beginning here| see     |\n",
            "|                                        | a       |\n",
            "|                                        | long    |\n",
            "|                                        | string  |\n",
            "+----------------------------------------+---------+",
        )
    );
}

#[test]
fn render_empty_cell() {
    let grid = util::grid_with_data::<2, 2>(&[((0, 1), "")]);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+\n",
            "|0-0|   |\n",
            "+---+---+\n",
            "|1-0|1-1|\n",
            "+---+---+",
        )
    );
}

#[test]
fn render_row_span() {
    let mut grid = util::grid::<2, 2>();
    grid.set_span(Entity::Cell(0, 0), 2);
    grid.set_alignment_horizontal(Entity::Cell(0, 0), AlignmentHorizontal::Center);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+\n",
            "|  0-0  |\n",
            "+---+---+\n",
            "|1-0|1-1|\n",
            "+---+---+",
        )
    );
}

#[test]
fn render_miltiline_span() {
    let mut grid = util::grid_with_data::<2, 2>(&[((0, 0), "0-0\n0-1")]);
    grid.set_span(Entity::Cell(0, 0), 2);
    grid.set_alignment_horizontal(Entity::Cell(0, 0), AlignmentHorizontal::Center);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+\n",
            "|  0-0  |\n",
            "|  0-1  |\n",
            "+---+---+\n",
            "|1-0|1-1|\n",
            "+---+---+",
        )
    );
}

#[test]
fn render_row_span_multilane() {
    let mut grid = util::grid_from([
        ["first line", "", "e.g."],
        ["0", "1", "2"],
        ["0", "1", "2"],
        ["full last line", "", ""],
    ]);

    grid.set_span(Entity::Cell(0, 0), 2);
    grid.set_span(Entity::Cell(3, 0), 3);

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
            "+-----+----+----+",
        )
    );
}

#[test]
fn render_row_span_with_horizontal_ident() {
    let mut grid = util::grid::<3, 2>();
    grid.set_span(Entity::Cell(0, 0), 2);
    grid.set_padding(
        Entity::Cell(1, 0),
        Padding::new(
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
            "+-----------+---+",
        )
    );
}

#[test]
fn render_row_span_3x3_with_horizontal_ident() {
    let mut grid = util::grid::<3, 3>();
    grid.set_span(Entity::Cell(0, 0), 3);
    grid.set_span(Entity::Cell(1, 0), 2);
    grid.set_span(Entity::Cell(2, 0), 2);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+-+-+---+\n",
            "|0-0    |\n",
            "+-+-+---+\n",
            "|1-0|1-2|\n",
            "+-+-+---+\n",
            "|2-0|2-2|\n",
            "+-+-+---+",
        )
    );
}

#[test]
fn render_2_colided_row_span_3x3() {
    let mut grid = util::grid_with_data::<3, 3>(&[((0, 0), "0-0xxxxxxx")]);
    grid.set_span(Entity::Cell(0, 0), 2);
    grid.set_span(Entity::Cell(1, 1), 2);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+-----+----+---+\n",
            "|0-0xxxxxxx|0-2|\n",
            "+-----+----+---+\n",
            "|1-0  |1-1     |\n",
            "+-----+----+---+\n",
            "|2-0  |2-1 |2-2|\n",
            "+-----+----+---+",
        )
    );

    let mut grid = util::grid_with_data::<3, 3>(&[((1, 1), "1-1xxxxxxx")]);
    grid.set_span(Entity::Cell(0, 0), 2);
    grid.set_span(Entity::Cell(1, 1), 2);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+-----+----+\n",
            "|0-0      |0-2 |\n",
            "+---+-----+----+\n",
            "|1-0|1-1xxxxxxx|\n",
            "+---+-----+----+\n",
            "|2-0|2-1  |2-2 |\n",
            "+---+-----+----+",
        )
    );

    let mut grid =
        util::grid_with_data::<3, 3>(&[((1, 1), "1-1xxxxxxx"), ((2, 0), "2-0xxxxxxxxxxxxx")]);
    grid.set_span(Entity::Cell(0, 0), 2);
    grid.set_span(Entity::Cell(1, 1), 2);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+----------------+-----+----+\n",
            "|0-0                   |0-2 |\n",
            "+----------------+-----+----+\n",
            "|1-0             |1-1xxxxxxx|\n",
            "+----------------+-----+----+\n",
            "|2-0xxxxxxxxxxxxx|2-1  |2-2 |\n",
            "+----------------+-----+----+",
        )
    );

    let mut grid = util::grid_with_data::<3, 3>(&[((2, 1), "2-1xxxxxxxxxxxxx")]);
    grid.set_span(Entity::Cell(0, 0), 2);
    grid.set_span(Entity::Cell(1, 1), 2);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+----------------+---+\n",
            "|0-0                 |0-2|\n",
            "+---+----------------+---+\n",
            "|1-0|1-1                 |\n",
            "+---+----------------+---+\n",
            "|2-0|2-1xxxxxxxxxxxxx|2-2|\n",
            "+---+----------------+---+",
        )
    );

    let mut grid = util::grid_with_data::<3, 3>(&[((0, 2), "0-2xxxxxxxxxxxxx")]);
    grid.set_span(Entity::Cell(0, 0), 2);
    grid.set_span(Entity::Cell(1, 1), 2);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+----------------+\n",
            "|0-0    |0-2xxxxxxxxxxxxx|\n",
            "+---+---+----------------+\n",
            "|1-0|1-1                 |\n",
            "+---+---+----------------+\n",
            "|2-0|2-1|2-2             |\n",
            "+---+---+----------------+",
        )
    );
}

#[test]
fn render_spaned_column_in_first_cell_3x3() {
    let mut grid = util::grid_with_data::<3, 3>(&[((0, 0), "0-0xxxxxxx")]);
    grid.set_span(Entity::Cell(0, 0), 2);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+-----+----+---+\n",
            "|0-0xxxxxxx|0-2|\n",
            "+-----+----+---+\n",
            "|1-0  |1-1 |1-2|\n",
            "+-----+----+---+\n",
            "|2-0  |2-1 |2-2|\n",
            "+-----+----+---+",
        )
    );
}

#[test]
fn render_row_span_with_different_length() {
    let mut grid = util::grid_from([["first row", ""], ["0", "1"], ["a longer second row", ""]]);
    grid.set_span(Entity::Cell(0, 0), 2);
    grid.set_span(Entity::Cell(2, 0), 2);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---------+---------+\n",
            "|first row          |\n",
            "+---------+---------+\n",
            "|0        |1        |\n",
            "+---------+---------+\n",
            "|a longer second row|\n",
            "+---------+---------+",
        )
    );
}

#[test]
fn render_row_span_with_odd_length() {
    let mut grid = util::grid_from([["3   ", ""], ["2", "4"]]);
    grid.set_span(Entity::Cell(0, 0), 2);

    assert_eq!(
        grid.to_string(),
        concat!("+--+-+\n", "|3   |\n", "+--+-+\n", "|2 |4|\n", "+--+-+")
    );
}

#[test]
fn render_only_row_spaned() {
    let mut grid = util::grid::<3, 2>();
    grid.set_span(Entity::Cell(0, 0), 2);
    grid.set_span(Entity::Cell(1, 0), 2);
    grid.set_span(Entity::Cell(2, 0), 2);

    assert_eq!(
        grid.to_string(),
        "+-+-+\n\
         |0-0|\n\
         +-+-+\n\
         |1-0|\n\
         +-+-+\n\
         |2-0|\n\
         +-+-+",
    );
}

#[test]
fn grid_2x2_span_test() {
    let mut grid = util::grid_from([["123", ""], ["asd", "asd"]]);
    grid.set_span(Entity::Cell(0, 0), 2);

    assert_eq!(
        grid.to_string(),
        "+---+---+\n\
         |123    |\n\
         +---+---+\n\
         |asd|asd|\n\
         +---+---+",
    )
}

#[test]
fn grid_2x2_span_2_test() {
    let mut grid = util::grid_from([["1234", ""], ["asdw", ""]]);
    grid.set_span(Entity::Cell(0, 0), 2);
    grid.set_span(Entity::Cell(1, 0), 2);

    assert_eq!(
        grid.to_string(),
        "+--+-+\n\
         |1234|\n\
         +--+-+\n\
         |asdw|\n\
         +--+-+",
    );

    let mut grid = util::grid_from([["1", ""], ["a", ""]]);
    grid.set_span(Entity::Cell(0, 0), 2);
    grid.set_span(Entity::Cell(1, 0), 2);

    let str = grid.to_string();
    assert_eq!(
        str,
        "+++\n\
         |1|\n\
         +++\n\
         |a|\n\
         +++",
    );
}

#[test]
fn render_row_span_with_no_split_style() {
    let mut grid = util::grid::<2, 2>();
    grid.set_borders(Borders::default());

    grid.set_span(Entity::Cell(0, 0), 2);
    grid.set_alignment_horizontal(Entity::Cell(0, 0), AlignmentHorizontal::Center);

    assert_eq!(grid.to_string(), concat!(" 0-0  \n", "1-01-1"));
}

#[test]
#[ignore = "This is a pretty complex logic which is not clear if is worth to support"]
fn render_zero_span_of_first_cell() {
    let mut grid = util::grid::<2, 2>();
    grid.set_span(Entity::Cell(0, 0), 0);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+\n",
            "|0-1    |\n",
            "+---+---+\n",
            "|1-0|1-1|\n",
            "+---+---+",
        )
    );

    grid.set_span(Entity::Cell(1, 0), 0);

    assert_eq!(
        grid.to_string(),
        concat!("+-+-+\n", "|0-1|\n", "+-+-+\n", "|1-1|\n", "+-+-+")
    );
}

#[test]
fn render_zero_span_between_cells() {
    let mut grid = util::grid::<2, 3>();
    grid.set_span(Entity::Cell(0, 1), 0);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+---+\n",
            "|0-0    |0-2|\n",
            "+---+---+---+\n",
            "|1-0|1-1|1-2|\n",
            "+---+---+---+",
        )
    );

    grid.set_span(Entity::Cell(1, 1), 0);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+-+-+---+\n",
            "|0-0|0-2|\n",
            "+-+-+---+\n",
            "|1-0|1-2|\n",
            "+-+-+---+",
        )
    );
}

#[test]
fn render_zero_span_at_the_end() {
    let mut grid = util::grid::<2, 3>();
    grid.set_span(Entity::Cell(0, 1), 0);
    grid.set_span(Entity::Cell(0, 2), 0);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+---+---+\n",
            "|0-0        |\n",
            "+---+---+---+\n",
            "|1-0|1-1|1-2|\n",
            "+---+---+---+",
        )
    );

    grid.set_span(Entity::Cell(1, 1), 0);
    grid.set_span(Entity::Cell(1, 2), 0);

    assert_eq!(
        grid.to_string(),
        concat!("+-+++\n", "|0-0|\n", "+-+++\n", "|1-0|\n", "+-+++")
    );
}

#[test]
fn render_zero_span_grid() {
    let mut grid = util::grid::<2, 2>();

    grid.set_span(Entity::Cell(0, 0), 0);
    grid.set_span(Entity::Cell(0, 1), 0);
    grid.set_span(Entity::Cell(1, 0), 0);
    grid.set_span(Entity::Cell(1, 1), 0);

    // todo: determine if it's correct behaviour?
    assert_eq!(grid.to_string(), "+-+-+\n|0-0|\n+-+-+\n|1-0|\n+-+-+");
}

#[test]
#[ignore = "I am not sure what is the right behaiviour here"]
fn hieroglyph_handling() {
    let grid = util::grid_from([["哈哈", "哈"]]);

    assert_eq!(
        grid.to_string(),
        "+----+--+\n\
         |哈哈  |哈 |\n\
         +----+--+",
    )
}

#[test]
fn hieroglyph_multiline_handling() {
    let grid = util::grid_from([["哈哈", "哈\n哈"]]);

    assert_eq!(
        grid.to_string(),
        "+----+--+\n\
         |哈哈|哈|\n\
         |    |哈|\n\
         +----+--+",
    )
}

#[test]
fn hieroglyph_handling_2() {
    let grid = util::grid_from([["জী._ডি._ব্লক_সল্টলেক_দূর্গা_পুজো_২০১৮.jpg"], ["Hello"]]);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+------------------------------------+\n",
            "|জী._ডি._ব্লক_সল্টলেক_দূর্গা_পুজো_২০১৮.jpg|\n",
            "+------------------------------------+\n",
            "|Hello                               |\n",
            "+------------------------------------+",
        )
    )
}

#[test]
fn render_return_carige() {
    let grid = util::grid_with_data::<2, 2>(&[((0, 1), "123\r\r\r567")]);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+------+\n",
            "|0-0|123567|\n",
            "+---+------+\n",
            "|1-0|1-1   |\n",
            "+---+------+",
        )
    );

    let grid = util::grid_with_data::<2, 2>(&[((0, 1), "123\r\r\r567"), ((1, 1), "12345678")]);

    assert_eq!(
        grid.to_string(),
        concat!(
            "+---+--------+\n",
            "|0-0|123567  |\n",
            "+---+--------+\n",
            "|1-0|12345678|\n",
            "+---+--------+",
        )
    );
}
