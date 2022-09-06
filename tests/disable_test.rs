use tabled::{object::Segment, style::HorizontalLine, Alignment, Disable, Modify, Style};

use crate::util::{create_table, test_table};

mod util;

test_table!(
    disable_rows,
    create_table::<3, 3>().with(Disable::Row(1..=2)),
    "+---+----------+----------+----------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| 2 |   2-0    |   2-1    |   2-2    |"
    "+---+----------+----------+----------+"
);

test_table!(
    disable_header,
    create_table::<3, 3>().with(Style::psql()).with(Disable::Row(..1)),
    " 0 | 0-0 | 0-1 | 0-2 "
    "---+-----+-----+-----"
    " 1 | 1-0 | 1-1 | 1-2 "
    " 2 | 2-0 | 2-1 | 2-2 "
);

test_table!(
    disable_all_table_via_rows,
    create_table::<3, 3>()
        .with(Style::psql())
        .with(Disable::Row(..)),
    ""
);

test_table!(
    disable_header_with_new_styling,
    create_table::<3, 3>()
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Disable::Row(..1))
        .with(Style::modern().off_horizontal().horizontals([HorizontalLine::new(1, Style::modern().get_horizontal())])),
    "┌───┬─────┬─────┬─────┐"
    "│ 0 │ 0-0 │ 0-1 │ 0-2 │"
    "├───┼─────┼─────┼─────┤"
    "│ 1 │ 1-0 │ 1-1 │ 1-2 │"
    "│ 2 │ 2-0 │ 2-1 │ 2-2 │"
    "└───┴─────┴─────┴─────┘"
);

test_table!(
    disable_columns,
    create_table::<3, 3>().with(Style::psql()).with(Disable::Column(..1)),
    " column 0 | column 1 | column 2 "
    "----------+----------+----------"
    "   0-0    |   0-1    |   0-2    "
    "   1-0    |   1-1    |   1-2    "
    "   2-0    |   2-1    |   2-2    "
);

test_table!(
    disable_all_table_via_columns,
    create_table::<3, 3>()
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Disable::Column(..)),
    ""
);
