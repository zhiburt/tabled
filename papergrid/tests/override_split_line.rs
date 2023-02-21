#![cfg(feature = "std")]

mod util;

use papergrid::grid::spanned::config::Offset;
use util::{grid, test_table, DEFAULT_BORDERS};

test_table!(
    override_top_test_0,
    grid(2, 2).config(|cfg| cfg.override_split_line(0, "T", Offset::Begin(0))).build(),
    "T---+---+"
    "|0-0|0-1|"
    "+---+---+"
    "|1-0|1-1|"
    "+---+---+"
);

test_table!(
    override_top_test_1,
    grid(2, 2).config(|cfg| cfg.override_split_line(0, " Tab", Offset::Begin(0))).build(),
    " Tab+---+"
    "|0-0|0-1|"
    "+---+---+"
    "|1-0|1-1|"
    "+---+---+"
);

test_table!(
    override_top_test_2,
    grid(2, 2).config(|cfg| cfg.override_split_line(0, "Table", Offset::Begin(0))).build(),
    "Table---+"
    "|0-0|0-1|"
    "+---+---+"
    "|1-0|1-1|"
    "+---+---+"
);

test_table!(
    override_top_test_3,
    grid(2, 2).config(|cfg| cfg.override_split_line(0, "Table T", Offset::Begin(0))).build(),
    "Table T-+"
    "|0-0|0-1|"
    "+---+---+"
    "|1-0|1-1|"
    "+---+---+"
);

test_table!(
    override_top_test_4,
    grid(2, 2).config(|cfg| cfg.override_split_line(0, "Table TES", Offset::Begin(0))).build(),
    "Table TES"
    "|0-0|0-1|"
    "+---+---+"
    "|1-0|1-1|"
    "+---+---+"
);

test_table!(
    override_top_test_5,
    grid(2, 2).config(|cfg| cfg.override_split_line(0, "Table LONGER THEN LINE", Offset::Begin(0))).build(),
    "Table LON"
    "|0-0|0-1|"
    "+---+---+"
    "|1-0|1-1|"
    "+---+---+"
);

test_table!(
    override_bottom_test_0,
    grid(2, 2).config(|cfg| cfg.override_split_line(2, "T", Offset::Begin(0))).build(),
    "+---+---+"
    "|0-0|0-1|"
    "+---+---+"
    "|1-0|1-1|"
    "T---+---+"
);

test_table!(
    override_bottom_test_1,
    grid(2, 2).config(|cfg| cfg.override_split_line(2, " Tab", Offset::Begin(0))).build(),
    "+---+---+"
    "|0-0|0-1|"
    "+---+---+"
    "|1-0|1-1|"
    " Tab+---+"
);

test_table!(
    override_bottom_test_2,
    grid(2, 2).config(|cfg| cfg.override_split_line(2, "Table", Offset::Begin(0))).build(),
    "+---+---+"
    "|0-0|0-1|"
    "+---+---+"
    "|1-0|1-1|"
    "Table---+"
);

test_table!(
    override_bottom_test_3,
    grid(2, 2).config(|cfg| cfg.override_split_line(2, "Table T", Offset::Begin(0))).build(),
    "+---+---+"
    "|0-0|0-1|"
    "+---+---+"
    "|1-0|1-1|"
    "Table T-+"
);

test_table!(
    override_bottom_test_4,
    grid(2, 2).config(|cfg| cfg.override_split_line(2, "Table TES", Offset::Begin(0))).build(),
    "+---+---+"
    "|0-0|0-1|"
    "+---+---+"
    "|1-0|1-1|"
    "Table TES"
);

test_table!(
    override_bottom_test_5,
    grid(2, 2).config(|cfg| cfg.override_split_line(2, "Table LONGER THEN LINE", Offset::Begin(0))).build(),
    "+---+---+"
    "|0-0|0-1|"
    "+---+---+"
    "|1-0|1-1|"
    "Table LON"
);

test_table!(
    line_is_not_showed_if_there_no_border_line_0,
    grid(2, 2).config(|cfg| {
        let mut borders = DEFAULT_BORDERS;
        borders.bottom = None;
        borders.bottom_intersection = None;
        borders.bottom_left = None;
        borders.bottom_right = None;
        cfg.set_borders(borders);
        cfg.override_split_line(2, "Table LONGER THEN LINE", Offset::Begin(0));
    }).build(),
    "+---+---+"
    "|0-0|0-1|"
    "+---+---+"
    "|1-0|1-1|"
);

test_table!(
    string_which_starts_from_emojie,
    grid(2, 2).config(|cfg| {
        cfg.override_split_line(2, "🇻🇬🇻🇬🇻🇬🇻🇬🇻🇬🇻🇬🇻🇬", Offset::Begin(0));
    }).build(),
    "+---+---+"
    "|0-0|0-1|"
    "+---+---+"
    "|1-0|1-1|"
    "🇻🇬🇻🇬🇻🇬🇻🇬🇻"
);

test_table!(
    offset_test_1,
    grid(2, 2).config(|cfg| cfg.override_split_line(0, "T", Offset::Begin(1))).build(),
    "+T--+---+"
    "|0-0|0-1|"
    "+---+---+"
    "|1-0|1-1|"
    "+---+---+"
);

test_table!(
    offset_test_2,
    grid(2, 2).config(|cfg| cfg.override_split_line(0, "T", Offset::Begin(4))).build(),
    "+---T---+"
    "|0-0|0-1|"
    "+---+---+"
    "|1-0|1-1|"
    "+---+---+"
);

test_table!(
    offset_test_3,
    grid(2, 2).config(|cfg| cfg.override_split_line(0, "T", Offset::Begin(8))).build(),
    "+---+---T"
    "|0-0|0-1|"
    "+---+---+"
    "|1-0|1-1|"
    "+---+---+"
);

test_table!(
    offset_test_4,
    grid(2, 2).config(|cfg| cfg.override_split_line(0, "T", Offset::Begin(100))).build(),
    "+---+---+"
    "|0-0|0-1|"
    "+---+---+"
    "|1-0|1-1|"
    "+---+---+"
);

test_table!(
    offset_end_test_1,
    grid(2, 2).config(|cfg| cfg.override_split_line(0, "Test", Offset::End(0))).build(),
    "+---+---+"
    "|0-0|0-1|"
    "+---+---+"
    "|1-0|1-1|"
    "+---+---+"
);

test_table!(
    offset_end_test_2,
    grid(2, 2).config(|cfg| cfg.override_split_line(0, "Test", Offset::End(1))).build(),
    "+---+---T"
    "|0-0|0-1|"
    "+---+---+"
    "|1-0|1-1|"
    "+---+---+"
);

test_table!(
    offset_end_test_3,
    grid(2, 2).config(|cfg| cfg.override_split_line(0, "Test", Offset::End(9))).build(),
    "Test+---+"
    "|0-0|0-1|"
    "+---+---+"
    "|1-0|1-1|"
    "+---+---+"
);

test_table!(
    offset_end_test_4,
    grid(2, 2).config(|cfg| cfg.override_split_line(0, "T", Offset::End(100))).build(),
    "+---+---+"
    "|0-0|0-1|"
    "+---+---+"
    "|1-0|1-1|"
    "+---+---+"
);
