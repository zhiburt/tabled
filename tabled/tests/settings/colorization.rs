#![cfg(feature = "std")]

use tabled::settings::{
    object::{Cell, Object},
    themes::Colorization,
    Color,
};

use crate::matrix::Matrix;
use testing_table::test_table;

test_table!(
    chess_2x3,
    Matrix::new(2, 3).with(Colorization::chess(color1(), color2())),
    "+---+----------+----------+----------+\n|\u{1b}[41m \u{1b}[49m\u{1b}[41mN\u{1b}[49m\u{1b}[41m \u{1b}[49m|\u{1b}[41m \u{1b}[49m\u{1b}[41mcolumn 0\u{1b}[49m\u{1b}[41m \u{1b}[49m|\u{1b}[106m \u{1b}[49m\u{1b}[106mcolumn 1\u{1b}[49m\u{1b}[106m \u{1b}[49m|\u{1b}[41m \u{1b}[49m\u{1b}[41mcolumn 2\u{1b}[49m\u{1b}[41m \u{1b}[49m|\n+---+----------+----------+----------+\n|\u{1b}[106m \u{1b}[49m\u{1b}[106m0\u{1b}[49m\u{1b}[106m \u{1b}[49m|\u{1b}[41m \u{1b}[49m\u{1b}[41m  \u{1b}[49m\u{1b}[41m0-0\u{1b}[49m\u{1b}[41m   \u{1b}[49m\u{1b}[41m \u{1b}[49m|\u{1b}[41m \u{1b}[49m\u{1b}[41m  \u{1b}[49m\u{1b}[41m0-1\u{1b}[49m\u{1b}[41m   \u{1b}[49m\u{1b}[41m \u{1b}[49m|\u{1b}[106m \u{1b}[49m\u{1b}[106m  \u{1b}[49m\u{1b}[106m0-2\u{1b}[49m\u{1b}[106m   \u{1b}[49m\u{1b}[106m \u{1b}[49m|\n+---+----------+----------+----------+\n|\u{1b}[41m \u{1b}[49m\u{1b}[41m1\u{1b}[49m\u{1b}[41m \u{1b}[49m|\u{1b}[106m \u{1b}[49m\u{1b}[106m  \u{1b}[49m\u{1b}[106m1-0\u{1b}[49m\u{1b}[106m   \u{1b}[49m\u{1b}[106m \u{1b}[49m|\u{1b}[41m \u{1b}[49m\u{1b}[41m  \u{1b}[49m\u{1b}[41m1-1\u{1b}[49m\u{1b}[41m   \u{1b}[49m\u{1b}[41m \u{1b}[49m|\u{1b}[41m \u{1b}[49m\u{1b}[41m  \u{1b}[49m\u{1b}[41m1-2\u{1b}[49m\u{1b}[41m   \u{1b}[49m\u{1b}[41m \u{1b}[49m|\n+---+----------+----------+----------+"
);

