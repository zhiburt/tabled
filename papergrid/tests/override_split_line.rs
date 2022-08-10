mod util;

use util::{grid, test_table};

test_table!(
    override_top_test_0,
    grid(2, 2).config(|cfg| cfg.override_split_line(0, "T")).build(),
    "T---+---+"
    "|0-0|0-1|"
    "+---+---+"
    "|1-0|1-1|"
    "+---+---+"
);

test_table!(
    override_top_test_1,
    grid(2, 2).config(|cfg| cfg.override_split_line(0, " Tab")).build(),
    " Tab+---+"
    "|0-0|0-1|"
    "+---+---+"
    "|1-0|1-1|"
    "+---+---+"
);

test_table!(
    override_top_test_2,
    grid(2, 2).config(|cfg| cfg.override_split_line(0, "Table")).build(),
    "Table---+"
    "|0-0|0-1|"
    "+---+---+"
    "|1-0|1-1|"
    "+---+---+"
);

test_table!(
    override_top_test_3,
    grid(2, 2).config(|cfg| cfg.override_split_line(0, "Table T")).build(),
    "Table T-+"
    "|0-0|0-1|"
    "+---+---+"
    "|1-0|1-1|"
    "+---+---+"
);

test_table!(
    override_top_test_4,
    grid(2, 2).config(|cfg| cfg.override_split_line(0, "Table TES")).build(),
    "Table TES"
    "|0-0|0-1|"
    "+---+---+"
    "|1-0|1-1|"
    "+---+---+"
);

test_table!(
    override_top_test_5,
    grid(2, 2).config(|cfg| cfg.override_split_line(0, "Table LONGER THEN LINE")).build(),
    "Table LON"
    "|0-0|0-1|"
    "+---+---+"
    "|1-0|1-1|"
    "+---+---+"
);

test_table!(
    override_bottom_test_0,
    grid(2, 2).config(|cfg| cfg.override_split_line(2, "T")).build(),
    "+---+---+"
    "|0-0|0-1|"
    "+---+---+"
    "|1-0|1-1|"
    "T---+---+"
);

test_table!(
    override_bottom_test_1,
    grid(2, 2).config(|cfg| cfg.override_split_line(2, " Tab")).build(),
    "+---+---+"
    "|0-0|0-1|"
    "+---+---+"
    "|1-0|1-1|"
    " Tab+---+"
);

test_table!(
    override_bottom_test_2,
    grid(2, 2).config(|cfg| cfg.override_split_line(2, "Table")).build(),
    "+---+---+"
    "|0-0|0-1|"
    "+---+---+"
    "|1-0|1-1|"
    "Table---+"
);

test_table!(
    override_bottom_test_3,
    grid(2, 2).config(|cfg| cfg.override_split_line(2, "Table T")).build(),
    "+---+---+"
    "|0-0|0-1|"
    "+---+---+"
    "|1-0|1-1|"
    "Table T-+"
);

test_table!(
    override_bottom_test_4,
    grid(2, 2).config(|cfg| cfg.override_split_line(2, "Table TES")).build(),
    "+---+---+"
    "|0-0|0-1|"
    "+---+---+"
    "|1-0|1-1|"
    "Table TES"
);

test_table!(
    override_bottom_test_5,
    grid(2, 2).config(|cfg| cfg.override_split_line(2, "Table LONGER THEN LINE")).build(),
    "+---+---+"
    "|0-0|0-1|"
    "+---+---+"
    "|1-0|1-1|"
    "Table LON"
);
