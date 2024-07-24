#![cfg(feature = "std")]

use tabled::{
    settings::{themes::ColumnNames, Alignment, Color, Padding},
    Table,
};

use crate::matrix::Matrix;
use testing_table::test_table;

test_table!(
    new,
    Matrix::new(3, 3).with(ColumnNames::new(["1", "2", "3", "4"])),
    "+1--+2---------+3---------+4---------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| 0 |   0-0    |   0-1    |   0-2    |"
    "+---+----------+----------+----------+"
    "| 1 |   1-0    |   1-1    |   1-2    |"
    "+---+----------+----------+----------+"
    "| 2 |   2-0    |   2-1    |   2-2    |"
    "+---+----------+----------+----------+"
);

test_table!(
    new_more_names_then_columns,
    Matrix::new(3, 3).with(ColumnNames::new(["1", "2", "3", "4", "5", "6", "7"])),
    "+1--+2---------+3---------+4---------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| 0 |   0-0    |   0-1    |   0-2    |"
    "+---+----------+----------+----------+"
    "| 1 |   1-0    |   1-1    |   1-2    |"
    "+---+----------+----------+----------+"
    "| 2 |   2-0    |   2-1    |   2-2    |"
    "+---+----------+----------+----------+"
);

test_table!(
    new_less_names_then_columns,
    Matrix::new(3, 3).with(ColumnNames::new(["1", "2"])),
    "+1--+2---------+----------+----------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| 0 |   0-0    |   0-1    |   0-2    |"
    "+---+----------+----------+----------+"
    "| 1 |   1-0    |   1-1    |   1-2    |"
    "+---+----------+----------+----------+"
    "| 2 |   2-0    |   2-1    |   2-2    |"
    "+---+----------+----------+----------+"
);

test_table!(
    new_empty,
    Matrix::new(3, 3).with(ColumnNames::new([""; 0])),
    "+---+----------+----------+----------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| 0 |   0-0    |   0-1    |   0-2    |"
    "+---+----------+----------+----------+"
    "| 1 |   1-0    |   1-1    |   1-2    |"
    "+---+----------+----------+----------+"
    "| 2 |   2-0    |   2-1    |   2-2    |"
    "+---+----------+----------+----------+"
);

test_table!(
    default,
    Matrix::new(3, 3).with(ColumnNames::default()),
    "+N--+column 0+column 1+column 2+"
    "| 0 |  0-0   |  0-1   |  0-2   |"
    "+---+--------+--------+--------+"
    "| 1 |  1-0   |  1-1   |  1-2   |"
    "+---+--------+--------+--------+"
    "| 2 |  2-0   |  2-1   |  2-2   |"
    "+---+--------+--------+--------+"
);

test_table!(
    alignment_left,
    Table::new([("Hello", "World"), ("and", "looooong\nword")])
        .with(ColumnNames::default().alignment(Alignment::left())),
    "+&str---+&str------+"
    "| Hello | World    |"
    "+-------+----------+"
    "| and   | looooong |"
    "|       | word     |"
    "+-------+----------+"
);

test_table!(
    alignment_right,
    Table::new([("Hello", "World"), ("and", "looooong\nword")])
        .with(ColumnNames::default().alignment(Alignment::right())),
    "+---&str+------&str+"
    "| Hello | World    |"
    "+-------+----------+"
    "| and   | looooong |"
    "|       | word     |"
    "+-------+----------+"
);

test_table!(
    alignment_center,
    Table::new([("Hello", "World"), ("and", "looooong\nword")])
        .with(ColumnNames::default().alignment(Alignment::center())),
    "+-&str--+---&str---+"
    "| Hello | World    |"
    "+-------+----------+"
    "| and   | looooong |"
    "|       | word     |"
    "+-------+----------+"
);

test_table!(
    alignment_center_long,
    Table::new([("Hello", "World"), ("and", "looooong\nword")])
        .with(ColumnNames::new(["&&&&&&&str", "&&&&&&&str"]).alignment(Alignment::center())),
    "+&&&&&&&str+&&&&&&&str+"
    "| &str     | &str     |"
    "+----------+----------+"
    "| Hello    | World    |"
    "+----------+----------+"
    "| and      | looooong |"
    "|          | word     |"
    "+----------+----------+"
);

test_table!(
    alignment_array,
    Table::new([("Hello", "World"), ("and", "looooong\nword")])
        .with(ColumnNames::default().alignment(vec![Alignment::right(), Alignment::center()])),
    "+---&str+---&str---+"
    "| Hello | World    |"
    "+-------+----------+"
    "| and   | looooong |"
    "|       | word     |"
    "+-------+----------+"
);

