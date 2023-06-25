#![cfg(feature = "std")]

use papergrid::config::{
    AlignmentHorizontal, AlignmentVertical, Borders,
    Entity::{self, *},
    Indent, Sides,
};

use crate::util::grid;
use testing_table::test_table;

test_table!(
    _2x2_vertical_alignment_center,
    grid(2, 2)
        .config(|cfg|{
            cfg.set_row_span((0, 0), 2);
            cfg.set_alignment_vertical(Cell(0, 0), AlignmentVertical::Center);
        })
        .build(),
    "+---+---+"
    "|   |0-1|"
    "+0-0+---+"
    "|   |1-1|"
    "+---+---+"
);

test_table!(
    _2x2_vertical_alignment_bottom,
    grid(2, 2)
        .config(|cfg|{
            cfg.set_row_span((0, 0), 2);
            cfg.set_alignment_vertical(Cell(0, 0), AlignmentVertical::Bottom);
        })
        .build(),
    "+---+---+"
    "|   |0-1|"
    "+   +---+"
    "|0-0|1-1|"
    "+---+---+"
);

test_table!(
    _2x2_multiline,
    grid(2, 2)
        .change_cell((0, 0), "0-0\n0-1xxx")
        .config(|cfg|{
            cfg.set_row_span((0, 0), 2);
            cfg.set_alignment_vertical(Cell(0, 0), AlignmentVertical::Center);
        })
        .build(),
    "+------+---+"
    "|0-0   |0-1|"
    "+0-1xxx+---+"
    "|      |1-1|"
    "+------+---+"
);

test_table!(
    _2x2_multiline_vertical_alignment_bottom,
    grid(2, 2)
        .change_cell((0, 0), "0-0\n0-1xxx")
        .config(|cfg|{
            cfg.set_row_span((0, 0), 2);
            cfg.set_alignment_vertical(Cell(0, 0), AlignmentVertical::Bottom);
        })
        .build(),
    "+------+---+"
    "|      |0-1|"
    "+0-0   +---+"
    "|0-1xxx|1-1|"
    "+------+---+"
);

test_table!(
    _4x3_multiline_0,
    grid(4, 3)
        .data([
            ["first line", "0-1", "full last line"],
            ["", "1", ""],
            ["0", "1", ""],
            ["3-0", "3-1", ""],
        ])
        .config(|cfg|{
            cfg.set_row_span((0, 0), 2);
            cfg.set_row_span((0, 2), 4);
        })
        .build(),
    "+----------+---+--------------+"
    "|first line|0-1|full last line|"
    "+          +---+              +"
    "|          |1  |              |"
    "+----------+---+              +"
    "|0         |1  |              |"
    "+----------+---+              +"
    "|3-0       |3-1|              |"
    "+----------+---+--------------+"
);

test_table!(
    _3x2_with_horizontal_ident_on_spanned_cell,
    grid(3, 2)
        .config(|cfg|{
            cfg.set_row_span((0, 0), 2);
            cfg.set_padding(Cell(1, 0), Sides::new(Indent::spaced(4), Indent::spaced(4), Indent::default(), Indent::default()));
        })
        .build(),
    "+---+---+"
    "|0-0|0-1|"
    "+   +---+"
    "|   |1-1|"
    "+---+---+"
    "|2-0|2-1|"
    "+---+---+"
);

test_table!(
    _3x2_with_horizontal_ident,
    grid(3, 2)
        .config(|cfg|{
            cfg.set_row_span((0, 0), 2);
            cfg.set_padding(Cell(0, 0), Sides::new(Indent::spaced(4), Indent::spaced(4), Indent::default(), Indent::default()));
        })
        .build(),
    "+-----------+---+"
    "|    0-0    |0-1|"
    "+           +---+"
    "|           |1-1|"
    "+-----------+---+"
    "|2-0        |2-1|"
    "+-----------+---+"
);

test_table!(
    _3x2_with_vertical_ident,
    grid(3, 2)
        .config(|cfg|{
            cfg.set_row_span((0, 0), 2);
            cfg.set_padding(Cell(0, 0), Sides::new(Indent::default(), Indent::default(), Indent::spaced(4), Indent::spaced(4)));
        })
        .build(),
    "+---+---+"
    "|   |0-1|"
    "|   |   |"
    "|   |   |"
    "|   |   |"
    "+0-0+---+"
    "|   |1-1|"
    "|   |   |"
    "|   |   |"
    "|   |   |"
    "+---+---+"
    "|2-0|2-1|"
    "+---+---+"
);

