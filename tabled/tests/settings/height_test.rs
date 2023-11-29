#![cfg(feature = "std")]

use tabled::settings::{
    object::{Columns, Segment},
    Alignment, Format, Height, Modify, Style,
};

use crate::matrix::Matrix;
use testing_table::test_table;

#[cfg(feature = "color")]
use owo_colors::OwoColorize;

test_table!(
    cell_height_increase,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .with(
            Modify::new(Columns::first())
                .with(Height::increase(3))
        )
        .with(Modify::new(Segment::all()).with(
            Alignment::center_vertical()
        )),
    "| N |          |          |          |"
    "|   | column 0 | column 1 | column 2 |"
    "|   |          |          |          |"
    "|---|----------|----------|----------|"
    "| 0 |          |          |          |"
    "|   |   0-0    |   0-1    |   0-2    |"
    "|   |          |          |          |"
    "| 1 |          |          |          |"
    "|   |   1-0    |   1-1    |   1-2    |"
    "|   |          |          |          |"
    "| 2 |          |          |          |"
    "|   |   2-0    |   2-1    |   2-2    |"
    "|   |          |          |          |"
);

test_table!(
    table_height_increase,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Modify::new(Columns::first()).with(Alignment::center_vertical()))
        .with(Height::increase(10)),
    "|   | column 0 | column 1 | column 2 |"
    "| N |          |          |          |"
    "|   |          |          |          |"
    "|---|----------|----------|----------|"
    "| 0 |   0-0    |   0-1    |   0-2    |"
    "|   |          |          |          |"
    "| 1 |   1-0    |   1-1    |   1-2    |"
    "|   |          |          |          |"
    "| 2 |   2-0    |   2-1    |   2-2    |"
    "|   |          |          |          |"
);

test_table!(
    cell_height_increase_zero,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .with(
            Modify::new(Columns::first())
                .with(Height::increase(0))
        )
        .with(Modify::new(Segment::all()).with(
            Alignment::center_vertical()
        )),
    "| N | column 0 | column 1 | column 2 |"
    "|---|----------|----------|----------|"
    "| 0 |   0-0    |   0-1    |   0-2    |"
    "| 1 |   1-0    |   1-1    |   1-2    |"
    "| 2 |   2-0    |   2-1    |   2-2    |"
);

test_table!(
    table_height_increase_zero,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Modify::new(Columns::first()).with(Alignment::center_vertical()))
        .with(Height::increase(0)),
    "| N | column 0 | column 1 | column 2 |"
    "|---|----------|----------|----------|"
    "| 0 |   0-0    |   0-1    |   0-2    |"
    "| 1 |   1-0    |   1-1    |   1-2    |"
    "| 2 |   2-0    |   2-1    |   2-2    |"
);

test_table!(
    cell_height_limit,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Modify::new(Columns::first()).with(Format::content(|s| format!("xxxx\n{s}xxxx\nxxxx\n"))))
        .with(
            Modify::new(Columns::first())
                .with(Height::limit(1))
        )
        .with(Modify::new(Segment::all()).with(
            Alignment::center_vertical()
        )),
    "| xxxx | column 0 | column 1 | column 2 |"
    "|------|----------|----------|----------|"
    "| xxxx |   0-0    |   0-1    |   0-2    |"
    "| xxxx |   1-0    |   1-1    |   1-2    |"
    "| xxxx |   2-0    |   2-1    |   2-2    |"
);

test_table!(
    table_height_limit,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Modify::new(Columns::first()).with(Format::content(|s| format!("xxxx\n{s}xxxx\nxxxx\n"))))
        .with(Modify::new(Columns::first()).with(Alignment::center_vertical()))
        .with(Height::limit(10)),
    "| xxxx  | column 0 | column 1 | column 2 |"
    "| Nxxxx |          |          |          |"
    "|-------|----------|----------|----------|"
    "| xxxx  |   0-0    |   0-1    |   0-2    |"
    "| 0xxxx |          |          |          |"
    "| xxxx  |   1-0    |   1-1    |   1-2    |"
    "| 1xxxx |          |          |          |"
    "| xxxx  |   2-0    |   2-1    |   2-2    |"
    "| 2xxxx |          |          |          |"
    "| xxxx  |          |          |          |"
);

test_table!(
    table_height_limit_style_change_after,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Modify::new(Columns::first()).with(Format::content(|s| format!("xxxx\n{s}xxxx\nxxxx\n"))))
        .with(Modify::new(Columns::first()).with(Alignment::center_vertical()))
        .with(Height::limit(7)),
    "| xxxx  | column 0 | column 1 | column 2 |"
    "|-------|----------|----------|----------|"
    "| xxxx  |   0-0    |   0-1    |   0-2    |"
    "| xxxx  |   1-0    |   1-1    |   1-2    |"
    "| 1xxxx |          |          |          |"
    "| xxxx  |   2-0    |   2-1    |   2-2    |"
    "| 2xxxx |          |          |          |"
);