test_table!(
    line,
    Matrix::new(3, 3).with(ColumnNames::default().line(1)),
    "+---+--------+--------+--------+"
    "| 0 |  0-0   |  0-1   |  0-2   |"
    "+N--+column 0+column 1+column 2+"
    "| 1 |  1-0   |  1-1   |  1-2   |"
    "+---+--------+--------+--------+"
    "| 2 |  2-0   |  2-1   |  2-2   |"
    "+---+--------+--------+--------+"
);

test_table!(
    line_max_out,
    Matrix::new(3, 3).with(ColumnNames::default().line(100)),
    "+---+----------+----------+----------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| 0 |   0-0    |   0-1    |   0-2    |"
    "+---+----------+----------+----------+"
    "| 1 |   1-0    |   1-1    |   1-2    |"
    "+---+----------+----------+----------+"
    "| 2 |   2-0    |   2-1    |   2-2    |"
    "+---+----------+----------+----------+"
);

test_table!(
    line_0,
    Matrix::new(3, 3).with(ColumnNames::default().line(0)),
    "+N--+column 0+column 1+column 2+"
    "| 0 |  0-0   |  0-1   |  0-2   |"
    "+---+--------+--------+--------+"
    "| 1 |  1-0   |  1-1   |  1-2   |"
    "+---+--------+--------+--------+"
    "| 2 |  2-0   |  2-1   |  2-2   |"
    "+---+--------+--------+--------+"
);

test_table!(
    colors_some_some,
    Table::new([("Hello", "World"), ("and", "looooong\nword")])
        .with(ColumnNames::default().color(vec![Color::BG_BLACK, Color::BG_BLUE])),
    "+\u{1b}[40m&\u{1b}[49m\u{1b}[40ms\u{1b}[49m\u{1b}[40mt\u{1b}[49m\u{1b}[40mr\u{1b}[49m---+\u{1b}[44m&\u{1b}[49m\u{1b}[44ms\u{1b}[49m\u{1b}[44mt\u{1b}[49m\u{1b}[44mr\u{1b}[49m------+"
    "| Hello | World    |"
    "+-------+----------+"
    "| and   | looooong |"
    "|       | word     |"
    "+-------+----------+"
);

test_table!(
    colors_none_some,
    Table::new([("Hello", "World"), ("and", "looooong\nword")])
        .with(ColumnNames::default().color(vec![Color::default(), Color::BG_BLUE])),
        "+&str---+\u{1b}[44m&\u{1b}[49m\u{1b}[44ms\u{1b}[49m\u{1b}[44mt\u{1b}[49m\u{1b}[44mr\u{1b}[49m------+"
        "| Hello | World    |"
        "+-------+----------+"
        "| and   | looooong |"
        "|       | word     |"
        "+-------+----------+"
);

test_table!(
    colors_none_none,
    Table::new([("Hello", "World"), ("and", "looooong\nword")])
        .with(ColumnNames::default().color(vec![Color::default(), Color::default()])),
    "+&str---+&str------+"
    "| Hello | World    |"
    "+-------+----------+"
    "| and   | looooong |"
    "|       | word     |"
    "+-------+----------+"
);

test_table!(
    colors_empty,
    Table::new([("Hello", "World"), ("and", "looooong\nword")])
        .with(ColumnNames::default().color({ Color::default(); vec![] as std::vec::Vec<tabled::settings::Color> })),
    "+&str---+&str------+"
    "| Hello | World    |"
    "+-------+----------+"
    "| and   | looooong |"
    "|       | word     |"
    "+-------+----------+"
);

test_table!(
    new_vertical,
    Matrix::new(3, 3).with(ColumnNames::new(["1", "2", "3", "4"]).alignment(Alignment::top())),
    "+---+----------+----------+----------+"
    "1 N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "2 0 |   0-0    |   0-1    |   0-2    |"
    "+---+----------+----------+----------+"
    "3 1 |   1-0    |   1-1    |   1-2    |"
    "+---+----------+----------+----------+"
    "4 2 |   2-0    |   2-1    |   2-2    |"
    "+---+----------+----------+----------+"
);