test_table!(
    _3x3_render_0,
    grid(3, 3)
        .config(|cfg|{
            cfg.set_row_span((0, 0), 3);
            cfg.set_row_span((0, 1), 2);
            cfg.set_row_span((0, 2), 2);
        })
        .build(),
    "+---+---+---+"
    "+0-0+0-1+0-2+"
    "+   +---+---+"
    "|   |2-1|2-2|"
    "+---+---+---+"
);

test_table!(
    _3x3_render_1,
    grid(3, 3)
        .change_cell((0, 1), "t\ne\nx\nt")
        .config(|cfg|{
            cfg.set_row_span((0, 0), 3);
            cfg.set_row_span((0, 1), 2);
            cfg.set_row_span((0, 2), 2);
        })
        .build(),
    "+---+---+---+"
    "|0-0|t  |0-2|"
    "|   |e  |   |"
    "+   +x  +   +"
    "|   |t  |   |"
    "+   +---+---+"
    "|   |2-1|2-2|"
    "+---+---+---+"
);

test_table!(
    _3x3_coliison_0,
    grid(3, 3)
        .change_cell((0, 0), "0-0xxxxxxx")
        .config(|cfg|{
            cfg.set_row_span((0, 0), 2);
            cfg.set_row_span((1, 1), 2);
        })
        .build(),
    "+----------+---+---+"
    "|0-0xxxxxxx|0-1|0-2|"
    "+          +---+---+"
    "|          |1-1|1-2|"
    "+----------+   +---+"
    "|2-0       |   |2-2|"
    "+----------+---+---+"
);

test_table!(
    _3x3_coliison_1,
    grid(3, 3)
        .change_cell((1, 1), "1-1xxxxxxx")
        .config(|cfg|{
            cfg.set_row_span((0, 0), 2);
            cfg.set_row_span((1, 1), 2);
        })
        .build(),
    "+---+----------+---+"
    "|0-0|0-1       |0-2|"
    "+   +----------+---+"
    "|   |1-1xxxxxxx|1-2|"
    "+---+          +---+"
    "|2-0|          |2-2|"
    "+---+----------+---+"
);

test_table!(
    _3x3_coliison_2,
    grid(3, 3)
        .change_cell((1, 1), "1-1\nx\nx\nxxxxx")
        .change_cell((0, 2), "2-0x\nxxx\nxx\nxxxxxxx")
        .config(|cfg|{
            cfg.set_row_span((0, 0), 2);
            cfg.set_row_span((1, 1), 2);
        })
        .build(),
    "+---+-----+-------+"
    "|0-0|0-1  |2-0x   |"
    "|   |     |xxx    |"
    "|   |     |xx     |"
    "|   |     |xxxxxxx|"
    "+   +-----+-------+"
    "|   |1-1  |1-2    |"
    "|   |x    |       |"
    "+---+x    +-------+"
    "|2-0|xxxxx|2-2    |"
    "+---+-----+-------+"
);

test_table!(
    _3x3_coliison_3,
    grid(3, 3)
        .change_cell((1, 2), "2-1\nxx\nxx\nxx\nxxxxxx\nx")
        .config(|cfg|{
            cfg.set_row_span((0, 0), 2);
            cfg.set_row_span((1, 1), 2);
        })
        .build(),
    "+---+---+------+"
    "|0-0|0-1|0-2   |"
    "+   +---+------+"
    "|   |1-1|2-1   |"
    "|   |   |xx    |"
    "|   |   |xx    |"
    "|   |   |xx    |"
    "|   |   |xxxxxx|"
    "|   |   |x     |"
    "+---+   +------+"
    "|2-0|   |2-2   |"
    "+---+---+------+"
);

