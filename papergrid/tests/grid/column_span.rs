#![cfg(feature = "std")]

use papergrid::config::{AlignmentHorizontal, Borders, Entity, Indent, Sides};

use crate::util::grid;
use testing_table::test_table;

test_table!(
    row_span,
    grid(2, 2)
        .config(|cfg|{
            cfg.set_column_span((0, 0), 2);
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
    miltiline_span,
    grid(2, 2)
        .change_cell((0, 0), "0-0\n0-1")
        .config(|cfg|{
            cfg.set_column_span((0, 0), 2);
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
    row_span_multilane,
    grid(4, 3)
        .data([
            ["first line", "", "e.g."],
            ["0", "1", "2"],
            ["0", "1", "2"],
            ["full last line", "", ""],
        ])
        .config(|cfg|{
            cfg.set_column_span((0, 0), 2);
            cfg.set_column_span((3, 0), 3);
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
    row_span_with_horizontal_ident,
    grid(3, 2)
        .config(|cfg| {
            cfg.set_column_span((0, 0), 2);
            cfg.set_padding(
                Entity::Cell(1, 0),
                Sides::new(
                    Indent::spaced(4),
                    Indent::spaced(4),
                    Indent::zero(),
                    Indent::zero(),
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
    _row_span_3x3_with_horizontal_ident,
    grid(3, 3)
        .config(|cfg| {
            cfg.set_column_span((0, 0), 3);
            cfg.set_column_span((1, 0), 2);
            cfg.set_column_span((2, 0), 2);
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
    _3x3_with_2_colided_row_span_0,
    grid(3, 3)
        .change_cell((0, 0), "0-0xxxxxxx")
        .config(|cfg| {
            cfg.set_column_span((0, 0), 2);
            cfg.set_column_span((1, 1), 2);
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
    _3x3_with_2_colided_row_span_1,
    grid(3, 3)
        .change_cell((1, 1), "1-1xxxxxxx")
        .config(|cfg| {
            cfg.set_column_span((0, 0), 2);
            cfg.set_column_span((1, 1), 2);
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
    _3x3_with_2_colided_row_span_2,
    grid(3, 3)
        .change_cell((1, 1), "1-1xxxxxxx")
        .change_cell((2, 0), "2-0xxxxxxxxxxxxx")
        .config(|cfg| {
            cfg.set_column_span((0, 0), 2);
            cfg.set_column_span((1, 1), 2);
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
    _3x3_with_2_colided_row_span_3,
    grid(3, 3)
        .change_cell((2, 1), "2-1xxxxxxxxxxxxx")
        .config(|cfg| {
            cfg.set_column_span((0, 0), 2);
            cfg.set_column_span((1, 1), 2);
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
    _3x3_with_2_colided_row_span_4,
    grid(3, 3)
        .change_cell((0, 2), "0-2xxxxxxxxxxxxx")
        .config(|cfg| {
            cfg.set_column_span((0, 0), 2);
            cfg.set_column_span((1, 1), 2);
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
    spaned_column_in_first_cell_3x3,
    grid(3, 3)
        .change_cell((0, 0), "0-0xxxxxxx")
        .config(|cfg| cfg.set_column_span((0, 0), 2))
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
    row_span_with_different_length,
    grid(3, 2)
        .data([["first row", ""], ["0", "1"], ["a longer second row", ""]])
        .config(|cfg| {
            cfg.set_column_span((0, 0), 2);
            cfg.set_column_span((2, 0), 2);
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
    row_span_with_odd_length,
    grid(2, 2)
        .data([["3   ", ""], ["2", "4"]])
        .config(|cfg| cfg.set_column_span((0, 0), 2))
        .build(),
    "+--+-+"
    "|3   |"
    "+--+-+"
    "|2 |4|"
    "+--+-+"
);

test_table!(
    only_row_spaned,
    grid(3, 2)
        .config(|cfg| {
            cfg.set_column_span((0, 0), 2);
            cfg.set_column_span((1, 0), 2);
            cfg.set_column_span((2, 0), 2);
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
        .config(|cfg| cfg.set_column_span((0, 0), 2))
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
            cfg.set_column_span((0, 0), 2);
            cfg.set_column_span((1, 0), 2);
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
            cfg.set_column_span((0, 0), 2);
            cfg.set_column_span((1, 0), 2);
        })
        .build(),
    "+++"
    "|1|"
    "+++"
    "|a|"
    "+++"
);

test_table!(
    row_span_with_no_split_style,
    grid(2, 2)
        .config(|cfg| {
            cfg.set_borders(Borders::default());
            cfg.set_column_span((0, 0), 2);
            cfg.set_alignment_horizontal(Entity::Cell(0, 0), AlignmentHorizontal::Center);
        })
        .build(),
    " 0-0  "
    "1-01-1"
);

test_table!(
    _2x3_zero_span_between_cells_0,
    grid(2, 3)
        .config(|cfg| cfg.set_column_span((0, 0), 2))
        .build(),
    "+---+---+---+"
    "|0-0    |0-2|"
    "+---+---+---+"
    "|1-0|1-1|1-2|"
    "+---+---+---+"
);

test_table!(
    _2x3_zero_span_between_cells_1,
    grid(2, 3)
        .config(|cfg| {
            cfg.set_column_span((0, 0), 2);
            cfg.set_column_span((1, 0), 2);
        })
        .build(),
    "+-+-+---+"
    "|0-0|0-2|"
    "+-+-+---+"
    "|1-0|1-2|"
    "+-+-+---+"
);

test_table!(
    _2x3_zero_span_at_the_end_0,
    grid(2, 3)
        .config(|cfg| {
            cfg.set_column_span((0, 0), 3);
        })
        .build(),
    "+---+---+---+"
    "|0-0        |"
    "+---+---+---+"
    "|1-0|1-1|1-2|"
    "+---+---+---+"
);

test_table!(
    _2x3_zero_span_at_the_end_1,
    grid(2, 3)
        .config(|cfg| {
            cfg.set_column_span((0, 0), 3);
            cfg.set_column_span((1, 0), 3);
        })
        .build(),
    "+-+++"
    "|0-0|"
    "+-+++"
    "|1-0|"
    "+-+++"
);

test_table!(
    zero_span_grid,
    grid(2, 2)
        .data([["123", ""], ["asd", "asd"]])
        .config(|cfg| {
            cfg.set_column_span((0, 0), 2);
            cfg.set_column_span((1, 0), 2);
        })
        .build(),
    "+-+-+"
    "|123|"
    "+-+-+"
    "|asd|"
    "+-+-+"
);

test_table!(
    zero_span_grid_1,
    grid(2, 2)
        .data([["123", ""], ["asd", "asd"]])
        .config(|cfg| {
            cfg.set_row_span((0, 0), 2);
            cfg.set_row_span((0, 1), 2);
        })
        .build(),
    "+---++"
    "+123++"
    "+---++"
);

test_table!(
    zero_span_grid_2,
    grid(2, 2)
        .data([["123", "axc"], ["asd", "asd"]])
        .config(|cfg| {
            cfg.set_row_span((0, 0), 2);
            cfg.set_row_span((0, 1), 2);
        })
        .build(),
    "+---+---+"
    "+123+axc+"
    "+---+---+"
);

test_table!(
    zero_span_is_not_handled,
    grid(2, 2)
        .config(|cfg| { cfg.set_column_span((0, 1), 0); })
        .build(),
    "+---+---+"
    "|0-0|0-1|"
    "+---+---+"
    "|1-0|1-1|"
    "+---+---+"
);