test_table!(
    chess_3x3,
    Matrix::new(3, 3).with(Colorization::chess(color1(), color2())),
    "+---+----------+----------+----------+\n|\u{1b}[41m \u{1b}[49m\u{1b}[41mN\u{1b}[49m\u{1b}[41m \u{1b}[49m|\u{1b}[106m \u{1b}[49m\u{1b}[106mcolumn 0\u{1b}[49m\u{1b}[106m \u{1b}[49m|\u{1b}[41m \u{1b}[49m\u{1b}[41mcolumn 1\u{1b}[49m\u{1b}[41m \u{1b}[49m|\u{1b}[106m \u{1b}[49m\u{1b}[106mcolumn 2\u{1b}[49m\u{1b}[106m \u{1b}[49m|\n+---+----------+----------+----------+\n|\u{1b}[106m \u{1b}[49m\u{1b}[106m0\u{1b}[49m\u{1b}[106m \u{1b}[49m|\u{1b}[41m \u{1b}[49m\u{1b}[41m  \u{1b}[49m\u{1b}[41m0-0\u{1b}[49m\u{1b}[41m   \u{1b}[49m\u{1b}[41m \u{1b}[49m|\u{1b}[106m \u{1b}[49m\u{1b}[106m  \u{1b}[49m\u{1b}[106m0-1\u{1b}[49m\u{1b}[106m   \u{1b}[49m\u{1b}[106m \u{1b}[49m|\u{1b}[41m \u{1b}[49m\u{1b}[41m  \u{1b}[49m\u{1b}[41m0-2\u{1b}[49m\u{1b}[41m   \u{1b}[49m\u{1b}[41m \u{1b}[49m|\n+---+----------+----------+----------+\n|\u{1b}[41m \u{1b}[49m\u{1b}[41m1\u{1b}[49m\u{1b}[41m \u{1b}[49m|\u{1b}[106m \u{1b}[49m\u{1b}[106m  \u{1b}[49m\u{1b}[106m1-0\u{1b}[49m\u{1b}[106m   \u{1b}[49m\u{1b}[106m \u{1b}[49m|\u{1b}[41m \u{1b}[49m\u{1b}[41m  \u{1b}[49m\u{1b}[41m1-1\u{1b}[49m\u{1b}[41m   \u{1b}[49m\u{1b}[41m \u{1b}[49m|\u{1b}[106m \u{1b}[49m\u{1b}[106m  \u{1b}[49m\u{1b}[106m1-2\u{1b}[49m\u{1b}[106m   \u{1b}[49m\u{1b}[106m \u{1b}[49m|\n+---+----------+----------+----------+\n|\u{1b}[106m \u{1b}[49m\u{1b}[106m2\u{1b}[49m\u{1b}[106m \u{1b}[49m|\u{1b}[41m \u{1b}[49m\u{1b}[41m  \u{1b}[49m\u{1b}[41m2-0\u{1b}[49m\u{1b}[41m   \u{1b}[49m\u{1b}[41m \u{1b}[49m|\u{1b}[106m \u{1b}[49m\u{1b}[106m  \u{1b}[49m\u{1b}[106m2-1\u{1b}[49m\u{1b}[106m   \u{1b}[49m\u{1b}[106m \u{1b}[49m|\u{1b}[41m \u{1b}[49m\u{1b}[41m  \u{1b}[49m\u{1b}[41m2-2\u{1b}[49m\u{1b}[41m   \u{1b}[49m\u{1b}[41m \u{1b}[49m|\n+---+----------+----------+----------+"
);

test_table!(
    rows,
    Matrix::new(2, 3).with(Colorization::rows([color1(), color2(), color3()])),
    "+---+----------+----------+----------+\n|\u{1b}[41m \u{1b}[49m\u{1b}[41mN\u{1b}[49m\u{1b}[41m \u{1b}[49m|\u{1b}[41m \u{1b}[49m\u{1b}[41mcolumn 0\u{1b}[49m\u{1b}[41m \u{1b}[49m|\u{1b}[41m \u{1b}[49m\u{1b}[41mcolumn 1\u{1b}[49m\u{1b}[41m \u{1b}[49m|\u{1b}[41m \u{1b}[49m\u{1b}[41mcolumn 2\u{1b}[49m\u{1b}[41m \u{1b}[49m|\n+---+----------+----------+----------+\n|\u{1b}[106m \u{1b}[49m\u{1b}[106m0\u{1b}[49m\u{1b}[106m \u{1b}[49m|\u{1b}[106m \u{1b}[49m\u{1b}[106m  \u{1b}[49m\u{1b}[106m0-0\u{1b}[49m\u{1b}[106m   \u{1b}[49m\u{1b}[106m \u{1b}[49m|\u{1b}[106m \u{1b}[49m\u{1b}[106m  \u{1b}[49m\u{1b}[106m0-1\u{1b}[49m\u{1b}[106m   \u{1b}[49m\u{1b}[106m \u{1b}[49m|\u{1b}[106m \u{1b}[49m\u{1b}[106m  \u{1b}[49m\u{1b}[106m0-2\u{1b}[49m\u{1b}[106m   \u{1b}[49m\u{1b}[106m \u{1b}[49m|\n+---+----------+----------+----------+\n|\u{1b}[1m \u{1b}[22m\u{1b}[1m1\u{1b}[22m\u{1b}[1m \u{1b}[22m|\u{1b}[1m \u{1b}[22m\u{1b}[1m  \u{1b}[22m\u{1b}[1m1-0\u{1b}[22m\u{1b}[1m   \u{1b}[22m\u{1b}[1m \u{1b}[22m|\u{1b}[1m \u{1b}[22m\u{1b}[1m  \u{1b}[22m\u{1b}[1m1-1\u{1b}[22m\u{1b}[1m   \u{1b}[22m\u{1b}[1m \u{1b}[22m|\u{1b}[1m \u{1b}[22m\u{1b}[1m  \u{1b}[22m\u{1b}[1m1-2\u{1b}[22m\u{1b}[1m   \u{1b}[22m\u{1b}[1m \u{1b}[22m|\n+---+----------+----------+----------+"
);