test_table!(
    _3x3_coliison_4,
    grid(3, 3)
        .change_cell((2, 1), "0-2\nx\nx\nx\nx\nxxxxxxx\nx\nx")
        .config(|cfg|{
            cfg.set_row_span((0, 0), 2);
            cfg.set_row_span((1, 2), 2);
        })
        .build(),
    "+---+-------+---+"
    "|0-0|0-1    |0-2|"
    "+   +-------+---+"
    "|   |1-1    |1-2|"
    "+---+-------+   +"
    "|2-0|0-2    |   |"
    "|   |x      |   |"
    "|   |x      |   |"
    "|   |x      |   |"
    "|   |x      |   |"
    "|   |xxxxxxx|   |"
    "|   |x      |   |"
    "|   |x      |   |"
    "+---+-------+---+"
);

test_table!(
    _3x3_first_row,
    grid(3, 3)
        .change_cell((0, 0), "0-0\nxx\nx\nx\nx\nx\nx")
        .config(|cfg|{ cfg.set_row_span((0, 0), 2); })
        .build(),
    "+---+---+---+"
    "|0-0|0-1|0-2|"
    "|xx |   |   |"
    "|x  |   |   |"
    "+x  +---+---+"
    "|x  |1-1|1-2|"
    "|x  |   |   |"
    "|x  |   |   |"
    "+---+---+---+"
    "|2-0|2-1|2-2|"
    "+---+---+---+"
);

test_table!(
    _2x3_with_different_length,
    grid(2, 3)
        .change_cell((0, 0), "f\nir\nst\n ro\nw")
        .change_cell((0, 2), "a\n \nlonger\n \nsecond\n \nrow")
        .change_cell((1, 0), "0")
        .change_cell((1, 1), "1")
        .config(|cfg|{
            cfg.set_row_span((0, 0), 2);
            cfg.set_row_span((0, 2), 2);
        })
        .build(),
    "+---+---+------+"
    "|f  |0-1|a     |"
    "|ir |   |      |"
    "|st |   |longer|"
    "+ ro+---+      +"
    "|w  |1  |second|"
    "|   |   |      |"
    "|   |   |row   |"
    "+---+---+------+"
);

test_table!(
    _2x2_with_odd_length,
    grid(2, 2)
        .change_cell((0, 0), "3\n \n \n ")
        .change_cell((0, 1), "2")
        .change_cell((1, 1), "4")
        .config(|cfg|{
            cfg.set_row_span((0, 0), 2);
        })
        .build(),
    "+-+-+"
    "|3|2|"
    "| | |"
    "+ +-+"
    "| |4|"
    "+-+-+"
);

test_table!(
    _2x3_only_col_spaned,
    grid(2, 3)
        .config(|cfg|{
            cfg.set_row_span((0, 0), 2);
            cfg.set_row_span((0, 1), 2);
            cfg.set_row_span((0, 2), 2);
        })
        .build(),
    "+---+---+---+"
    "+0-0+0-1+0-2+"
    "+---+---+---+"
);

test_table!(
    _2x2_render_0,
    grid(2, 2)
        .change_cell((0, 0), "1\n\n\n\n\n\n\n23")
        .config(|cfg|{
            cfg.set_row_span((0, 0), 2);
        })
        .build(),
    "+--+---+"
    "|1 |0-1|"
    "|  |   |"
    "|  |   |"
    "|  |   |"
    "+  +---+"
    "|  |1-1|"
    "|  |   |"
    "|23|   |"
    "+--+---+"
);

test_table!(
    _2x2_render_1,
    grid(2, 2)
        .data([["12\n3\n4", "a\ns\ndw"], ["asd", "asd"]])
        .config(|cfg|{
            cfg.set_row_span((0, 0), 2);
            cfg.set_row_span((0, 1), 2);
        })
        .build(),
    "+--+--+"
    "|12|a |"
    "+3 +s +"
    "|4 |dw|"
    "+--+--+"
);

test_table!(
    _2x2_render_2,
    grid(2, 2)
        .data([["1", "a"], ["asd", "asd"]])
        .config(|cfg|{
            cfg.set_row_span((0, 0), 2);
            cfg.set_row_span((0, 1), 2);
        })
        .build(),
    "+-+-+"
    "+1+a+"
    "+-+-+"
);

test_table!(
    _2x2_render_3,
    grid(2, 2)
        .data([["1as\nd\n", "a"], ["as\ndasdds\na", "asd"]])
        .config(|cfg|{
            cfg.set_row_span((0, 0), 2);
            cfg.set_row_span((0, 1), 2);
        })
        .build(),
    "+---+-+"
    "|1as|a|"
    "+d  + +"
    "|   | |"
    "+---+-+"
);

