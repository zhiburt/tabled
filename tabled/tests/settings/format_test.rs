#![cfg(feature = "std")]
#![cfg(feature = "assert")]

use tabled::{
    assert::test_table,
    settings::{
        object::{Cell, Columns, Object, Rows, Segment},
        Alignment, Format, Modify, Padding, Style,
    },
};

use crate::matrix::Matrix;

#[cfg(feature = "ansi")]
use tabled::settings::Color;

test_table!(
    formatting_full_test,
    Matrix::new(3, 3).with(Modify::new(Segment::all()).with(Format::content(|s| format!("[{s}]")))),
    "+-----+------------+------------+------------+"
    "| [N] | [column 0] | [column 1] | [column 2] |"
    "+-----+------------+------------+------------+"
    "| [0] |   [0-0]    |   [0-1]    |   [0-2]    |"
    "+-----+------------+------------+------------+"
    "| [1] |   [1-0]    |   [1-1]    |   [1-2]    |"
    "+-----+------------+------------+------------+"
    "| [2] |   [2-0]    |   [2-1]    |   [2-2]    |"
    "+-----+------------+------------+------------+"
);

test_table!(
    formatting_head_test,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Modify::new(Rows::first()).with(Format::content(|s| format!(":{s}")))),
    "| :N | :column 0 | :column 1 | :column 2 |"
    "|----|-----------|-----------|-----------|"
    "| 0  |    0-0    |    0-1    |    0-2    |"
    "| 1  |    1-0    |    1-1    |    1-2    |"
    "| 2  |    2-0    |    2-1    |    2-2    |"
);

test_table!(
    formatting_row_test,
    Matrix::new(3, 3)
        .with(Style::psql())
        .with(Modify::new(Rows::new(1..)).with(Format::content(|s| format!("<{s}>")))),
    "  N  | column 0 | column 1 | column 2 "
    "-----+----------+----------+----------"
    " <0> |  <0-0>   |  <0-1>   |  <0-2>   "
    " <1> |  <1-0>   |  <1-1>   |  <1-2>   "
    " <2> |  <2-0>   |  <2-1>   |  <2-2>   "
);

test_table!(
    formatting_column_test,
    Matrix::new(3, 3)
        .with(Style::psql())
        .with(Modify::new(Columns::single(0)).with(Format::content(|s| format!("(x) {s}")))),
    " (x) N | column 0 | column 1 | column 2 "
    "-------+----------+----------+----------"
    " (x) 0 |   0-0    |   0-1    |   0-2    "
    " (x) 1 |   1-0    |   1-1    |   1-2    "
    " (x) 2 |   2-0    |   2-1    |   2-2    "
);

test_table!(
    formatting_multiline_test,
    Matrix::new(3, 3)
        .insert((2, 2).into(), "E\nnde\navou\nros")
        .insert((3, 2).into(), "Red\nHat")
        .insert((3, 3).into(), "https://\nwww\n.\nredhat\n.com\n/en")
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Format::content(|s| format!("(x) {s}")).multiline())),
    " (x) N | (x) column 0 | (x) column 1 | (x) column 2 "
    "-------+--------------+--------------+--------------"
    " (x) 0 |   (x) 0-0    |   (x) 0-1    |   (x) 0-2    "
    " (x) 1 |   (x) 1-0    |   (x) E      |   (x) 1-2    "
    "       |              |   (x) nde    |              "
    "       |              |   (x) avou   |              "
    "       |              |   (x) ros    |              "
    " (x) 2 |   (x) 2-0    |   (x) Red    | (x) https:// "
    "       |              |   (x) Hat    | (x) www      "
    "       |              |              | (x) .        "
    "       |              |              | (x) redhat   "
    "       |              |              | (x) .com     "
    "       |              |              | (x) /en      "
);

test_table!(
    formatting_cell_test,
    Matrix::new(3, 3)
        .with(Style::psql())
        .with(Modify::new(Cell::new(0, 0)).with(Format::content(|s| format!("(x) {s}"))))
        .with(Modify::new(Cell::new(0, 1)).with(Format::content(|s| format!("(x) {s}"))))
        .with(Modify::new(Cell::new(0, 2)).with(Format::content(|s| format!("(x) {s}")))),
    " (x) N | (x) column 0 | (x) column 1 | column 2 "
    "-------+--------------+--------------+----------"
    "   0   |     0-0      |     0-1      |   0-2    "
    "   1   |     1-0      |     1-1      |   1-2    "
    "   2   |     2-0      |     2-1      |   2-2    "
);