test_table!(
    columns,
    Matrix::new(2, 3).with(Colorization::columns([color1(), color2(), color3()])),
    "+---+----------+----------+----------+\n|\u{1b}[41m \u{1b}[49m\u{1b}[41mN\u{1b}[49m\u{1b}[41m \u{1b}[49m|\u{1b}[106m \u{1b}[49m\u{1b}[106mcolumn 0\u{1b}[49m\u{1b}[106m \u{1b}[49m|\u{1b}[1m \u{1b}[22m\u{1b}[1mcolumn 1\u{1b}[22m\u{1b}[1m \u{1b}[22m|\u{1b}[41m \u{1b}[49m\u{1b}[41mcolumn 2\u{1b}[49m\u{1b}[41m \u{1b}[49m|\n+---+----------+----------+----------+\n|\u{1b}[41m \u{1b}[49m\u{1b}[41m0\u{1b}[49m\u{1b}[41m \u{1b}[49m|\u{1b}[106m \u{1b}[49m\u{1b}[106m  \u{1b}[49m\u{1b}[106m0-0\u{1b}[49m\u{1b}[106m   \u{1b}[49m\u{1b}[106m \u{1b}[49m|\u{1b}[1m \u{1b}[22m\u{1b}[1m  \u{1b}[22m\u{1b}[1m0-1\u{1b}[22m\u{1b}[1m   \u{1b}[22m\u{1b}[1m \u{1b}[22m|\u{1b}[41m \u{1b}[49m\u{1b}[41m  \u{1b}[49m\u{1b}[41m0-2\u{1b}[49m\u{1b}[41m   \u{1b}[49m\u{1b}[41m \u{1b}[49m|\n+---+----------+----------+----------+\n|\u{1b}[41m \u{1b}[49m\u{1b}[41m1\u{1b}[49m\u{1b}[41m \u{1b}[49m|\u{1b}[106m \u{1b}[49m\u{1b}[106m  \u{1b}[49m\u{1b}[106m1-0\u{1b}[49m\u{1b}[106m   \u{1b}[49m\u{1b}[106m \u{1b}[49m|\u{1b}[1m \u{1b}[22m\u{1b}[1m  \u{1b}[22m\u{1b}[1m1-1\u{1b}[22m\u{1b}[1m   \u{1b}[22m\u{1b}[1m \u{1b}[22m|\u{1b}[41m \u{1b}[49m\u{1b}[41m  \u{1b}[49m\u{1b}[41m1-2\u{1b}[49m\u{1b}[41m   \u{1b}[49m\u{1b}[41m \u{1b}[49m|\n+---+----------+----------+----------+"
);

test_table!(
    by_row,
    Matrix::new(2, 3).with(Colorization::by_row([color1(), color2(), color3()])),
    "+---+----------+----------+----------+\n|\u{1b}[41m \u{1b}[49m\u{1b}[41mN\u{1b}[49m\u{1b}[41m \u{1b}[49m|\u{1b}[106m \u{1b}[49m\u{1b}[106mcolumn 0\u{1b}[49m\u{1b}[106m \u{1b}[49m|\u{1b}[1m \u{1b}[22m\u{1b}[1mcolumn 1\u{1b}[22m\u{1b}[1m \u{1b}[22m|\u{1b}[41m \u{1b}[49m\u{1b}[41mcolumn 2\u{1b}[49m\u{1b}[41m \u{1b}[49m|\n+---+----------+----------+----------+\n|\u{1b}[106m \u{1b}[49m\u{1b}[106m0\u{1b}[49m\u{1b}[106m \u{1b}[49m|\u{1b}[1m \u{1b}[22m\u{1b}[1m  \u{1b}[22m\u{1b}[1m0-0\u{1b}[22m\u{1b}[1m   \u{1b}[22m\u{1b}[1m \u{1b}[22m|\u{1b}[41m \u{1b}[49m\u{1b}[41m  \u{1b}[49m\u{1b}[41m0-1\u{1b}[49m\u{1b}[41m   \u{1b}[49m\u{1b}[41m \u{1b}[49m|\u{1b}[106m \u{1b}[49m\u{1b}[106m  \u{1b}[49m\u{1b}[106m0-2\u{1b}[49m\u{1b}[106m   \u{1b}[49m\u{1b}[106m \u{1b}[49m|\n+---+----------+----------+----------+\n|\u{1b}[1m \u{1b}[22m\u{1b}[1m1\u{1b}[22m\u{1b}[1m \u{1b}[22m|\u{1b}[41m \u{1b}[49m\u{1b}[41m  \u{1b}[49m\u{1b}[41m1-0\u{1b}[49m\u{1b}[41m   \u{1b}[49m\u{1b}[41m \u{1b}[49m|\u{1b}[106m \u{1b}[49m\u{1b}[106m  \u{1b}[49m\u{1b}[106m1-1\u{1b}[49m\u{1b}[106m   \u{1b}[49m\u{1b}[106m \u{1b}[49m|\u{1b}[1m \u{1b}[22m\u{1b}[1m  \u{1b}[22m\u{1b}[1m1-2\u{1b}[22m\u{1b}[1m   \u{1b}[22m\u{1b}[1m \u{1b}[22m|\n+---+----------+----------+----------+"
);

