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

use papergrid::{AlignmentHorizontal, AlignmentVertical, Borders, Entity, Indent, Padding};

use crate::util::{grid, test_table};

mod util;

test_table!(render_0x0, grid(0, 0).build(), "");

test_table!(
    render_1x1,
    grid(1, 1).change_cell((0, 0), "one line").build(),
    "+--------+"
    "|one line|"
    "+--------+"
);

test_table!(
    render_2x2,
    grid(2, 2).build(),
    "+---+---+"
    "|0-0|0-1|"
    "+---+---+"
    "|1-0|1-1|"
    "+---+---+"
);

test_table!(
    render_3x2,
    grid(3, 2).build(),
    "+---+---+"
    "|0-0|0-1|"
    "+---+---+"
    "|1-0|1-1|"
    "+---+---+"
    "|2-0|2-1|"
    "+---+---+"
);

test_table!(
    render_1x2,
    grid(1, 2).data([["hello", "world"]]).build(),
    "+-----+-----+"
    "|hello|world|"
    "+-----+-----+"
);

test_table!(
    render_multilane,
    grid(2, 2)
        .data([
            ["left\ncell", "right one"],
            ["the second column got the beginning here", "and here\nwe\nsee\na\nlong\nstring"],
        ])
        .build(),
    "+----------------------------------------+---------+"
    "|left                                    |right one|"
    "|cell                                    |         |"
    "+----------------------------------------+---------+"
    "|the second column got the beginning here|and here |"
    "|                                        |we       |"
    "|                                        |see      |"
    "|                                        |a        |"
    "|                                        |long     |"
    "|                                        |string   |"
    "+----------------------------------------+---------+"
);

test_table!(
    render_multilane_alignment,
    grid(2, 2)
        .config(|cfg|{
            cfg.set_alignment_horizontal(Entity::Cell(0, 0), AlignmentHorizontal::Center);
            cfg.set_alignment_horizontal(Entity::Cell(1, 1), AlignmentHorizontal::Right);
        })
        .data([
            ["left\ncell", "right one"],
            ["the second column got the beginning here", "and here\nwe\nsee\na\nlong\nstring"],
        ])
        .build(),
    "+----------------------------------------+---------+"
    "|                  left                  |right one|"
    "|                  cell                  |         |"
    "+----------------------------------------+---------+"
    "|the second column got the beginning here| and here|"
    "|                                        | we      |"
    "|                                        | see     |"
    "|                                        | a       |"
    "|                                        | long    |"
    "|                                        | string  |"
    "+----------------------------------------+---------+"
);

test_table!(
    render_multilane_vertical_alignment,
    grid(2, 2)
        .data([
            ["left\ncell", "right one"],
            ["the second column got the beginning here", "and here\nwe\nsee\na\nlong\nstring"],
        ])
        .config(|cfg|{
            cfg.set_alignment_horizontal(Entity::Cell(0, 0), AlignmentHorizontal::Center);
            cfg.set_alignment_vertical(Entity::Cell(1, 0), AlignmentVertical::Center);
            cfg.set_alignment_horizontal(Entity::Cell(1, 1), AlignmentHorizontal::Right);
        })
        .build(),
    "+----------------------------------------+---------+"
    "|                  left                  |right one|"
    "|                  cell                  |         |"
    "+----------------------------------------+---------+"
    "|                                        | and here|"
    "|                                        | we      |"
    "|the second column got the beginning here| see     |"
    "|                                        | a       |"
    "|                                        | long    |"
    "|                                        | string  |"
    "+----------------------------------------+---------+"
);

test_table!(
    render_empty_cell,
    grid(2, 2).change_cell((0, 1), "").build(),
    "+---+---+"
    "|0-0|   |"
    "+---+---+"
    "|1-0|1-1|"
    "+---+---+"
);

test_table!(
    render_row_span,
    grid(2, 2)
        .config(|cfg|{
            cfg.set_span((0, 0), 2);
            cfg.set_alignment_horizontal(Entity::Cell(0, 0), AlignmentHorizontal::Center);
        })
        .build(),
    "+---+---+"
    "|  0-0  |"
    "+---+---+"
    "|1-0|1-1|"
    "+---+---+"
);

