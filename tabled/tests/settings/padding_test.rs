#![cfg(feature = "std")]

use tabled::settings::{
    object::{Rows, Segment},
    Alignment, Modify, Padding, Style,
};

use crate::matrix::Matrix;
use testing_table::test_table;

#[cfg(feature = "ansi")]
use ::{
    owo_colors::OwoColorize,
    tabled::settings::{Color, PaddingColor},
};

test_table!(
    padding,
    Matrix::new(3, 3)
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
    Matrix::new(3, 3)
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Padding::new(1, 2, 1, 1).fill('>', '<', 'V', '^'))),
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
    Matrix::new(3, 3)
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Padding::zero().fill('>', '<', '^', 'V'))),
    "N|column 0|column 1|column 2"
    "-+--------+--------+--------"
    "0|  0-0   |  0-1   |  0-2   "
    "1|  1-0   |  1-1   |  1-2   "
    "2|  2-0   |  2-1   |  2-2   "
);

test_table!(
    padding_multiline,
    Matrix::new(3, 3)
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
    Matrix::new(3, 3)
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

#[cfg(feature = "ansi")]
test_table!(
    padding_color,
    Matrix::new(3, 3)
        .with(Style::psql())
        .modify(Rows::new(1..), Padding::new(2, 2, 2, 2))
        .modify(Rows::new(1..), PaddingColor::new(
            Color::parse(' '.on_yellow().to_string()),
            Color::parse(' '.on_blue().to_string()),
            Color::parse(' '.on_red().to_string()),
            Color::parse(' '.on_green().to_string()),
        )),
    "  N  | column 0 | column 1 | column 2 \n-----+----------+----------+----------\n\u{1b}[41m     \u{1b}[49m|\u{1b}[41m          \u{1b}[49m|\u{1b}[41m          \u{1b}[49m|\u{1b}[41m          \u{1b}[49m\n\u{1b}[41m     \u{1b}[49m|\u{1b}[41m          \u{1b}[49m|\u{1b}[41m          \u{1b}[49m|\u{1b}[41m          \u{1b}[49m\n\u{1b}[43m  \u{1b}[49m0\u{1b}[44m  \u{1b}[49m|\u{1b}[43m  \u{1b}[49m 0-0  \u{1b}[44m  \u{1b}[49m|\u{1b}[43m  \u{1b}[49m 0-1  \u{1b}[44m  \u{1b}[49m|\u{1b}[43m  \u{1b}[49m 0-2  \u{1b}[44m  \u{1b}[49m\n\u{1b}[42m     \u{1b}[49m|\u{1b}[42m          \u{1b}[49m|\u{1b}[42m          \u{1b}[49m|\u{1b}[42m          \u{1b}[49m\n\u{1b}[42m     \u{1b}[49m|\u{1b}[42m          \u{1b}[49m|\u{1b}[42m          \u{1b}[49m|\u{1b}[42m          \u{1b}[49m\n\u{1b}[41m     \u{1b}[49m|\u{1b}[41m          \u{1b}[49m|\u{1b}[41m          \u{1b}[49m|\u{1b}[41m          \u{1b}[49m\n\u{1b}[41m     \u{1b}[49m|\u{1b}[41m          \u{1b}[49m|\u{1b}[41m          \u{1b}[49m|\u{1b}[41m          \u{1b}[49m\n\u{1b}[43m  \u{1b}[49m1\u{1b}[44m  \u{1b}[49m|\u{1b}[43m  \u{1b}[49m 1-0  \u{1b}[44m  \u{1b}[49m|\u{1b}[43m  \u{1b}[49m 1-1  \u{1b}[44m  \u{1b}[49m|\u{1b}[43m  \u{1b}[49m 1-2  \u{1b}[44m  \u{1b}[49m\n\u{1b}[42m     \u{1b}[49m|\u{1b}[42m          \u{1b}[49m|\u{1b}[42m          \u{1b}[49m|\u{1b}[42m          \u{1b}[49m\n\u{1b}[42m     \u{1b}[49m|\u{1b}[42m          \u{1b}[49m|\u{1b}[42m          \u{1b}[49m|\u{1b}[42m          \u{1b}[49m\n\u{1b}[41m     \u{1b}[49m|\u{1b}[41m          \u{1b}[49m|\u{1b}[41m          \u{1b}[49m|\u{1b}[41m          \u{1b}[49m\n\u{1b}[41m     \u{1b}[49m|\u{1b}[41m          \u{1b}[49m|\u{1b}[41m          \u{1b}[49m|\u{1b}[41m          \u{1b}[49m\n\u{1b}[43m  \u{1b}[49m2\u{1b}[44m  \u{1b}[49m|\u{1b}[43m  \u{1b}[49m 2-0  \u{1b}[44m  \u{1b}[49m|\u{1b}[43m  \u{1b}[49m 2-1  \u{1b}[44m  \u{1b}[49m|\u{1b}[43m  \u{1b}[49m 2-2  \u{1b}[44m  \u{1b}[49m\n\u{1b}[42m     \u{1b}[49m|\u{1b}[42m          \u{1b}[49m|\u{1b}[42m          \u{1b}[49m|\u{1b}[42m          \u{1b}[49m\n\u{1b}[42m     \u{1b}[49m|\u{1b}[42m          \u{1b}[49m|\u{1b}[42m          \u{1b}[49m|\u{1b}[42m          \u{1b}[49m"
);

test_table!(
    padding_table,
    Matrix::new(3, 3)
        .with(Style::psql())
        .with(Padding::new(1, 1, 0, 2)),
    " N | column 0 | column 1 | column 2 "
    "   |          |          |          "
    "   |          |          |          "
    "---+----------+----------+----------"
    " 0 |   0-0    |   0-1    |   0-2    "
    "   |          |          |          "
    "   |          |          |          "
    " 1 |   1-0    |   1-1    |   1-2    "
    "   |          |          |          "
    "   |          |          |          "
    " 2 |   2-0    |   2-1    |   2-2    "
    "   |          |          |          "
    "   |          |          |          "
);
