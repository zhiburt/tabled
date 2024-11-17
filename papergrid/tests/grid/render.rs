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

#![cfg(feature = "std")]

use std::vec;

use papergrid::{
    colors::NoColors,
    config::spanned::SpannedConfig,
    config::{AlignmentHorizontal, AlignmentVertical, Borders, Entity},
    grid::iterable::Grid,
    records::IterRecords,
};

use crate::util::{grid, ConstantDimension, DEFAULT_BORDERS};
use testing_table::test_table;

test_table!(render_0x0, grid(0, 0).build(), "");

test_table!(
    render_1x1,
    grid(1, 1).change_cell((0, 0), "one line").build(),
    "+--------+"
    "|one line|"
    "+--------+"
);

test_table!(
    render_1x1_empty,
    grid(1, 1).change_cell((0, 0), "").build(),
    "++"
    "||"
    "++"
);

test_table!(
    render_1x1_empty_with_height_0,
    {
        let data = vec![vec![""]];
        let data = IterRecords::new(data, 1, Some(1));

        let dims = ConstantDimension(vec![0], vec![0]);

        let mut cfg = SpannedConfig::default();
        cfg.set_borders(DEFAULT_BORDERS);

        let grid = Grid::new(&data, &dims, &cfg, NoColors);
        grid.to_string()
    },
    "++"
    "++"
);

test_table!(
    render_1x1_empty_with_height_with_width,
    {
        let data = vec![vec![String::from("")]];
        let data = IterRecords::new(&data, 1, Some(1));

        let dims = ConstantDimension(vec![10], vec![0]);
        let mut cfg = SpannedConfig::default();
        cfg.set_borders(Borders {
            top_left: Some('┌'),
            top_right: Some('┐'),
            bottom_left: Some('└'),
            bottom_right: Some('┘'),
            top: Some('─'),
            bottom: Some('─'),
            ..Default::default()
        });

        let grid = Grid::new(data, &dims, &cfg, NoColors);
        grid.to_string()
    },
    "┌──────────┐"
    "└──────────┘"
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
    hieroglyph_multiline_handling,
    grid(1, 2).data([["哈哈", "哈\n哈"]]).build(),
    "+----+--+"
    "|哈哈|哈|"
    "|    |哈|"
    "+----+--+"
);

#[test]
fn emoji_width_test() {
    use papergrid::util::string::get_string_width;
    assert_eq!(get_string_width("👩"), 2);
    assert_eq!(get_string_width("🔬"), 2);
    assert_eq!(get_string_width("👩\u{200D}🔬"), 2);
}

test_table!(
    emoji_handling,
    grid(2, 1).data([["👩👩👩👩👩👩"], ["Hello"]]).build(),
    "+------------+"
    "|👩👩👩👩👩👩|"
    "+------------+"
    "|Hello       |"
    "+------------+"
);

test_table!(
    emoji_handling_2,
    grid(2, 1).data([["👩\u{200D}🔬👩\u{200D}🔬👩\u{200D}🔬👩\u{200D}🔬👩\u{200D}🔬👩\u{200D}🔬"], ["Hello"]]).build(),
    "+------------+"
    "|👩\u{200D}🔬👩\u{200D}🔬👩\u{200D}🔬👩\u{200D}🔬👩\u{200D}🔬👩\u{200D}🔬|"
    "+------------+"
    "|Hello       |"
    "+------------+"
);

test_table!(
    doesnt_render_return_carige_0,
    grid(2, 2).change_cell((0, 1), "123\r\r\r567").build(),
    "+---+---------+"
    "|0-0|123\r\r\r567|"
    "+---+---------+"
    "|1-0|1-1      |"
    "+---+---------+"
);

test_table!(
    doesnt_render_return_carige_1,
    grid(2, 2).change_cell((1, 1), "12345678").change_cell((0, 1), "123\r\r\r567").build(),
    "+---+---------+"
    "|0-0|123\r\r\r567|"
    "+---+---------+"
    "|1-0|12345678 |"
    "+---+---------+"
);

// #[test]
// #[ignore = "I am not sure what is the right behaviour here"]
// fn hieroglyph_handling() {
//     let grid = util::grid_from([["哈哈", "哈"]]);

//     assert_eq!(
//         grid,
//         "+----+--+\n\
//          |哈哈  |哈 |\n\
//          +----+--+",
//     )
// }