test_table!(
    cell_height_limit_zero,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Modify::new(Columns::first()).with(Format::content(|s| format!("xxxx\n{s}xxxx\nxxxx\n"))))
        .with(
            Modify::new(Columns::first())
                .with(Height::limit(0))
        )
        .with(Modify::new(Segment::all()).with(
            Alignment::center_vertical()
        )),
    "|  | column 0 | column 1 | column 2 |"
    "|--|----------|----------|----------|"
    "|  |   0-0    |   0-1    |   0-2    |"
    "|  |   1-0    |   1-1    |   1-2    |"
    "|  |   2-0    |   2-1    |   2-2    |"
);

test_table!(
    table_height_limit_zero,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .with(
            Modify::new(Columns::new(..))
                .with(Format::content(|s| format!("xxxx\n{s}xxxx\nxxxx\n")))
        )
        .with(Height::limit(0)),
    "|--|--|--|--|"
);

test_table!(
    table_height_limit_zero_1,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Height::limit(0))
        .with(
            Modify::new(Columns::new(..)).with(Format::content(|s| format!("xxxx\n{s}xxxx\nxxxx\n")))
        ),
        "| xxxx | xxxx | xxxx | xxxx |"
        "| xxxx | xxxx | xxxx | xxxx |"
        "| xxxx | xxxx | xxxx | xxxx |"
        "|      |      |      |      |"
        "|------|------|------|------|"
        "| xxxx | xxxx | xxxx | xxxx |"
        "| xxxx | xxxx | xxxx | xxxx |"
        "| xxxx | xxxx | xxxx | xxxx |"
        "|      |      |      |      |"
        "| xxxx | xxxx | xxxx | xxxx |"
        "| xxxx | xxxx | xxxx | xxxx |"
        "| xxxx | xxxx | xxxx | xxxx |"
        "|      |      |      |      |"
        "| xxxx | xxxx | xxxx | xxxx |"
        "| xxxx | xxxx | xxxx | xxxx |"
        "| xxxx | xxxx | xxxx | xxxx |"
        "|      |      |      |      |"
);

#[cfg(feature = "color")]
test_table!(
    cell_height_limit_colored,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Modify::new(Columns::first()).with(Format::content(|s| format!("xxxx\n{s}xxxx\nxxxx\n").red().to_string())))
        .with(
            Modify::new(Columns::first())
                .with(Height::limit(1))
        )
        .with(Modify::new(Segment::all()).with(
            Alignment::center_vertical()
        )),
        "| \u{1b}[31mxxxx\u{1b}[39m | column 0 | column 1 | column 2 |"
        "|------|----------|----------|----------|"
        "| \u{1b}[31mxxxx\u{1b}[39m |   0-0    |   0-1    |   0-2    |"
        "| \u{1b}[31mxxxx\u{1b}[39m |   1-0    |   1-1    |   1-2    |"
        "| \u{1b}[31mxxxx\u{1b}[39m |   2-0    |   2-1    |   2-2    |"
);

#[cfg(feature = "color")]
test_table!(
    table_height_limit_colored,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Modify::new(Columns::first()).with(Format::content(|s| format!("xxxx\n{s}xxxx\nxxxx\n").blue().on_green().to_string())))
        .with(Modify::new(Columns::first()).with(Alignment::center_vertical()))
        .with(Height::limit(10)),
        "| \u{1b}[34;42mxxxx\u{1b}[39m\u{1b}[49m  | column 0 | column 1 | column 2 |"
        "| \u{1b}[34m\u{1b}[42mNxxxx\u{1b}[39m\u{1b}[49m |          |          |          |"
        "|-------|----------|----------|----------|"
        "| \u{1b}[34;42mxxxx\u{1b}[39m\u{1b}[49m  |   0-0    |   0-1    |   0-2    |"
        "| \u{1b}[34m\u{1b}[42m0xxxx\u{1b}[39m\u{1b}[49m |          |          |          |"
        "| \u{1b}[34;42mxxxx\u{1b}[39m\u{1b}[49m  |   1-0    |   1-1    |   1-2    |"
        "| \u{1b}[34m\u{1b}[42m1xxxx\u{1b}[39m\u{1b}[49m |          |          |          |"
        "| \u{1b}[34;42mxxxx\u{1b}[39m\u{1b}[49m  |   2-0    |   2-1    |   2-2    |"
        "| \u{1b}[34m\u{1b}[42m2xxxx\u{1b}[39m\u{1b}[49m |          |          |          |"
        "| \u{1b}[34m\u{1b}[42mxxxx\u{1b}[39m\u{1b}[49m  |          |          |          |"
);

#[cfg(feature = "macros")]
test_table!(
    cell_height_1x1,
    tabled::row![tabled::col!["SGML"].with(Height::increase(4))],
    "+----------+"
    "| +------+ |"
    "| | SGML | |"
    "| |      | |"
    "| +------+ |"
    "+----------+"
);

#[cfg(feature = "macros")]
test_table!(
    cell_height_1x1_no_top_border,
    tabled::row![tabled::col!["SGML"].with(Style::ascii().remove_top()).with(Height::increase(4))],
    "+----------+"
    "| | SGML | |"
    "| |      | |"
    "| |      | |"
    "| +------+ |"
    "+----------+"
);