test_table!(
    render_miltiline_span,
    grid(2, 2)
        .change_cell((0, 0), "0-0\n0-1")
        .config(|cfg|{
            cfg.set_span((0, 0), 2);
            cfg.set_alignment_horizontal(Entity::Cell(0, 0), AlignmentHorizontal::Center);
        })
        .build(),
    "+---+---+"
    "|  0-0  |"
    "|  0-1  |"
    "+---+---+"
    "|1-0|1-1|"
    "+---+---+"
);

test_table!(
    render_row_span_multilane,
    grid(4, 3)
        .data([
            ["first line", "", "e.g."],
            ["0", "1", "2"],
            ["0", "1", "2"],
            ["full last line", "", ""],
        ])
        .config(|cfg|{
            cfg.set_span((0, 0), 2);
            cfg.set_span((3, 0), 3);
        })
        .build(),
    "+-----+----+----+"
    "|first line|e.g.|"
    "+-----+----+----+"
    "|0    |1   |2   |"
    "+-----+----+----+"
    "|0    |1   |2   |"
    "+-----+----+----+"
    "|full last line |"
    "+-----+----+----+"
);

test_table!(
    render_row_span_with_horizontal_ident,
    grid(3, 2)
        .config(|cfg| {
            cfg.set_span((0, 0), 2);
            cfg.set_padding(
                Entity::Cell(1, 0),
                Padding::new(
                    Indent::spaced(4),
                    Indent::spaced(4),
                    Indent::default(),
                    Indent::default(),
                ),
            );
        })
        .build(),
    "+-----------+---+"
    "|0-0            |"
    "+-----------+---+"
    "|    1-0    |1-1|"
    "+-----------+---+"
    "|2-0        |2-1|"
    "+-----------+---+"
);

test_table!(
    render_row_span_3x3_with_horizontal_ident,
    grid(3, 3)
        .config(|cfg| {
            cfg.set_span((0, 0), 3);
            cfg.set_span((1, 0), 2);
            cfg.set_span((2, 0), 2);
        })
        .build(),
    "+-+-+---+"
    "|0-0    |"
    "+-+-+---+"
    "|1-0|1-2|"
    "+-+-+---+"
    "|2-0|2-2|"
    "+-+-+---+"
);

test_table!(
    render_3x3_with_2_colided_row_span_0,
    grid(3, 3)
        .change_cell((0, 0), "0-0xxxxxxx")
        .config(|cfg| {
            cfg.set_span((0, 0), 2);
            cfg.set_span((1, 1), 2);
        })
        .build(),
    "+-----+----+---+"
    "|0-0xxxxxxx|0-2|"
    "+-----+----+---+"
    "|1-0  |1-1     |"
    "+-----+----+---+"
    "|2-0  |2-1 |2-2|"
    "+-----+----+---+"
);

test_table!(
    render_3x3_with_2_colided_row_span_1,
    grid(3, 3)
        .change_cell((1, 1), "1-1xxxxxxx")
        .config(|cfg| {
            cfg.set_span((0, 0), 2);
            cfg.set_span((1, 1), 2);
        })
        .build(),
    "+---+-----+----+"
    "|0-0      |0-2 |"
    "+---+-----+----+"
    "|1-0|1-1xxxxxxx|"
    "+---+-----+----+"
    "|2-0|2-1  |2-2 |"
    "+---+-----+----+"
);

test_table!(
    render_3x3_with_2_colided_row_span_2,
    grid(3, 3)
        .change_cell((1, 1), "1-1xxxxxxx")
        .change_cell((2, 0), "2-0xxxxxxxxxxxxx")
        .config(|cfg| {
            cfg.set_span((0, 0), 2);
            cfg.set_span((1, 1), 2);
        })
        .build(),
    "+----------------+-----+----+"
    "|0-0                   |0-2 |"
    "+----------------+-----+----+"
    "|1-0             |1-1xxxxxxx|"
    "+----------------+-----+----+"
    "|2-0xxxxxxxxxxxxx|2-1  |2-2 |"
    "+----------------+-----+----+"
);

