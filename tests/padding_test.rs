use tabled::{
    object::{Rows, Segment},
    Alignment, Modify, Padding, Style, Table,
};

use crate::util::{create_vector, test_table};

mod util;

test_table!(
    padding,
    Table::new(create_vector::<3, 3>())
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Rows::new(1..)).with(Padding::new(1, 1, 0, 2))),
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    " 0 | 0-0      | 0-1      | 0-2      "
    "   |          |          |          "
    "   |          |          |          "
    " 1 | 1-0      | 1-1      | 1-2      "
    "   |          |          |          "
    "   |          |          |          "
    " 2 | 2-0      | 2-1      | 2-2      "
    "   |          |          |          "
    "   |          |          |          "
);

test_table!(
    padding_with_set_characters,
    Table::new(create_vector::<3, 3>())
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Padding::new(1, 2, 1, 1).set_fill('>', '<', 'V', '^'))),
    "VVVV|VVVVVVVVVVV|VVVVVVVVVVV|VVVVVVVVVVV"
    ">N<<|>column 0<<|>column 1<<|>column 2<<"
    "^^^^|^^^^^^^^^^^|^^^^^^^^^^^|^^^^^^^^^^^"
    "----+-----------+-----------+-----------"
    "VVVV|VVVVVVVVVVV|VVVVVVVVVVV|VVVVVVVVVVV"
    ">0<<|>  0-0   <<|>  0-1   <<|>  0-2   <<"
    "^^^^|^^^^^^^^^^^|^^^^^^^^^^^|^^^^^^^^^^^"
    "VVVV|VVVVVVVVVVV|VVVVVVVVVVV|VVVVVVVVVVV"
    ">1<<|>  1-0   <<|>  1-1   <<|>  1-2   <<"
    "^^^^|^^^^^^^^^^^|^^^^^^^^^^^|^^^^^^^^^^^"
    "VVVV|VVVVVVVVVVV|VVVVVVVVVVV|VVVVVVVVVVV"
    ">2<<|>  2-0   <<|>  2-1   <<|>  2-2   <<"
    "^^^^|^^^^^^^^^^^|^^^^^^^^^^^|^^^^^^^^^^^"
);

test_table!(
    padding_with_set_characters_and_zero_ident,
    Table::new(create_vector::<3, 3>())
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Padding::zero().set_fill('>', '<', '^', 'V'))),
    "N|column 0|column 1|column 2"
    "-+--------+--------+--------"
    "0|  0-0   |  0-1   |  0-2   "
    "1|  1-0   |  1-1   |  1-2   "
    "2|  2-0   |  2-1   |  2-2   "
);

test_table!(
    padding_multiline,
    Table::new(create_vector::<3, 3>())
        .with(Style::psql())
        .with(Modify::new(Rows::new(1..)).with(Padding::new(1, 1, 1, 1))),
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    "   |          |          |          "
    " 0 |   0-0    |   0-1    |   0-2    "
    "   |          |          |          "
    "   |          |          |          "
    " 1 |   1-0    |   1-1    |   1-2    "
    "   |          |          |          "
    "   |          |          |          "
    " 2 |   2-0    |   2-1    |   2-2    "
    "   |          |          |          "
);

test_table!(
    padding_multiline_with_vertical_alignment,
    Table::new(create_vector::<3, 3>())
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Alignment::center()).with(Alignment::center_vertical()))
        .with(Modify::new(Rows::new(1..)).with(Padding::new(1, 1, 1, 1))),
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    "   |          |          |          "
    " 0 |   0-0    |   0-1    |   0-2    "
    "   |          |          |          "
    "   |          |          |          "
    " 1 |   1-0    |   1-1    |   1-2    "
    "   |          |          |          "
    "   |          |          |          "
    " 2 |   2-0    |   2-1    |   2-2    "
    "   |          |          |          "
);