test_table!(
    new_vertical_1,
    Matrix::new(2, 2).with(Padding::new(1, 1, 2, 2)).with(ColumnNames::new(["1", "2", "3", "4"]).alignment(Alignment::top())),
    "+---+----------+----------+"
    "1   |          |          |"
    "|   |          |          |"
    "| N | column 0 | column 1 |"
    "|   |          |          |"
    "|   |          |          |"
    "+---+----------+----------+"
    "2   |          |          |"
    "|   |          |          |"
    "| 0 |   0-0    |   0-1    |"
    "|   |          |          |"
    "|   |          |          |"
    "+---+----------+----------+"
    "3   |          |          |"
    "|   |          |          |"
    "| 1 |   1-0    |   1-1    |"
    "|   |          |          |"
    "|   |          |          |"
    "+---+----------+----------+"
);

test_table!(
    new_vertical_2,
    Matrix::new(2, 2).with(Padding::new(1, 1, 2, 2)).with(ColumnNames::new(["1", "2", "3", "4"]).alignment(Alignment::bottom())),
    "+---+----------+----------+"
    "|   |          |          |"
    "|   |          |          |"
    "| N | column 0 | column 1 |"
    "|   |          |          |"
    "1   |          |          |"
    "+---+----------+----------+"
    "|   |          |          |"
    "|   |          |          |"
    "| 0 |   0-0    |   0-1    |"
    "|   |          |          |"
    "2   |          |          |"
    "+---+----------+----------+"
    "|   |          |          |"
    "|   |          |          |"
    "| 1 |   1-0    |   1-1    |"
    "|   |          |          |"
    "3   |          |          |"
    "+---+----------+----------+"
);

test_table!(
    new_vertical_3,
    Matrix::new(2, 2).with(Padding::new(1, 1, 2, 2)).with(ColumnNames::new(["1", "2", "3", "4"]).alignment(Alignment::center_vertical())),
    "+---+----------+----------+"
    "|   |          |          |"
    "|   |          |          |"
    "1 N | column 0 | column 1 |"
    "|   |          |          |"
    "|   |          |          |"
    "+---+----------+----------+"
    "|   |          |          |"
    "|   |          |          |"
    "2 0 |   0-0    |   0-1    |"
    "|   |          |          |"
    "|   |          |          |"
    "+---+----------+----------+"
    "|   |          |          |"
    "|   |          |          |"
    "3 1 |   1-0    |   1-1    |"
    "|   |          |          |"
    "|   |          |          |"
    "+---+----------+----------+"
);

test_table!(
    new_vertical_default_0,
    Matrix::new(2, 2).with(Padding::new(1, 1, 2, 2)).with(ColumnNames::default().alignment(Alignment::top())),
    "+---+-----+-----+"
    "N   |     |     |"
    "|   |     |     |"
    "| 0 | 0-0 | 0-1 |"
    "|   |     |     |"
    "|   |     |     |"
    "+---+-----+-----+"
    "c   |     |     |"
    "o   |     |     |"
    "l 1 | 1-0 | 1-1 |"
    "u   |     |     |"
    "m   |     |     |"
    "n   |     |     |"
    "    |     |     |"
    "0   |     |     |"
    "+---+-----+-----+"
);

test_table!(
    new_vertical_default_2,
    Matrix::new(2, 2).with(Padding::new(1, 1, 2, 2)).with(ColumnNames::default().alignment(Alignment::bottom())),
    "+---+-----+-----+"
    "|   |     |     |"
    "|   |     |     |"
    "| 0 | 0-0 | 0-1 |"
    "|   |     |     |"
    "N   |     |     |"
    "+---+-----+-----+"
    "c   |     |     |"
    "o   |     |     |"
    "l 1 | 1-0 | 1-1 |"
    "u   |     |     |"
    "m   |     |     |"
    "n   |     |     |"
    "    |     |     |"
    "0   |     |     |"
    "+---+-----+-----+"
);

test_table!(
    new_vertical_default_1,
    Matrix::new(2, 2).with(Padding::new(1, 1, 5, 5)).with(ColumnNames::default().alignment(Alignment::center_vertical())),
    "+---+-----+-----+"
    "|   |     |     |"
    "|   |     |     |"
    "|   |     |     |"
    "|   |     |     |"
    "|   |     |     |"
    "N 0 | 0-0 | 0-1 |"
    "|   |     |     |"
    "|   |     |     |"
    "|   |     |     |"
    "|   |     |     |"
    "|   |     |     |"
    "+---+-----+-----+"
    "|   |     |     |"
    "c   |     |     |"
    "o   |     |     |"
    "l   |     |     |"
    "u   |     |     |"
    "m 1 | 1-0 | 1-1 |"
    "n   |     |     |"
    "    |     |     |"
    "0   |     |     |"
    "|   |     |     |"
    "|   |     |     |"
    "+---+-----+-----+"
);