test_table!(
    _2x2_render_4,
    grid(2, 2)
        .data([["1as\nd\n", "a"], ["as\ndasdds\na", "asd"]])
        .config(|cfg|{
            cfg.set_row_span((0, 0), 2);
            cfg.set_row_span((0, 1), 2);
            cfg.set_alignment_vertical(Entity::Global, AlignmentVertical::Center)
        })
        .build(),
    "+---+-+"
    "|1as| |"
    "+d  +a+"
    "|   | |"
    "+---+-+"
);

test_table!(
    _2x2_render_5,
    grid(2, 2)
        .data([["1a\ns\nd\n", "a"], ["as\ndasdds\na", "asd"]])
        .config(|cfg|{
            cfg.set_row_span((0, 0), 2);
            cfg.set_row_span((0, 1), 2);
            cfg.set_alignment_vertical(Entity::Global, AlignmentVertical::Center)
        })
        .build(),
    "+--+-+"
    "|1a| |"
    "|s |a|"
    "+d + +"
    "|  | |"
    "+--+-+"
);

test_table!(
    _2x2_render_6,
    grid(2, 2)
        .data([["1a\ns\nd", "a"], ["as\ndasdds\na", "asd"]])
        .config(|cfg|{
            cfg.set_row_span((0, 0), 2);
            cfg.set_row_span((0, 1), 2);
            cfg.set_alignment_vertical(Entity::Global, AlignmentVertical::Bottom)
        })
        .build(),
    "+--+-+"
    "|1a| |"
    "+s + +"
    "|d |a|"
    "+--+-+"
);

test_table!(
    _2x2_with_no_split_style,
    grid(2, 2)
        .change_cell((0, 0), "1\n2\n3")
        .config(|cfg|{
            cfg.set_borders(Borders::default());
            cfg.set_row_span((0, 0), 2);
            cfg.set_alignment_vertical(Cell(0, 0), AlignmentVertical::Center);
        })
        .build(),
        "10-1"
        "2   "
        "31-1"
);

test_table!(
    _3x2_with_zero_row_span_0,
    grid(3, 2)
        .config(|cfg|{
            cfg.set_row_span((0, 0), 2);
        })
        .build(),
    "+---+---+"
    "|0-0|0-1|"
    "+   +---+"
    "|   |1-1|"
    "+---+---+"
    "|2-0|2-1|"
    "+---+---+"
);

test_table!(
    _3x2_with_zero_row_span_1,
    grid(3, 2)
        .config(|cfg|{
            cfg.set_row_span((0, 0), 2);
            cfg.set_row_span((0, 1), 2);
        })
        .build(),
    "+---+---+"
    "+0-0+0-1+"
    "+---+---+"
    "|2-0|2-1|"
    "+---+---+"
);

test_table!(
    _3x2_with_zero_row_span_2,
    grid(3, 2)
        .config(|cfg|{
            cfg.set_row_span((0, 1), 3);
        })
        .build(),
    "+---+---+"
    "|0-0|0-1|"
    "+---+   +"
    "|1-0|   |"
    "+---+   +"
    "|2-0|   |"
    "+---+---+"
);

test_table!(
    _3x2_with_zero_row_span_3,
    grid(3, 2)
        .config(|cfg|{
            cfg.set_row_span((0, 1), 3);
            cfg.set_row_span((0, 0), 3);
        })
        .build(),
    "+---+---+"
    "+0-0+0-1+"
    "+   +   +"
    "+---+---+"
);

test_table!(
    _2x2_with_zero_row_span_4,
    grid(2, 2)
        .config(|cfg|{
            cfg.set_row_span((0, 0), 2);
            cfg.set_row_span((0, 1), 2);
        })
        .build(),
    "+---+---+"
    "+0-0+0-1+"
    "+---+---+"
);

test_table!(
    _4x4_with_row_span_and_col_span_0,
    grid(4, 4)
        .change_cell((1, 1), "123\n345\n555\n333")
        .config(|cfg|{
            cfg.set_row_span((1, 1), 2);
            cfg.set_column_span((1, 1), 2);
            cfg.set_alignment_horizontal(Cell(1, 1), AlignmentHorizontal::Center);
            cfg.set_alignment_vertical(Cell(1, 1), AlignmentVertical::Center);
        })
        .build(),
    "+---+---+---+---+"
    "|0-0|0-1|0-2|0-3|"
    "+---+---+---+---+"
    "|1-0|  123  |1-3|"
    "|   |  345  |   |"
    "+---+  555  +---+"
    "|2-0|  333  |2-3|"
    "+---+---+---+---+"
    "|3-0|3-1|3-2|3-3|"
    "+---+---+---+---+"
);