test_table!(
    render_3x3_with_2_colided_row_span_3,
    grid(3, 3)
        .change_cell((2, 1), "2-1xxxxxxxxxxxxx")
        .config(|cfg| {
            cfg.set_span((0, 0), 2);
            cfg.set_span((1, 1), 2);
        })
        .build(),
    "+---+----------------+---+"
    "|0-0                 |0-2|"
    "+---+----------------+---+"
    "|1-0|1-1                 |"
    "+---+----------------+---+"
    "|2-0|2-1xxxxxxxxxxxxx|2-2|"
    "+---+----------------+---+"
);

test_table!(
    render_3x3_with_2_colided_row_span_4,
    grid(3, 3)
        .change_cell((0, 2), "0-2xxxxxxxxxxxxx")
        .config(|cfg| {
            cfg.set_span((0, 0), 2);
            cfg.set_span((1, 1), 2);
        })
        .build(),
    "+---+---+----------------+"
    "|0-0    |0-2xxxxxxxxxxxxx|"
    "+---+---+----------------+"
    "|1-0|1-1                 |"
    "+---+---+----------------+"
    "|2-0|2-1|2-2             |"
    "+---+---+----------------+"
);

test_table!(
    render_spaned_column_in_first_cell_3x3,
    grid(3, 3)
        .change_cell((0, 0), "0-0xxxxxxx")
        .config(|cfg| cfg.set_span((0, 0), 2))
        .build(),
    "+-----+----+---+"
    "|0-0xxxxxxx|0-2|"
    "+-----+----+---+"
    "|1-0  |1-1 |1-2|"
    "+-----+----+---+"
    "|2-0  |2-1 |2-2|"
    "+-----+----+---+"
);

test_table!(
    render_row_span_with_different_length,
    grid(3, 2)
        .data([["first row", ""], ["0", "1"], ["a longer second row", ""]])
        .config(|cfg| {
            cfg.set_span((0, 0), 2);
            cfg.set_span((2, 0), 2);
        })
        .build(),
    "+---------+---------+"
    "|first row          |"
    "+---------+---------+"
    "|0        |1        |"
    "+---------+---------+"
    "|a longer second row|"
    "+---------+---------+"
);

test_table!(
    render_row_span_with_odd_length,
    grid(2, 2)
        .data([["3   ", ""], ["2", "4"]])
        .config(|cfg| cfg.set_span((0, 0), 2))
        .build(),
    "+--+-+"
    "|3   |"
    "+--+-+"
    "|2 |4|"
    "+--+-+"
);

test_table!(
    render_only_row_spaned,
    grid(3, 2)
        .config(|cfg| {
            cfg.set_span((0, 0), 2);
            cfg.set_span((1, 0), 2);
            cfg.set_span((2, 0), 2);
        })
        .build(),
    "+-+-+"
    "|0-0|"
    "+-+-+"
    "|1-0|"
    "+-+-+"
    "|2-0|"
    "+-+-+"
);

test_table!(
    grid_2x2_span_test,
    grid(2, 2)
        .data([["123", ""], ["asd", "asd"]])
        .config(|cfg| cfg.set_span((0, 0), 2))
        .build(),
    "+---+---+"
    "|123    |"
    "+---+---+"
    "|asd|asd|"
    "+---+---+"
);

test_table!(
    grid_2x2_span_2_test_0,
    grid(2, 2)
        .data([["1234", ""], ["asdw", ""]])
        .config(|cfg| {
            cfg.set_span((0, 0), 2);
            cfg.set_span((1, 0), 2);
        })
        .build(),
    "+--+-+"
    "|1234|"
    "+--+-+"
    "|asdw|"
    "+--+-+"
);

test_table!(
    grid_2x2_span_2_test_1,
    grid(2, 2)
        .data([["1", ""], ["a", ""]])
        .config(|cfg| {
            cfg.set_span((0, 0), 2);
            cfg.set_span((1, 0), 2);
        })
        .build(),
    "+++"
    "|1|"
    "+++"
    "|a|"
    "+++"
);

test_table!(
    render_row_span_with_no_split_style,
    grid(2, 2)
        .config(|cfg| {
            cfg.set_borders(Borders::default());
            cfg.set_span((0, 0), 2);
            cfg.set_alignment_horizontal(Entity::Cell(0, 0), AlignmentHorizontal::Center);
        })
        .build(),
    " 0-0  "
    "1-01-1"
);

