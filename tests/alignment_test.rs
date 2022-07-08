use tabled::{
    object::{Columns, Rows, Segment},
    Alignment, Modify, Padding, Style, Table,
};

use crate::util::{create_vector, test_table};

mod util;

test_table!(
    full_alignment,
    Table::new(create_vector::<3, 3>()).with(Style::psql()).with(Modify::new(Segment::all()).with(Alignment::left())),
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    " 0 | 0-0      | 0-1      | 0-2      "
    " 1 | 1-0      | 1-1      | 1-2      "
    " 2 | 2-0      | 2-1      | 2-2      "
);

test_table!(
    head_and_data_alignment,
    Table::new(create_vector::<3, 3>())
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
    Table::new({
            let mut data = create_vector::<3, 3>();
            data[2][2] = String::from("https://\nwww\n.\nredhat\n.com\n/en");
            data
        })
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
    Table::new({
            let mut data = create_vector::<3, 3>();
            data[1][2] = String::from("E\nnde\navou\nros");
            data[2][2] = String::from("Red\nHat");
            data[2][3] = String::from("https://\nwww\n.\nredhat\n.com\n/en");
            data
        })
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
    Table::new(create_vector::<3, 3>())
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Padding::new(3, 0, 0, 0)))
        .with(Modify::new(Segment::all()).with(Alignment::left())),
    "   N|   column 0|   column 1|   column 2"
    "----+-----------+-----------+-----------"
    "   0|   0-0     |   0-1     |   0-2     "
    "   1|   1-0     |   1-1     |   1-2     "
    "   2|   2-0     |   2-1     |   2-2     "
);