test_table!(
    _4x4_with_row_span_and_col_span_1,
    grid(4, 4)
        .change_cell((0, 0), "123\n345\n555\n333")
        .config(|cfg|{
            cfg.set_row_span((0, 0), 2);
            cfg.set_column_span((0, 0), 2);
            cfg.set_alignment_horizontal(Cell(0, 0), AlignmentHorizontal::Center);
            cfg.set_alignment_vertical(Cell(0, 0), AlignmentVertical::Center);
        })
        .build(),
    "+---+---+---+---+"
    "|  123  |0-2|0-3|"
    "|  345  |   |   |"
    "+  555  +---+---+"
    "|  333  |1-2|1-3|"
    "+---+---+---+---+"
    "|2-0|2-1|2-2|2-3|"
    "+---+---+---+---+"
    "|3-0|3-1|3-2|3-3|"
    "+---+---+---+---+"
);

test_table!(
    _4x4_with_row_span_and_col_span_2,
    grid(4, 4)
        .change_cell((2, 0), "123\n345\n555\n333")
        .config(|cfg|{
            cfg.set_row_span((2, 0), 2);
            cfg.set_column_span((2, 0), 2);
            cfg.set_alignment_horizontal(Cell(2, 0), AlignmentHorizontal::Center);
            cfg.set_alignment_vertical(Cell(2, 0), AlignmentVertical::Center);
        })
        .build(),
    "+---+---+---+---+"
    "|0-0|0-1|0-2|0-3|"
    "+---+---+---+---+"
    "|1-0|1-1|1-2|1-3|"
    "+---+---+---+---+"
    "|  123  |2-2|2-3|"
    "|  345  |   |   |"
    "+  555  +---+---+"
    "|  333  |3-2|3-3|"
    "+---+---+---+---+"
);

test_table!(
    _4x4_with_row_span_and_col_span_3,
    grid(4, 4)
        .change_cell((2, 2), "123\n345\n555\n333")
        .config(|cfg|{
            cfg.set_row_span((2, 2), 2);
            cfg.set_column_span((2, 2), 2);
            cfg.set_alignment_horizontal(Cell(2, 2), AlignmentHorizontal::Center);
            cfg.set_alignment_vertical(Cell(2, 2), AlignmentVertical::Center);
        })
        .build(),
    "+---+---+---+---+"
    "|0-0|0-1|0-2|0-3|"
    "+---+---+---+---+"
    "|1-0|1-1|1-2|1-3|"
    "+---+---+---+---+"
    "|2-0|2-1|  123  |"
    "|   |   |  345  |"
    "+---+---+  555  +"
    "|3-0|3-1|  333  |"
    "+---+---+---+---+"
);

test_table!(
    _4x4_with_row_span_and_col_span_4,
    grid(4, 4)
        .change_cell((0, 2), "123\n345\n555\n333")
        .config(|cfg|{
            cfg.set_row_span((0, 2), 2);
            cfg.set_column_span((0, 2), 2);
            cfg.set_alignment_horizontal(Cell(0, 2), AlignmentHorizontal::Center);
            cfg.set_alignment_vertical(Cell(0, 2), AlignmentVertical::Center);
        })
        .build(),
    "+---+---+---+---+"
    "|0-0|0-1|  123  |"
    "|   |   |  345  |"
    "+---+---+  555  +"
    "|1-0|1-1|  333  |"
    "+---+---+---+---+"
    "|2-0|2-1|2-2|2-3|"
    "+---+---+---+---+"
    "|3-0|3-1|3-2|3-3|"
    "+---+---+---+---+"
);

