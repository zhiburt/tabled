#![cfg(feature = "std")]

use tabled::settings::{
    object::{Cell, Columns, Rows, Segment},
    Dup,
};

use crate::matrix::Matrix;
use testing_table::test_table;

test_table!(
    dup_cell_to_cell,
    Matrix::new(3, 3).with(Dup::new(Cell::new(0, 0), Cell::new(0, 1))),
    "+----------+----------+----------+----------+"
    "| column 0 | column 0 | column 1 | column 2 |"
    "+----------+----------+----------+----------+"
    "|    0     |   0-0    |   0-1    |   0-2    |"
    "+----------+----------+----------+----------+"
    "|    1     |   1-0    |   1-1    |   1-2    |"
    "+----------+----------+----------+----------+"
    "|    2     |   2-0    |   2-1    |   2-2    |"
    "+----------+----------+----------+----------+"
);

test_table!(
    dup_cell_to_column,
    Matrix::new(3, 3).with(Dup::new(Columns::single(1), Cell::new(0, 0))),
    "+---+---+----------+----------+"
    "| N | N | column 1 | column 2 |"
    "+---+---+----------+----------+"
    "| 0 | N |   0-1    |   0-2    |"
    "+---+---+----------+----------+"
    "| 1 | N |   1-1    |   1-2    |"
    "+---+---+----------+----------+"
    "| 2 | N |   2-1    |   2-2    |"
    "+---+---+----------+----------+"
);

test_table!(
    dup_row_to_row_single,
    Matrix::new(3, 3).with(Dup::new(Rows::single(1), Rows::single(0))),
    "+---+----------+----------+----------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| 1 |   1-0    |   1-1    |   1-2    |"
    "+---+----------+----------+----------+"
    "| 2 |   2-0    |   2-1    |   2-2    |"
    "+---+----------+----------+----------+"
);

test_table!(
    dup_row_to_row_single_to_many,
    Matrix::new(3, 3).with(Dup::new(Rows::new(1..3), Rows::single(0))),
    "+---+----------+----------+----------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| 2 |   2-0    |   2-1    |   2-2    |"
    "+---+----------+----------+----------+"
);

test_table!(
    dup_row_to_row_single_to_all,
    Matrix::new(3, 3).with(Dup::new(Rows::new(1..), Rows::single(0))),
    "+---+----------+----------+----------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
);

test_table!(
    dup_row_to_column_single,
    Matrix::new(3, 3).with(Dup::new(Columns::single(1), Rows::single(0))),
    "+---+----------+----------+----------+"
    "| N |    N     | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| 0 | column 0 |   0-1    |   0-2    |"
    "+---+----------+----------+----------+"
    "| 1 | column 1 |   1-1    |   1-2    |"
    "+---+----------+----------+----------+"
    "| 2 | column 2 |   2-1    |   2-2    |"
    "+---+----------+----------+----------+"
);

test_table!(
    dup_column_to_row_single,
    Matrix::new(3, 3).with(Dup::new(Columns::single(1), Columns::single(0))),
    "+---+---+----------+----------+"
    "| N | N | column 1 | column 2 |"
    "+---+---+----------+----------+"
    "| 0 | 0 |   0-1    |   0-2    |"
    "+---+---+----------+----------+"
    "| 1 | 1 |   1-1    |   1-2    |"
    "+---+---+----------+----------+"
    "| 2 | 2 |   2-1    |   2-2    |"
    "+---+---+----------+----------+"
);

test_table!(
    dup_row_to_column_single_repeat,
    Matrix::new(4, 3).with(Dup::new(Columns::single(1), Rows::single(0))),
    "+---+----------+----------+----------+"
    "| N |    N     | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| 0 | column 0 |   0-1    |   0-2    |"
    "+---+----------+----------+----------+"
    "| 1 | column 1 |   1-1    |   1-2    |"
    "+---+----------+----------+----------+"
    "| 2 | column 2 |   2-1    |   2-2    |"
    "+---+----------+----------+----------+"
    "| 3 |    N     |   3-1    |   3-2    |"
    "+---+----------+----------+----------+"
);

test_table!(
    dup_column_to_row_single_stop,
    Matrix::new(4, 3).with(Dup::new(Rows::single(1), Columns::single(0))),
    "+---+----------+----------+----------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| N |    0     |    1     |    2     |"
    "+---+----------+----------+----------+"
    "| 1 |   1-0    |   1-1    |   1-2    |"
    "+---+----------+----------+----------+"
    "| 2 |   2-0    |   2-1    |   2-2    |"
    "+---+----------+----------+----------+"
    "| 3 |   3-0    |   3-1    |   3-2    |"
    "+---+----------+----------+----------+"
);

test_table!(
    dup_row_to_global,
    Matrix::new(4, 3).with(Dup::new(Segment::all(), Rows::single(0))),
    "+---+----------+----------+----------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
);

test_table!(
    dup_column_to_global,
    Matrix::new(4, 3).with(Dup::new(Segment::all(), Columns::single(0))),
    "+---+---+---+---+"
    "| N | 0 | 1 | 2 |"
    "+---+---+---+---+"
    "| 3 | N | 0 | 1 |"
    "+---+---+---+---+"
    "| 2 | 3 | N | 0 |"
    "+---+---+---+---+"
    "| 1 | 2 | 3 | N |"
    "+---+---+---+---+"
    "| 0 | 1 | 2 | 3 |"
    "+---+---+---+---+"
);

test_table!(
    dup_empty_table,
    Matrix::empty().with(Dup::new(Segment::all(), Columns::single(0))),
    ""
);

test_table!(
    dup_invalid_target,
    Matrix::new(4, 3).with(Dup::new(Segment::all(), Columns::single(99))),
    Matrix::new(4, 3),
);

test_table!(
    dup_invalid_source,
    Matrix::new(4, 3).with(Dup::new(Rows::single(99), Columns::first())),
    Matrix::new(4, 3),
);