test_table!(
    formatting_combination_and_test,
    Matrix::new(3, 3)
        .with(Style::psql())
        .with(
            Modify::new(Columns::single(0).and(Rows::one(0)))
                .with(Format::content(|s| format!("(x) {s}"))),
        ),
    " (x) N | (x) column 0 | (x) column 1 | (x) column 2 "
    "-------+--------------+--------------+--------------"
    " (x) 0 |     0-0      |     0-1      |     0-2      "
    " (x) 1 |     1-0      |     1-1      |     1-2      "
    " (x) 2 |     2-0      |     2-1      |     2-2      "
);

test_table!(
    formatting_combination_not_test,
    Matrix::new(3, 3)
        .with(Style::psql())
        .with(
            Modify::new(Columns::single(0).and(Rows::one(0)).not(Cell::new(0, 0)))
                .with(Format::content(|s| format!("(x) {s}"))),
        ),
    "   N   | (x) column 0 | (x) column 1 | (x) column 2 "
    "-------+--------------+--------------+--------------"
    " (x) 0 |     0-0      |     0-1      |     0-2      "
    " (x) 1 |     1-0      |     1-1      |     1-2      "
    " (x) 2 |     2-0      |     2-1      |     2-2      "
);

test_table!(
    formatting_combination_inverse_test,
    Matrix::new(3, 3)
        .with(Style::psql())
        .with(Modify::new(Columns::single(0).inverse()).with(Format::content(|s| format!("(x) {s}")))),
    " N | (x) column 0 | (x) column 1 | (x) column 2 "
    "---+--------------+--------------+--------------"
    " 0 |   (x) 0-0    |   (x) 0-1    |   (x) 0-2    "
    " 1 |   (x) 1-0    |   (x) 1-1    |   (x) 1-2    "
    " 2 |   (x) 2-0    |   (x) 2-1    |   (x) 2-2    "
);

test_table!(
    formatting_combination_intersect_test,
    Matrix::new(3, 3)
        .with(Style::psql())
        .with(
            Modify::new(Columns::new(1..3).intersect(Rows::new(1..3)))
                .with(Format::content(|s| format!("(x) {s}"))),
        ),
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    " 0 | (x) 0-0  | (x) 0-1  |   0-2    "
    " 1 | (x) 1-0  | (x) 1-1  |   1-2    "
    " 2 |   2-0    |   2-1    |   2-2    "
);

test_table!(
    formatting_using_lambda_test,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Modify::new(Rows::first()).with(Format::content(|s| format!(":{s}")))),
    "| :N | :column 0 | :column 1 | :column 2 |"
    "|----|-----------|-----------|-----------|"
    "| 0  |    0-0    |    0-1    |    0-2    |"
    "| 1  |    1-0    |    1-1    |    1-2    |"
    "| 2  |    2-0    |    2-1    |    2-2    |"
);

test_table!(
    formatting_using_function_test,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Modify::new(Rows::first()).with(Format::content(str::to_uppercase))),
    "| N | COLUMN 0 | COLUMN 1 | COLUMN 2 |"
    "|---|----------|----------|----------|"
    "| 0 |   0-0    |   0-1    |   0-2    |"
    "| 1 |   1-0    |   1-1    |   1-2    |"
    "| 2 |   2-0    |   2-1    |   2-2    |"
);

test_table!(
    format_with_index,
    Matrix::new(3, 3)
        .with(Style::markdown())
        .with(Modify::new(Rows::first()).with(Format::positioned(|a, p| {
            let p: (_, _) = p.into();
            match p {
                (0, 0) => "(0, 0)".to_string(),
                (0, 1) => "(0, 1)".to_string(),
                (0, 2) => "(0, 2)".to_string(),
                _ => a.to_string(),
            }
        }))),
    "| (0, 0) | (0, 1) | (0, 2) | column 2 |"
    "|--------|--------|--------|----------|"
    "|   0    |  0-0   |  0-1   |   0-2    |"
    "|   1    |  1-0   |  1-1   |   1-2    |"
    "|   2    |  2-0   |  2-1   |   2-2    |"
);

test_table!(
    format_doesnt_change_padding,
    Matrix::new(3, 3)
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Padding::new(3, 1, 0, 0)))
        .with(Modify::new(Segment::all()).with(Format::content(|s| format!("[{s}]")))),
    "+-------+--------------+--------------+--------------+"
    "|   [N] |   [column 0] |   [column 1] |   [column 2] |"
    "+-------+--------------+--------------+--------------+"
    "|   [0] |   [0-0]      |   [0-1]      |   [0-2]      |"
    "+-------+--------------+--------------+--------------+"
    "|   [1] |   [1-0]      |   [1-1]      |   [1-2]      |"
    "+-------+--------------+--------------+--------------+"
    "|   [2] |   [2-0]      |   [2-1]      |   [2-2]      |"
    "+-------+--------------+--------------+--------------+"
);