test_table!(
    _4x4_with_row_span_and_col_span_5,
    grid(4, 4)
        .change_cell((0, 1), "123\n345\n555\n333")
        .config(|cfg|{
            cfg.set_row_span((0, 1), 2);
            cfg.set_column_span((0, 1), 2);
            cfg.set_alignment_horizontal(Cell(0, 1), AlignmentHorizontal::Center);
            cfg.set_alignment_vertical(Cell(0, 1), AlignmentVertical::Center);
        })
        .build(),
    "+---+---+---+---+"
    "|0-0|  123  |0-3|"
    "|   |  345  |   |"
    "+---+  555  +---+"
    "|1-0|  333  |1-3|"
    "+---+---+---+---+"
    "|2-0|2-1|2-2|2-3|"
    "+---+---+---+---+"
    "|3-0|3-1|3-2|3-3|"
    "+---+---+---+---+"
);

test_table!(
    _4x4_with_row_span_and_col_span_6,
    grid(4, 4)
        .change_cell((1, 1), "123\n345\n555\n333")
        .config(|cfg|{
            cfg.set_row_span((1, 1), 3);
            cfg.set_column_span((1, 1), 3);
            cfg.set_alignment_horizontal(Cell(1, 1), AlignmentHorizontal::Center);
            cfg.set_alignment_vertical(Cell(1, 1), AlignmentVertical::Center);
        })
        .build(),
    "+---+---+---+---+"
    "|0-0|0-1|0-2|0-3|"
    "+---+---+---+---+"
    "|1-0|    123    |"
    "+---+    345    +"
    "|2-0|    555    |"
    "+---+    333    +"
    "|3-0|           |"
    "+---+---+---+---+"
);

test_table!(
    _4x4_with_row_span_and_col_span_7,
    grid(4, 4)
        .change_cell((0, 0), "123\n345\n555\n333")
        .config(|cfg|{
            cfg.set_row_span((0, 0), 3);
            cfg.set_column_span((0, 0), 3);
            cfg.set_alignment_horizontal(Cell(0, 0), AlignmentHorizontal::Center);
            cfg.set_alignment_vertical(Cell(0, 0), AlignmentVertical::Center);
        })
        .build(),
    "+---+---+---+---+"
    "|    123    |0-3|"
    "+    345    +---+"
    "|    555    |1-3|"
    "+    333    +---+"
    "|           |2-3|"
    "+---+---+---+---+"
    "|3-0|3-1|3-2|3-3|"
    "+---+---+---+---+"
);

test_table!(
    _4x4_with_row_span_and_col_span_8,
    grid(4, 4)
        .change_cell((0, 1), "123\n345\n555\n333")
        .config(|cfg|{
            cfg.set_row_span((0, 1), 3);
            cfg.set_column_span((0, 1), 3);
            cfg.set_alignment_horizontal(Cell(0, 1), AlignmentHorizontal::Center);
            cfg.set_alignment_vertical(Cell(0, 1), AlignmentVertical::Center);
        })
        .build(),
    "+---+---+---+---+"
    "|0-0|    123    |"
    "+---+    345    +"
    "|1-0|    555    |"
    "+---+    333    +"
    "|2-0|           |"
    "+---+---+---+---+"
    "|3-0|3-1|3-2|3-3|"
    "+---+---+---+---+"
);

test_table!(
    _4x4_with_row_span_and_col_span_9,
    grid(4, 4)
        .change_cell((1, 0), "123\n345\n555\n333")
        .config(|cfg|{
            cfg.set_row_span((1, 0), 3);
            cfg.set_column_span((1, 0), 3);
            cfg.set_alignment_horizontal(Cell(1, 0), AlignmentHorizontal::Center);
            cfg.set_alignment_vertical(Cell(1, 0), AlignmentVertical::Center);
        })
        .build(),
    "+---+---+---+---+"
    "|0-0|0-1|0-2|0-3|"
    "+---+---+---+---+"
    "|    123    |1-3|"
    "+    345    +---+"
    "|    555    |2-3|"
    "+    333    +---+"
    "|           |3-3|"
    "+---+---+---+---+"
);

test_table!(
    _4x4_with_row_span_and_col_span_10,
    grid(4, 4)
        .change_cell((0, 0), "hello\nworld\n!\n!\n!\n!")
        .config(|cfg|{
            cfg.set_column_span((1, 1), 2);
            cfg.set_column_span((3, 0), 3);
            cfg.set_row_span((0, 0), 2);
            cfg.set_row_span((0, 3), 3);
        })
        .build(),
    "+-----+---+---+---+"
    "|hello|0-1|0-2|0-3|"
    "|world|   |   |   |"
    "|!    |   |   |   |"
    "+!    +---+---+   +"
    "|!    |1-1    |   |"
    "|!    |       |   |"
    "+-----+---+---+   +"
    "|2-0  |2-1|2-2|   |"
    "+-----+---+---+---+"
    "|3-0          |3-3|"
    "+-----+---+---+---+"
);

