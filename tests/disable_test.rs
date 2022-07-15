use tabled::{object::Segment, Alignment, Disable, Modify, Style, Table};

use crate::util::{create_vector, test_table};

mod util;

test_table!(
    disable_rows,
    Table::new(create_vector::<3, 3>()).with(Disable::Row(1..=2)),
    "+---+----------+----------+----------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| 2 |   2-0    |   2-1    |   2-2    |"
    "+---+----------+----------+----------+"
);

test_table!(
    disable_header,
    Table::new(create_vector::<3, 3>()).with(Style::psql()).with(Disable::Row(..1)),
    "---+-----+-----+-----"
    " 0 | 0-0 | 0-1 | 0-2 "
    " 1 | 1-0 | 1-1 | 1-2 "
    " 2 | 2-0 | 2-1 | 2-2 "
);

test_table!(
    disable_all_table_via_rows,
    Table::new(create_vector::<3, 3>())
        .with(Style::psql())
        .with(Disable::Row(..)),
    ""
);

test_table!(
    disable_header_with_new_styling,
    Table::new(create_vector::<3, 3>())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Disable::Row(..1))
        .with(Style::modern().off_horizontal().lines([(1, Style::modern().get_horizontal())])),
    "┌───┬─────┬─────┬─────┐"
    "│ 0 │ 0-0 │ 0-1 │ 0-2 │"
    "├───┼─────┼─────┼─────┤"
    "│ 1 │ 1-0 │ 1-1 │ 1-2 │"
    "│ 2 │ 2-0 │ 2-1 │ 2-2 │"
    "└───┴─────┴─────┴─────┘"
);

test_table!(
    disable_columns,
    Table::new(create_vector::<3, 3>()).with(Style::psql()).with(Disable::Column(..1)),
    "| column 0 | column 1 | column 2 "
    "+----------+----------+----------"
    "|   0-0    |   0-1    |   0-2    "
    "|   1-0    |   1-1    |   1-2    "
    "|   2-0    |   2-1    |   2-2    "
);

test_table!(
    disable_all_table_via_columns,
    Table::new(create_vector::<3, 3>())
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Disable::Column(..)),
    ""
);
