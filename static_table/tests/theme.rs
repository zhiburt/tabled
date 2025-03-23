use static_table::static_table;

use tabled::assert::test_table;

test_table!(
    static_table_with_theme_rounded,
    static_table!([[1, 2, 123], [1, 2, 123], [1, 2, 123]], THEME = "ROUNDED"),
    "╭───┬───┬─────╮"
    "│ 1 │ 2 │ 123 │"
    "├───┼───┼─────┤"
    "│ 1 │ 2 │ 123 │"
    "│ 1 │ 2 │ 123 │"
    "╰───┴───┴─────╯"
);

test_table!(
    static_table_with_theme_modern,
    static_table!([[1, 2, 123], [1, 2, 123], [1, 2, 123]], THEME = "MODERN"),
    "┌───┬───┬─────┐"
    "│ 1 │ 2 │ 123 │"
    "├───┼───┼─────┤"
    "│ 1 │ 2 │ 123 │"
    "├───┼───┼─────┤"
    "│ 1 │ 2 │ 123 │"
    "└───┴───┴─────┘"
);

test_table!(
    static_table_with_theme_extended,
    static_table!([[1, 2, 123], [1, 2, 123], [1, 2, 123]], THEME = "EXTENDED"),
    "╔═══╦═══╦═════╗"
    "║ 1 ║ 2 ║ 123 ║"
    "╠═══╬═══╬═════╣"
    "║ 1 ║ 2 ║ 123 ║"
    "╠═══╬═══╬═════╣"
    "║ 1 ║ 2 ║ 123 ║"
    "╚═══╩═══╩═════╝"
);

test_table!(
    static_table_with_theme_sharp,
    static_table!([[1, 2, 123], [1, 2, 123], [1, 2, 123]], THEME = "SHARP"),
    "┌───┬───┬─────┐"
    "│ 1 │ 2 │ 123 │"
    "├───┼───┼─────┤"
    "│ 1 │ 2 │ 123 │"
    "│ 1 │ 2 │ 123 │"
    "└───┴───┴─────┘"
);

test_table!(
    static_table_with_theme_ascii,
    static_table!([[1, 2, 123], [1, 2, 123], [1, 2, 123]], THEME = "ASCII"),
    "+---+---+-----+"
    "| 1 | 2 | 123 |"
    "+---+---+-----+"
    "| 1 | 2 | 123 |"
    "+---+---+-----+"
    "| 1 | 2 | 123 |"
    "+---+---+-----+"
);

test_table!(
    static_table_with_theme_ascii_rounded,
    static_table!([[1, 2, 123], [1, 2, 123], [1, 2, 123]], THEME = "ASCII_ROUNDED"),
    ".-------------."
    "| 1 | 2 | 123 |"
    "| 1 | 2 | 123 |"
    "| 1 | 2 | 123 |"
    "'-------------'"
);

test_table!(
    static_table_with_theme_ascii_dots,
    static_table!([[1, 2, 123], [1, 2, 123], [1, 2, 123]], THEME = "DOTS"),
    "..............."
    ": 1 : 2 : 123 :"
    ":...:...:.....:"
    ": 1 : 2 : 123 :"
    ":...:...:.....:"
    ": 1 : 2 : 123 :"
    ":...:...:.....:"
);

test_table!(
    static_table_with_theme_psql,
    static_table!([[1, 2, 123], [1, 2, 123], [1, 2, 123]], THEME = "PSQL"),
    " 1 | 2 | 123 "
    "---+---+-----"
    " 1 | 2 | 123 "
    " 1 | 2 | 123 "
);

test_table!(
    static_table_with_theme_markdown,
    static_table!([[1, 2, 123], [1, 2, 123], [1, 2, 123]], THEME = "MARKDOWN"),
    "| 1 | 2 | 123 |"
    "|---|---|-----|"
    "| 1 | 2 | 123 |"
    "| 1 | 2 | 123 |"
);

test_table!(
    static_table_with_theme_re_structured_text,
    static_table!([[1, 2, 123], [1, 2, 123], [1, 2, 123]], THEME = "RE_STRUCTURED_TEXT"),
    "=== === ====="
    " 1   2   123 "
    "=== === ====="
    " 1   2   123 "
    " 1   2   123 "
    "=== === ====="
);

test_table!(
    static_table_with_theme_blank,
    static_table!([[1, 2, 123], [1, 2, 123], [1, 2, 123]], THEME = "BLANK"),
    " 1   2   123 "
    " 1   2   123 "
    " 1   2   123 "
);

test_table!(
    static_table_with_theme_empty,
    static_table!([[1, 2, 123], [1, 2, 123], [1, 2, 123]], THEME = "EMPTY"),
    " 1  2  123 "
    " 1  2  123 "
    " 1  2  123 "
);