test_table!(
    _4x4_with_row_span_and_col_span_11,
    grid(4, 4)
        .change_cell((0, 2), "q\nw\ne\nr\nt")
        .change_cell((0, 3), "q1\nw1\ne1\nr1\nt1")
        .config(|cfg|{
            cfg.set_column_span((0, 0), 2);
            cfg.set_row_span((0, 2), 2);
            cfg.set_row_span((0, 3), 3);
        })
        .build(),
    "+---+---+---+---+"
    "|0-0    |q  |q1 |"
    "|       |w  |w1 |"
    "+---+---+e  +e1 +"
    "|1-0|1-1|r  |r1 |"
    "|   |   |t  |t1 |"
    "+---+---+---+   +"
    "|2-0|2-1|2-2|   |"
    "+---+---+---+---+"
    "|3-0|3-1|3-2|3-3|"
    "+---+---+---+---+"
);

test_table!(
    _3x5_with_row_span_and_col_span_12,
    grid(3, 5)
        .change_cell((0, 3), "q\nw\ne\nr\nt")
        .change_cell((0, 4), "q1\nw1\ne1\nr1\nt1")
        .config(|cfg|{
            cfg.set_column_span((0, 0), 2);
            cfg.set_row_span((0, 3), 2);
            cfg.set_row_span((0, 4), 3);
        })
        .build(),
    "+---+---+---+---+--+"
    "|0-0    |0-2|q  |q1|"
    "|       |   |w  |w1|"
    "+---+---+---+e  +e1+"
    "|1-0|1-1|1-2|r  |r1|"
    "|   |   |   |t  |t1|"
    "+---+---+---+---+  +"
    "|2-0|2-1|2-2|2-3|  |"
    "+---+---+---+---+--+"
);

test_table!(
    _3x5_with_row_span_and_col_span_13,
    grid(3, 5)
        .change_cell((0, 3), "q\nw\ne\nr\nt\n")
        .change_cell((0, 4), "q1\nw1\ne1\nr1\nt1\n")
        .config(|cfg|{
            cfg.set_column_span((0, 0), 2);
            cfg.set_row_span((0, 3), 2);
            cfg.set_row_span((0, 4), 3);
        })
        .build(),
    "+---+---+---+---+--+"
    "|0-0    |0-2|q  |q1|"
    "|       |   |w  |w1|"
    "|       |   |e  |e1|"
    "+---+---+---+r  +r1+"
    "|1-0|1-1|1-2|t  |t1|"
    "|   |   |   |   |  |"
    "+---+---+---+---+  +"
    "|2-0|2-1|2-2|2-3|  |"
    "+---+---+---+---+--+"
);

test_table!(
    _3x5_with_row_span_and_col_span_14,
    grid(3, 5)
        .change_cell((0, 3), "q\nw\ne\nr\nt\n")
        .change_cell((0, 4), "q1\nw1\ne1\nr1\nt1\n")
        .config(|cfg|{
            cfg.set_column_span((0, 0), 2);
            cfg.set_row_span((0, 3), 2);
            cfg.set_row_span((0, 4), 3);
            cfg.set_padding(
                Cell(0, 0),
                Sides::new(Indent::new(2, ' '), Indent::new(2, ' '), Indent::new(2, ' '), Indent::new(2, ' '))
            );
            cfg.set_padding(
                Cell(0, 3),
                Sides::new(Indent::new(2, ' '), Indent::new(2, ' '), Indent::new(2, ' '), Indent::new(2, ' '))
            );
            cfg.set_padding(
                Cell(1, 2),
                Sides::new(Indent::new(2, ' '), Indent::new(2, ' '), Indent::new(4, ' '), Indent::new(2, ' '))
            );
        })
        .build(),
    "+---+---+-------+-----+--+"
    "|       |0-2    |     |q1|"
    "|       |       |     |w1|"
    "|  0-0  |       |  q  |e1|"
    "|       |       |  w  |r1|"
    "|       |       |  e  |t1|"
    "+---+---+-------+  r  +  +"
    "|1-0|1-1|       |  t  |  |"
    "|   |   |       |     |  |"
    "|   |   |       |     |  |"
    "|   |   |       |     |  |"
    "|   |   |  1-2  |     |  |"
    "|   |   |       |     |  |"
    "|   |   |       |     |  |"
    "+---+---+-------+-----+  +"
    "|2-0|2-1|2-2    |2-3  |  |"
    "+---+---+-------+-----+--+"
);

