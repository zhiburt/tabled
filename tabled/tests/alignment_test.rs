#![cfg(feature = "std")]

use tabled::settings::{
    locator::ByColumnName,
    object::{Columns, Rows, Segment},
    Alignment, Modify, Padding, Style,
};

use crate::util::{create_table, init_table, test_table};

mod util;

test_table!(
    full_alignment,
    create_table::<3, 3>().with(Style::psql()).with(Modify::new(Segment::all()).with(Alignment::left())),
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    " 0 | 0-0      | 0-1      | 0-2      "
    " 1 | 1-0      | 1-1      | 1-2      "
    " 2 | 2-0      | 2-1      | 2-2      "
);

test_table!(
    head_and_data_alignment,
    create_table::<3, 3>()
        .with(Modify::new(Rows::first()).with(Alignment::left()))
        .with(Modify::new(Rows::new(1..)).with(Alignment::right())),
    "+---+----------+----------+----------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| 0 |      0-0 |      0-1 |      0-2 |"
    "+---+----------+----------+----------+"
    "| 1 |      1-0 |      1-1 |      1-2 |"
    "+---+----------+----------+----------+"
    "| 2 |      2-0 |      2-1 |      2-2 |"
    "+---+----------+----------+----------+"
);

test_table!(
    full_alignment_multiline,
    init_table::<3, 3, _, _>([((2, 2), "https://\nwww\n.\nredhat\n.com\n/en")])
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Alignment::left())),
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    " 0 | 0-0      | 0-1      | 0-2      "
    " 1 | 1-0      | 1-1      | 1-2      "
    " 2 | 2-0      | https:// | 2-2      "
    "   |          | www      |          "
    "   |          | .        |          "
    "   |          | redhat   |          "
    "   |          | .com     |          "
    "   |          | /en      |          "
);

test_table!(
    vertical_alignment_test,
    init_table::<3, 3, _, _>([
        ((1, 2), "E\nnde\navou\nros"),
        ((2, 2), "Red\nHat"),
        ((2, 3), "https://\nwww\n.\nredhat\n.com\n/en"),
    ])
    .with(Style::psql())
    .with(Modify::new(Columns::new(1..)).with(Alignment::bottom())),
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    " 0 |   0-0    |   0-1    |   0-2    "
    " 1 |          |   E      |          "
    "   |          |   nde    |          "
    "   |          |   avou   |          "
    "   |   1-0    |   ros    |   1-2    "
    " 2 |          |          | https:// "
    "   |          |          | www      "
    "   |          |          | .        "
    "   |          |          | redhat   "
    "   |          |   Red    | .com     "
    "   |   2-0    |   Hat    | /en      "
);

test_table!(
    alignment_doesnt_change_padding,
    create_table::<3, 3>()
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Padding::new(3, 0, 0, 0)))
        .with(Modify::new(Segment::all()).with(Alignment::left())),
    "   N|   column 0|   column 1|   column 2"
    "----+-----------+-----------+-----------"
    "   0|   0-0     |   0-1     |   0-2     "
    "   1|   1-0     |   1-1     |   1-2     "
    "   2|   2-0     |   2-1     |   2-2     "
);

test_table!(
    alignment_global,
    create_table::<3, 3>().with(Style::psql()).with(Alignment::right()),
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    " 0 |      0-0 |      0-1 |      0-2 "
    " 1 |      1-0 |      1-1 |      1-2 "
    " 2 |      2-0 |      2-1 |      2-2 "
);

test_table!(
    padding_by_column_name,
    create_table::<3, 3>()
        .with(Style::psql())
        .with(Modify::new(ByColumnName::new("column 0")).with(Padding::new(3, 3, 0, 0)))
        .with(Modify::new(Segment::all()).with(Alignment::center())),
        " N |   column 0   | column 1 | column 2 "
        "---+--------------+----------+----------"
        " 0 |     0-0      |   0-1    |   0-2    "
        " 1 |     1-0      |   1-1    |   1-2    "
        " 2 |     2-0      |   2-1    |   2-2    "
);

test_table!(
    padding_by_column_name_not_first_row,
    create_table::<3, 3>()
        .with(Style::psql())
        .with(Modify::new(ByColumnName::new("0-2")).with(Padding::new(3, 3, 0, 0)))
        .with(Modify::new(Segment::all()).with(Alignment::center())),
        " N | column 0 | column 1 | column 2 "
        "---+----------+----------+----------"
        " 0 |   0-0    |   0-1    |   0-2    "
        " 1 |   1-0    |   1-1    |   1-2    "
        " 2 |   2-0    |   2-1    |   2-2    "
);

test_table!(
    padding_by_column_name_not_existing,
    create_table::<3, 3>()
        .with(Style::psql())
        .with(Modify::new(ByColumnName::new("column 01123123")).with(Padding::new(3, 3, 0, 0)))
        .with(Modify::new(Segment::all()).with(Alignment::center())),
        " N | column 0 | column 1 | column 2 "
        "---+----------+----------+----------"
        " 0 |   0-0    |   0-1    |   0-2    "
        " 1 |   1-0    |   1-1    |   1-2    "
        " 2 |   2-0    |   2-1    |   2-2    "
);