test_table!(
    formatting_content_str_test,
    Matrix::new(3, 3).with(Modify::new(Segment::all()).with(Format::content(|_| String::from("Hello World")))),
    "+-------------+-------------+-------------+-------------+"
    "| Hello World | Hello World | Hello World | Hello World |"
    "+-------------+-------------+-------------+-------------+"
    "| Hello World | Hello World | Hello World | Hello World |"
    "+-------------+-------------+-------------+-------------+"
    "| Hello World | Hello World | Hello World | Hello World |"
    "+-------------+-------------+-------------+-------------+"
    "| Hello World | Hello World | Hello World | Hello World |"
    "+-------------+-------------+-------------+-------------+"
);

#[cfg(feature = "ansi")]
test_table!(
    color_test,
    Matrix::new(3, 3)
        .with(Style::psql())
        .modify(Columns::new(..1).and(Columns::new(2..)), Format::content(|s| Color::FG_RED.colorize(s)))
        .modify(Columns::new(1..2), Format::content(|s| Color::FG_BLUE.colorize(s))),
    " \u{1b}[31mN\u{1b}[39m | \u{1b}[34mcolumn 0\u{1b}[39m | \u{1b}[31mcolumn 1\u{1b}[39m | \u{1b}[31mcolumn 2\u{1b}[39m "
    "---+----------+----------+----------"
    " \u{1b}[31m0\u{1b}[39m |   \u{1b}[34m0-0\u{1b}[39m    |   \u{1b}[31m0-1\u{1b}[39m    |   \u{1b}[31m0-2\u{1b}[39m    "
    " \u{1b}[31m1\u{1b}[39m |   \u{1b}[34m1-0\u{1b}[39m    |   \u{1b}[31m1-1\u{1b}[39m    |   \u{1b}[31m1-2\u{1b}[39m    "
    " \u{1b}[31m2\u{1b}[39m |   \u{1b}[34m2-0\u{1b}[39m    |   \u{1b}[31m2-1\u{1b}[39m    |   \u{1b}[31m2-2\u{1b}[39m    "
);

#[cfg(feature = "ansi")]
test_table!(
    color_multiline_test,
    Matrix::new(3, 3)
        .insert((2, 2).into(), "E\nnde\navou\nros")
        .insert((3, 2).into(), "Red\nHat")
        .insert((3, 3).into(), "https://\nwww\n.\nredhat\n.com\n/en")
        .with(Style::psql())
        .modify(Columns::new(..1), Format::content(|s| Color::FG_RED.colorize(s)).multiline())
        .modify(Columns::new(1..2), Format::content(|s| Color::FG_BLUE.colorize(s)).multiline())
        .modify(Columns::new(2..), Format::content(|s| Color::FG_GREEN.colorize(s)).multiline()),
    " \u{1b}[31mN\u{1b}[39m | \u{1b}[34mcolumn 0\u{1b}[39m | \u{1b}[32mcolumn 1\u{1b}[39m | \u{1b}[32mcolumn 2\u{1b}[39m "
    "---+----------+----------+----------\n \u{1b}[31m0\u{1b}[39m |   \u{1b}[34m0-0\u{1b}[39m    |   \u{1b}[32m0-1\u{1b}[39m    |   \u{1b}[32m0-2\u{1b}[39m    "
    " \u{1b}[31m1\u{1b}[39m |   \u{1b}[34m1-0\u{1b}[39m    |   \u{1b}[32mE\u{1b}[39m      |   \u{1b}[32m1-2\u{1b}[39m    "
    "   |          |   \u{1b}[32mnde\u{1b}[39m    |          "
    "   |          |   \u{1b}[32mavou\u{1b}[39m   |          "
    "   |          |   \u{1b}[32mros\u{1b}[39m    |          "
    " \u{1b}[31m2\u{1b}[39m |   \u{1b}[34m2-0\u{1b}[39m    |   \u{1b}[32mRed\u{1b}[39m    | \u{1b}[32mhttps://\u{1b}[39m "
    "   |          |   \u{1b}[32mHat\u{1b}[39m    | \u{1b}[32mwww\u{1b}[39m      \n   |          |          | \u{1b}[32m.\u{1b}[39m        "
    "   |          |          | \u{1b}[32mredhat\u{1b}[39m   "
    "   |          |          | \u{1b}[32m.com\u{1b}[39m     "
    "   |          |          | \u{1b}[32m/en\u{1b}[39m      "
);