// is this correct?
test_table!(
    _3x4_with_row_span_and_col_span_13,
    grid(3, 5)
        .config(|cfg|{
            cfg.set_column_span((0, 0), 2);
            cfg.set_column_span((1, 0), 2);
            cfg.set_row_span((1, 0), 2);
        })
        .build(),
    "+-+-+---+---+---+"
    "|0-0|0-2|0-3|0-4|"
    "+-+-+---+---+---+"
    "|1-0|1-2|1-3|1-4|"
    "+   +---+---+---+"
    "|   |2-2|2-3|2-4|"
    "+-+-+---+---+---+"
);

test_table!(
    _5x2_render_0,
    grid(5, 2)
        .change_cell((1, 1), "1\n2\n3\n4")
        .config(|cfg|{
            cfg.set_row_span((1, 1), 4);
        })
        .build(),
    "+---+---+"
    "|0-0|0-1|"
    "+---+---+"
    "|1-0|1  |"
    "+---+2  +"
    "|2-0|3  |"
    "+---+4  +"
    "|3-0|   |"
    "+---+   +"
    "|4-0|   |"
    "+---+---+"
);

test_table!(
    _3x4_column_span_0,
    grid(3, 4)
        .config(|cfg|{
            cfg.set_column_span((0, 0), 2);
            cfg.set_column_span((1, 0), 2);
            cfg.set_column_span((2, 0), 2);
        })
        .build(),
    "+-+-+---+---+"
    "|0-0|0-2|0-3|"
    "+-+-+---+---+"
    "|1-0|1-2|1-3|"
    "+-+-+---+---+"
    "|2-0|2-2|2-3|"
    "+-+-+---+---+"
);

test_table!(
    _3x4_column_span_1,
    grid(3, 4)
        .config(|cfg|{
            cfg.set_column_span((0, 0), 3);
            cfg.set_column_span((1, 0), 3);
            cfg.set_column_span((2, 0), 3);
        })
        .build(),
    "+-+++---+"
    "|0-0|0-3|"
    "+-+++---+"
    "|1-0|1-3|"
    "+-+++---+"
    "|2-0|2-3|"
    "+-+++---+"
);

test_table!(
    _3x4_column_span_2,
    grid(3, 4)
        .change_cell((0, 0), "")
        .change_cell((1, 0), "")
        .change_cell((2, 0), "")
        .config(|cfg|{
            cfg.set_column_span((0, 0), 3);
            cfg.set_column_span((1, 0), 3);
            cfg.set_column_span((2, 0), 3);
        })
        .build(),
    "++++---+"
    "|  |0-3|"
    "++++---+"
    "|  |1-3|"
    "++++---+"
    "|  |2-3|"
    "++++---+"
);

// #[test]
// #[ignore = "todo; create some logic of combining spans? or somehow resolving to not get the following"]
// fn render_grid_with_row_3() {
//     let mut grid = util::new_grid::<3, 5>();

//     grid.set(Entity::Cell(0, 0), Settings::new().span(2));
//     grid.set(Entity::Cell(0, 1), Settings::new().span(2));
//     grid.set(Entity::Cell(0, 2), Settings::new().span(2));

//     assert_eq!(
//         grid.to_string(),
//         concat!(
//             "+---+---+---+---+--+\n",
//             "|0-0    |0-2|q  |q1|\n",
//             "|       |   |w  |w1|\n",
//             "+---+---+---+e  +e1+\n",
//             "|1-0|1-1|1-2|r  |r1|\n",
//             "|   |   |   |t  |t1|\n",
//             "+---+---+---+---+  +\n",
//             "|2-0|2-1|2-2|2-3|  |\n",
//             "+---+---+---+---+--+\n",
//         )
//     );
// }