test_table!(
    render_2x3_zero_span_between_cells_0,
    grid(2, 3)
        .config(|cfg| cfg.set_span((0, 1), 0))
        .build(),
    "+---+---+---+"
    "|0-0    |0-2|"
    "+---+---+---+"
    "|1-0|1-1|1-2|"
    "+---+---+---+"
);

test_table!(
    render_2x3_zero_span_between_cells_1,
    grid(2, 3)
        .config(|cfg| {
            cfg.set_span((0, 1), 0);
            cfg.set_span((1, 1), 0);
        })
        .build(),
    "+-+-+---+"
    "|0-0|0-2|"
    "+-+-+---+"
    "|1-0|1-2|"
    "+-+-+---+"
);

test_table!(
    render_2x3_zero_span_at_the_end_0,
    grid(2, 3)
        .config(|cfg| {
            cfg.set_span((0, 1), 0);
            cfg.set_span((0, 2), 0);
        })
        .build(),
    "+---+---+---+"
    "|0-0        |"
    "+---+---+---+"
    "|1-0|1-1|1-2|"
    "+---+---+---+"
);

test_table!(
    render_2x3_zero_span_at_the_end_1,
    grid(2, 3)
        .config(|cfg| {
            cfg.set_span((0, 1), 0);
            cfg.set_span((0, 2), 0);
            cfg.set_span((1, 1), 0);
            cfg.set_span((1, 2), 0);
        })
        .build(),
    "+-+++"
    "|0-0|"
    "+-+++"
    "|1-0|"
    "+-+++"
);

// todo: determine if it's correct behaviour?
test_table!(
    render_zero_span_grid,
    grid(2, 2)
        .data([["123", ""], ["asd", "asd"]])
        .config(|cfg| {
            cfg.set_span((0, 0), 0);
            cfg.set_span((0, 1), 0);
            cfg.set_span((1, 0), 0);
            cfg.set_span((1, 1), 0);
        })
        .build(),
    "+-+-+"
    "|123|"
    "+-+-+"
    "|asd|"
    "+-+-+"
);

test_table!(
    hieroglyph_multiline_handling,
    grid(1, 2).data([["哈哈", "哈\n哈"]]).build(),
    "+----+--+"
    "|哈哈|哈|"
    "|    |哈|"
    "+----+--+"
);

test_table!(
    hieroglyph_handling_2,
    grid(2, 1).data([["জী._ডি._ব্লক_সল্টলেক_দূর্গা_পুজো_২০১৮.jpg"], ["Hello"]]).build(),
    "+------------------------------------+"
    "|জী._ডি._ব্লক_সল্টলেক_দূর্গা_পুজো_২০১৮.jpg|"
    "+------------------------------------+"
    "|Hello                               |"
    "+------------------------------------+"
);

test_table!(
    render_return_carige_0,
    grid(2, 2).change_cell((0, 1), "123\r\r\r567").build(),
    "+---+------+"
    "|0-0|123567|"
    "+---+------+"
    "|1-0|1-1   |"
    "+---+------+"
);

test_table!(
    render_return_carige_1,
    grid(2, 2).change_cell((1, 1), "12345678").change_cell((0, 1), "123\r\r\r567").build(),
    "+---+--------+"
    "|0-0|123567  |"
    "+---+--------+"
    "|1-0|12345678|"
    "+---+--------+"
);

// #[test]
// #[ignore = "This is a pretty complex logic which is not clear if is worth to support"]
// fn render_zero_span_of_first_cell() {
//     let mut grid = util::grid::<2, 2>();
//     grid.set_span(Entity::Cell(0, 0), 0);

//     assert_eq!(
//         grid,
//         concat!(
//             "+---+---+\n",
//             "|0-1    |\n",
//             "+---+---+\n",
//             "|1-0|1-1|\n",
//             "+---+---+",
//         )
//     );

//     grid.set_span(Entity::Cell(1, 0), 0);

//     assert_eq!(
//         grid,
//         concat!("+-+-+\n", "|0-1|\n", "+-+-+\n", "|1-1|\n", "+-+-+")
//     );
// }

// #[test]
// #[ignore = "I am not sure what is the right behaiviour here"]
// fn hieroglyph_handling() {
//     let grid = util::grid_from([["哈哈", "哈"]]);

//     assert_eq!(
//         grid,
//         "+----+--+\n\
//          |哈哈  |哈 |\n\
//          +----+--+",
//     )
// }