test_table!(
    by_column,
    Matrix::new(2, 3).with(Colorization::by_column([color1(), color2(), color3()])),
    "+---+----------+----------+----------+\n|\u{1b}[41m \u{1b}[49m\u{1b}[41mN\u{1b}[49m\u{1b}[41m \u{1b}[49m|\u{1b}[41m \u{1b}[49m\u{1b}[41mcolumn 0\u{1b}[49m\u{1b}[41m \u{1b}[49m|\u{1b}[41m \u{1b}[49m\u{1b}[41mcolumn 1\u{1b}[49m\u{1b}[41m \u{1b}[49m|\u{1b}[41m \u{1b}[49m\u{1b}[41mcolumn 2\u{1b}[49m\u{1b}[41m \u{1b}[49m|\n+---+----------+----------+----------+\n|\u{1b}[106m \u{1b}[49m\u{1b}[106m0\u{1b}[49m\u{1b}[106m \u{1b}[49m|\u{1b}[106m \u{1b}[49m\u{1b}[106m  \u{1b}[49m\u{1b}[106m0-0\u{1b}[49m\u{1b}[106m   \u{1b}[49m\u{1b}[106m \u{1b}[49m|\u{1b}[106m \u{1b}[49m\u{1b}[106m  \u{1b}[49m\u{1b}[106m0-1\u{1b}[49m\u{1b}[106m   \u{1b}[49m\u{1b}[106m \u{1b}[49m|\u{1b}[106m \u{1b}[49m\u{1b}[106m  \u{1b}[49m\u{1b}[106m0-2\u{1b}[49m\u{1b}[106m   \u{1b}[49m\u{1b}[106m \u{1b}[49m|\n+---+----------+----------+----------+\n|\u{1b}[1m \u{1b}[22m\u{1b}[1m1\u{1b}[22m\u{1b}[1m \u{1b}[22m|\u{1b}[1m \u{1b}[22m\u{1b}[1m  \u{1b}[22m\u{1b}[1m1-0\u{1b}[22m\u{1b}[1m   \u{1b}[22m\u{1b}[1m \u{1b}[22m|\u{1b}[1m \u{1b}[22m\u{1b}[1m  \u{1b}[22m\u{1b}[1m1-1\u{1b}[22m\u{1b}[1m   \u{1b}[22m\u{1b}[1m \u{1b}[22m|\u{1b}[1m \u{1b}[22m\u{1b}[1m  \u{1b}[22m\u{1b}[1m1-2\u{1b}[22m\u{1b}[1m   \u{1b}[22m\u{1b}[1m \u{1b}[22m|\n+---+----------+----------+----------+"
);

test_table!(
    exact,
    Matrix::new(2, 3).with(Colorization::exact([color1(), color2(), color3()], Cell::new(0, 0).and(Cell::new(1, 1)).and(Cell::new(2, 2)))),
    "+---+----------+----------+----------+\n|\u{1b}[41m \u{1b}[49m\u{1b}[41mN\u{1b}[49m\u{1b}[41m \u{1b}[49m| column 0 | column 1 | column 2 |\n+---+----------+----------+----------+\n| 0 |\u{1b}[106m \u{1b}[49m\u{1b}[106m  \u{1b}[49m\u{1b}[106m0-0\u{1b}[49m\u{1b}[106m   \u{1b}[49m\u{1b}[106m \u{1b}[49m|   0-1    |   0-2    |\n+---+----------+----------+----------+\n| 1 |   1-0    |\u{1b}[1m \u{1b}[22m\u{1b}[1m  \u{1b}[22m\u{1b}[1m1-1\u{1b}[22m\u{1b}[1m   \u{1b}[22m\u{1b}[1m \u{1b}[22m|   1-2    |\n+---+----------+----------+----------+"
);

fn color1() -> Color {
    Color::BG_RED
}

fn color2() -> Color {
    Color::BG_BRIGHT_CYAN
}

fn color3() -> Color {
    Color::BOLD
}
