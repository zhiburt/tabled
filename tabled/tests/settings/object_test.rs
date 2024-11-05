#![cfg(feature = "std")]

use tabled::grid::config::Entity;
use tabled::settings::{
    object::{Columns, Object, ObjectIterator, Segment},
    Alignment, Style,
};

use crate::matrix::Matrix;
use testing_table::test_table;

// todo: Columns::all()

test_table!(
    skip,
    Matrix::new(3, 3).with(Style::psql()).modify(Columns::new(..).not(Columns::first()).skip(2), Alignment::right()),
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    " 0 |   0-0    |      0-1 |      0-2 "
    " 1 |      1-0 |      1-1 |      1-2 "
    " 2 |      2-0 |      2-1 |      2-2 "
);

test_table!(
    skip_segment_all,
    Matrix::new(3, 3).with(Style::psql()).modify(Segment::all().skip(1), Alignment::right()),
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    " 0 |   0-0    |   0-1    |   0-2    "
    " 1 |   1-0    |   1-1    |   1-2    "
    " 2 |   2-0    |   2-1    |   2-2    "
);

test_table!(
    step_by,
    Matrix::new(3, 3).with(Style::psql()).modify(Columns::new(..).step_by(3), Alignment::right()),
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    " 0 |   0-0    |   0-1    |      0-2 "
    " 1 |   1-0    |   1-1    |      1-2 "
    " 2 |   2-0    |   2-1    |      2-2 "
);

test_table!(
    filter,
    Matrix::new(3, 3).with(Style::psql()).modify(Columns::new(..).filter(|e| matches!(e, Entity::Column(1 | 3))), Alignment::right()),
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    " 0 |      0-0 |   0-1    |      0-2 "
    " 1 |      1-0 |   1-1    |      1-2 "
    " 2 |      2-0 |   2-1    |      2-2 "
);
