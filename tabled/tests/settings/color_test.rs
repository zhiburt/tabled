#![cfg(feature = "std")]
#![cfg(feature = "assert")]

use tabled::{
    assert::test_table,
    settings::{Color, Modify},
};

use crate::util::Matrix;

test_table!(
    color_global,
    Matrix::new(3, 3).with(Color::FG_MAGENTA),
    "+---+----------+----------+----------+"
    "| \u{1b}[35mN\u{1b}[39m | \u{1b}[35mcolumn 0\u{1b}[39m | \u{1b}[35mcolumn 1\u{1b}[39m | \u{1b}[35mcolumn 2\u{1b}[39m |"
    "+---+----------+----------+----------+"
    "| \u{1b}[35m0\u{1b}[39m |   \u{1b}[35m0-0\u{1b}[39m    |   \u{1b}[35m0-1\u{1b}[39m    |   \u{1b}[35m0-2\u{1b}[39m    |"
    "+---+----------+----------+----------+"
    "| \u{1b}[35m1\u{1b}[39m |   \u{1b}[35m1-0\u{1b}[39m    |   \u{1b}[35m1-1\u{1b}[39m    |   \u{1b}[35m1-2\u{1b}[39m    |"
    "+---+----------+----------+----------+"
    "| \u{1b}[35m2\u{1b}[39m |   \u{1b}[35m2-0\u{1b}[39m    |   \u{1b}[35m2-1\u{1b}[39m    |   \u{1b}[35m2-2\u{1b}[39m    |"
    "+---+----------+----------+----------+"
);

test_table!(
    color_cell,
    Matrix::new(3, 3).with(Modify::new((0, 0)).with(Color::BG_BRIGHT_BLACK)),
    "+---+----------+----------+----------+"
    "| \u{1b}[100mN\u{1b}[49m | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| 0 |   0-0    |   0-1    |   0-2    |"
    "+---+----------+----------+----------+"
    "| 1 |   1-0    |   1-1    |   1-2    |"
    "+---+----------+----------+----------+"
    "| 2 |   2-0    |   2-1    |   2-2    |"
    "+---+----------+----------+----------+"
);
