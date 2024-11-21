#![cfg(feature = "std")]

use tabled::settings::{
    location::ByColumnName,
    object::{Columns, Rows, Segment},
    style::{HorizontalLine, Style},
    Alignment, Modify, Remove,
};

use crate::matrix::Matrix;
use testing_table::test_table;

test_table!(
    disable_rows,
    Matrix::new(3, 3).with(Remove::row(Rows::new(1..=2))),
    "+---+----------+----------+----------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| 2 |   2-0    |   2-1    |   2-2    |"
    "+---+----------+----------+----------+"
);

test_table!(
    disable_header,
    Matrix::new(3, 3).with(Style::psql()).with(Remove::row(Rows::first())),
    " 0 | 0-0 | 0-1 | 0-2 "
    "---+-----+-----+-----"
    " 1 | 1-0 | 1-1 | 1-2 "
    " 2 | 2-0 | 2-1 | 2-2 "
);

test_table!(
    disable_all_table_via_rows,
    Matrix::new(3, 3)
        .with(Style::psql())
        .with(Remove::row(Columns::new(..))),
    ""
);

test_table!(
    disable_header_with_new_styling,
    Matrix::new(3, 3)
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Remove::row(Rows::new(..1)))
        .with(Style::modern().remove_horizontal().horizontals([(1, HorizontalLine::inherit(Style::modern()))])),
    "┌───┬─────┬─────┬─────┐"
    "│ 0 │ 0-0 │ 0-1 │ 0-2 │"
    "├───┼─────┼─────┼─────┤"
    "│ 1 │ 1-0 │ 1-1 │ 1-2 │"
    "│ 2 │ 2-0 │ 2-1 │ 2-2 │"
    "└───┴─────┴─────┴─────┘"
);

test_table!(
    disable_columns,
    Matrix::new(3, 3).with(Style::psql()).with(Remove::column(Columns::first())),
    " column 0 | column 1 | column 2 "
    "----------+----------+----------"
    "   0-0    |   0-1    |   0-2    "
    "   1-0    |   1-1    |   1-2    "
    "   2-0    |   2-1    |   2-2    "
);

test_table!(
    disable_column_by_name,
    Matrix::new(3, 3).with(Style::psql())
        .with(Remove::column(ByColumnName::new("column 1")))
        .with(Remove::column(ByColumnName::new("column 3"))),
    " N | column 0 | column 2 "
    "---+----------+----------"
    " 0 |   0-0    |   0-2    "
    " 1 |   1-0    |   1-2    "
    " 2 |   2-0    |   2-2    "
);

test_table!(
    disable_all_table_via_columns,
    Matrix::new(3, 3)
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Remove::column(Columns::new(..))),
    ""
);
